//! OKX REST API HTTP 客户端。
//!
//! 参考：官方 Python SDK 的组织方式
//! - <https://github.com/okxapi/python-okx>

use std::{future::Future, pin::Pin, time::Duration};

use reqwest::{Client, Method, Proxy, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use tracing::{debug, instrument};

use okx_core::{types::ApiResponse, Config, OkxError, Result, Signer};

type TransportFuture = Pin<Box<dyn Future<Output = Result<(StatusCode, String)>> + Send>>;

/// HTTP 传输层抽象（便于在无网络/受限环境中做纯内存测试）。
trait HttpTransport: Clone + Send + Sync + 'static {
    fn send(
        &self,
        method: Method,
        url: String,
        headers: Vec<(&'static str, String)>,
        body: Option<String>,
    ) -> TransportFuture;
}

#[derive(Debug, Clone)]
struct ReqwestTransport {
    client: Client,
}

impl HttpTransport for ReqwestTransport {
    fn send(
        &self,
        method: Method,
        url: String,
        headers: Vec<(&'static str, String)>,
        body: Option<String>,
    ) -> TransportFuture {
        let client = self.client.clone();
        Box::pin(async move {
            let mut request = client.request(method, &url);

            for (name, value) in headers {
                request = request.header(name, value);
            }

            if let Some(body) = body {
                request = request.body(body);
            }

            let response = request
                .send()
                .await
                .map_err(|e| OkxError::Http(e.to_string()))?;

            let status = response.status();
            let text = response
                .text()
                .await
                .map_err(|e| OkxError::Http(format!("Failed to read response: {e}")))?;

            Ok((status, text))
        })
    }
}

#[derive(Debug, Clone)]
struct OkxRestClientInner<T: HttpTransport> {
    transport: T,
    config: Config,
    signer: Signer,
}

impl<T: HttpTransport> OkxRestClientInner<T> {
    fn new(config: Config, transport: T) -> Self {
        let signer = Signer::new(config.credentials().clone());
        Self {
            transport,
            config,
            signer,
        }
    }

    fn config(&self) -> &Config {
        &self.config
    }

    async fn send_and_parse<TOut: DeserializeOwned>(
        &self,
        method: Method,
        url: String,
        headers: Vec<(&'static str, String)>,
        body: Option<String>,
    ) -> Result<Vec<TOut>> {
        let (status, text) = self.transport.send(method, url, headers, body).await?;
        debug!("Response status={} body={}", status, text);
        self.parse_body(status, &text)
    }

    async fn send_and_parse_raw(
        &self,
        method: Method,
        url: String,
        headers: Vec<(&'static str, String)>,
        body: Option<String>,
    ) -> Result<Value> {
        let (status, text) = self.transport.send(method, url, headers, body).await?;
        debug!("Raw response status={} body={}", status, text);
        self.parse_body_raw(status, &text)
    }

    fn build_url<P: Serialize + ?Sized>(&self, path: &str, params: Option<&P>) -> Result<String> {
        let base_url = format!("{}{}", self.config.rest_url(), path);

        match params {
            Some(p) => {
                let query = serde_urlencoded::to_string(p)
                    .map_err(|e| OkxError::Other(format!("Failed to encode params: {e}")))?;
                if query.is_empty() {
                    Ok(base_url)
                } else {
                    Ok(format!("{base_url}?{query}"))
                }
            }
            None => Ok(base_url),
        }
    }

    fn extract_request_path(&self, url: &str) -> String {
        url.strip_prefix(self.config.rest_url())
            .unwrap_or(url)
            .to_string()
    }

    fn parse_body<TOut: DeserializeOwned>(
        &self,
        status: StatusCode,
        text: &str,
    ) -> Result<Vec<TOut>> {
        if !status.is_success() {
            return Err(OkxError::Http(format!(
                "HTTP error: status={}, body={}",
                status, text
            )));
        }

        let api_response: ApiResponse<TOut> = serde_json::from_str(text)?;
        if api_response.is_success() {
            Ok(api_response.data)
        } else {
            Err(OkxError::api(api_response.code, api_response.msg))
        }
    }

    fn parse_body_raw(&self, status: StatusCode, text: &str) -> Result<Value> {
        if !status.is_success() {
            return Err(OkxError::Http(format!(
                "HTTP error: status={}, body={}",
                status, text
            )));
        }

        Ok(serde_json::from_str(text)?)
    }

    async fn get_public<TOut, P>(&self, path: &str, params: Option<&P>) -> Result<Vec<TOut>>
    where
        TOut: DeserializeOwned,
        P: Serialize + ?Sized,
    {
        let url = self.build_url(path, params)?;
        debug!("GET (public) {}", url);

        let headers = Signer::generate_public_headers(self.config.is_simulated());
        self.send_and_parse(Method::GET, url, headers, None).await
    }

    async fn get_public_raw<P>(&self, path: &str, params: Option<&P>) -> Result<Value>
    where
        P: Serialize + ?Sized,
    {
        let url = self.build_url(path, params)?;
        debug!("GET (public raw) {}", url);

        let headers = Signer::generate_public_headers(self.config.is_simulated());
        self.send_and_parse_raw(Method::GET, url, headers, None)
            .await
    }

    async fn get<TOut, P>(&self, path: &str, params: Option<&P>) -> Result<Vec<TOut>>
    where
        TOut: DeserializeOwned,
        P: Serialize + ?Sized,
    {
        let url = self.build_url(path, params)?;
        let request_path = self.extract_request_path(&url);
        debug!("GET (private) {}", url);

        let headers =
            self.signer
                .generate_headers("GET", &request_path, "", self.config.is_simulated());
        self.send_and_parse(Method::GET, url, headers, None).await
    }

    async fn get_raw<P>(&self, path: &str, params: Option<&P>) -> Result<Value>
    where
        P: Serialize + ?Sized,
    {
        let url = self.build_url(path, params)?;
        let request_path = self.extract_request_path(&url);
        debug!("GET (private raw) {}", url);

        let headers =
            self.signer
                .generate_headers("GET", &request_path, "", self.config.is_simulated());
        self.send_and_parse_raw(Method::GET, url, headers, None)
            .await
    }

    async fn post<TOut, B>(&self, path: &str, body: &B) -> Result<Vec<TOut>>
    where
        TOut: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let url = format!("{}{}", self.config.rest_url(), path);
        let body_str = serde_json::to_string(body)?;
        debug!("POST {} body={}", url, body_str);

        let headers =
            self.signer
                .generate_headers("POST", path, &body_str, self.config.is_simulated());
        self.send_and_parse(Method::POST, url, headers, Some(body_str))
            .await
    }

    async fn post_raw<B>(&self, path: &str, body: &B) -> Result<Value>
    where
        B: Serialize + ?Sized,
    {
        let url = format!("{}{}", self.config.rest_url(), path);
        let body_str = serde_json::to_string(body)?;
        debug!("POST (private raw) {} body={}", url, body_str);

        let headers =
            self.signer
                .generate_headers("POST", path, &body_str, self.config.is_simulated());
        self.send_and_parse_raw(Method::POST, url, headers, Some(body_str))
            .await
    }
}

/// OKX 交易所 REST API 客户端。
///
/// 负责：
/// - 构造 URL（含 query 编码）
/// - 生成签名请求头（私有接口）
/// - 发送请求并解析 OKX 标准响应
#[derive(Debug, Clone)]
pub struct OkxRestClient {
    inner: OkxRestClientInner<ReqwestTransport>,
}

impl OkxRestClient {
    /// 创建新的 REST 客户端。
    ///
    /// - 超时与代理来自 `Config`
    /// - HTTP 客户端构建失败将 panic（与现有行为保持一致）
    #[must_use]
    pub fn new(config: Config) -> Self {
        let mut builder = Client::builder().timeout(Duration::from_secs(config.timeout_secs()));

        if let Some(proxy_url) = config.proxy_url() {
            let proxy = Proxy::all(proxy_url).expect("Invalid proxy URL");
            builder = builder.proxy(proxy);
        }

        let http = builder.build().expect("Failed to build HTTP client");
        Self::with_http_client(config, http)
    }

    /// 使用自定义 `reqwest::Client` 创建 REST 客户端。
    #[must_use]
    pub fn with_http_client(config: Config, http: Client) -> Self {
        let transport = ReqwestTransport { client: http };
        Self {
            inner: OkxRestClientInner::new(config, transport),
        }
    }

    /// 读取配置。
    #[must_use]
    pub fn config(&self) -> &Config {
        self.inner.config()
    }

    /// 公有 GET（不签名）。
    #[instrument(skip(self, params), fields(path = %path))]
    pub async fn get_public<TOut, P>(&self, path: &str, params: Option<&P>) -> Result<Vec<TOut>>
    where
        TOut: DeserializeOwned,
        P: Serialize + ?Sized,
    {
        self.inner.get_public(path, params).await
    }

    /// 公有 GET（raw），返回完整 JSON（不解释 OKX `code`）。
    #[instrument(skip(self, params), fields(path = %path))]
    pub async fn get_public_raw<P>(&self, path: &str, params: Option<&P>) -> Result<Value>
    where
        P: Serialize + ?Sized,
    {
        self.inner.get_public_raw(path, params).await
    }

    /// 私有 GET（签名）。
    #[instrument(skip(self, params), fields(path = %path))]
    pub async fn get<TOut, P>(&self, path: &str, params: Option<&P>) -> Result<Vec<TOut>>
    where
        TOut: DeserializeOwned,
        P: Serialize + ?Sized,
    {
        self.inner.get(path, params).await
    }

    /// 私有 GET（raw，签名）。
    #[instrument(skip(self, params), fields(path = %path))]
    pub async fn get_raw<P>(&self, path: &str, params: Option<&P>) -> Result<Value>
    where
        P: Serialize + ?Sized,
    {
        self.inner.get_raw(path, params).await
    }

    /// 私有 POST（签名）。
    #[instrument(skip(self, body), fields(path = %path))]
    pub async fn post<TOut, B>(&self, path: &str, body: &B) -> Result<Vec<TOut>>
    where
        TOut: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        self.inner.post(path, body).await
    }

    /// 私有 POST（raw，签名）。
    #[instrument(skip(self, body), fields(path = %path))]
    pub async fn post_raw<B>(&self, path: &str, body: &B) -> Result<Value>
    where
        B: Serialize + ?Sized,
    {
        self.inner.post_raw(path, body).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use okx_core::{signer::headers, Credentials};
    use serde::{Deserialize, Serialize};
    use std::{
        collections::VecDeque,
        sync::{Arc, Mutex},
    };

    #[derive(Debug, Deserialize)]
    struct DummyData {
        value: i32,
    }

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct QueryParams {
        inst_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        limit: Option<String>,
    }

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct PostBody {
        name: String,
        count: i32,
    }

    #[derive(Debug)]
    struct RecordedRequest {
        method: Method,
        url: String,
        headers: Vec<(&'static str, String)>,
        body: Option<String>,
    }

    #[derive(Clone, Default)]
    struct FakeTransport {
        responses: Arc<Mutex<VecDeque<Result<(StatusCode, String)>>>>,
        requests: Arc<Mutex<Vec<RecordedRequest>>>,
    }

    impl FakeTransport {
        fn push_ok(&self, status: StatusCode, body: impl Into<String>) {
            self.responses
                .lock()
                .expect("锁 responses 失败")
                .push_back(Ok((status, body.into())));
        }

        fn push_err(&self, err: OkxError) {
            self.responses
                .lock()
                .expect("锁 responses 失败")
                .push_back(Err(err));
        }

        fn take_requests(&self) -> Vec<RecordedRequest> {
            std::mem::take(&mut *self.requests.lock().expect("锁 requests 失败"))
        }
    }

    impl HttpTransport for FakeTransport {
        fn send(
            &self,
            method: Method,
            url: String,
            headers: Vec<(&'static str, String)>,
            body: Option<String>,
        ) -> TransportFuture {
            self.requests
                .lock()
                .expect("锁 requests 失败")
                .push(RecordedRequest {
                    method,
                    url,
                    headers,
                    body,
                });

            let next = self
                .responses
                .lock()
                .expect("锁 responses 失败")
                .pop_front()
                .expect("FakeTransport 未配置返回值");

            Box::pin(async move { next })
        }
    }

    fn inner_client_with_base(
        base_url: &str,
        transport: FakeTransport,
    ) -> OkxRestClientInner<FakeTransport> {
        let cfg = Config::new(Credentials::new("k", "s", "p"))
            .simulated(true)
            .with_rest_url(base_url);
        OkxRestClientInner::new(cfg, transport)
    }

    #[test]
    fn build_url_encodes_special_chars_and_optional_params() {
        let transport = FakeTransport::default();
        let client = inner_client_with_base("https://example.com", transport);

        let params = QueryParams {
            inst_id: "BTC-USDT".into(),
            limit: None,
        };
        let url = client
            .build_url("/api/v5/market/ticker", Some(&params))
            .expect("build_url 应成功");
        assert_eq!(
            url,
            "https://example.com/api/v5/market/ticker?instId=BTC-USDT"
        );
    }

    #[tokio::test]
    async fn get_public_parses_success_response_and_sends_public_headers() {
        let transport = FakeTransport::default();
        transport.push_ok(
            StatusCode::OK,
            r#"{"code":"0","msg":"","data":[{"value":42}]}"#,
        );

        let client = inner_client_with_base("https://example.com", transport.clone());
        let params = QueryParams {
            inst_id: "BTC-USDT".into(),
            limit: None,
        };

        let data = client
            .get_public::<DummyData, _>("/api/v5/mock", Some(&params))
            .await
            .expect("应成功解析响应");
        assert_eq!(data[0].value, 42);

        let req = transport.take_requests().pop().expect("应记录一次请求");
        assert_eq!(req.method, Method::GET);
        assert_eq!(req.url, "https://example.com/api/v5/mock?instId=BTC-USDT");
        assert!(req.body.is_none());
        assert!(req.headers.iter().any(|(k, _)| *k == headers::CONTENT_TYPE));
        assert!(req
            .headers
            .iter()
            .any(|(k, v)| *k == headers::X_SIMULATED_TRADING && v == "1"));
    }

    #[tokio::test]
    async fn get_public_raw_returns_full_payload_even_when_code_is_nonzero() {
        let transport = FakeTransport::default();
        transport.push_ok(
            StatusCode::OK,
            r#"{"code":"51000","msg":"failure","data":[]}"#,
        );

        let client = inner_client_with_base("https://example.com", transport);
        let params = QueryParams {
            inst_id: "BTC-USDT".into(),
            limit: None,
        };

        let raw = client
            .get_public_raw("/api/v5/mock", Some(&params))
            .await
            .expect("raw 调用应成功返回 payload");

        assert_eq!(raw["code"], "51000");
        assert_eq!(raw["msg"], "failure");
    }

    #[tokio::test]
    async fn get_raw_and_post_raw_send_signed_headers_and_body() {
        let transport = FakeTransport::default();
        transport.push_ok(
            StatusCode::OK,
            r#"{"code":"0","msg":"","data":[{"value":7}]}"#,
        );
        transport.push_ok(StatusCode::OK, r#"{"value":1}"#);

        let client = inner_client_with_base("https://example.com", transport.clone());
        let params = QueryParams {
            inst_id: "BTC-USDT".into(),
            limit: None,
        };

        let data = client
            .get::<DummyData, _>("/api/v5/private/mock", Some(&params))
            .await
            .expect("私有 GET 应成功解析");
        assert_eq!(data[0].value, 7);

        let body = PostBody {
            name: "n".into(),
            count: 1,
        };
        let raw = client
            .post_raw("/api/v5/private/post", &body)
            .await
            .expect("私有 POST raw 应成功解析");
        assert_eq!(raw["value"], 1);

        let reqs = transport.take_requests();
        assert_eq!(reqs.len(), 2);

        // 私有 GET：带 query 的 request_path 也应参与签名
        assert_eq!(reqs[0].method, Method::GET);
        assert!(reqs[0].url.contains("/api/v5/private/mock?instId=BTC-USDT"));
        assert!(reqs[0]
            .headers
            .iter()
            .any(|(k, _)| *k == headers::OK_ACCESS_KEY));
        assert!(reqs[0]
            .headers
            .iter()
            .any(|(k, _)| *k == headers::OK_ACCESS_SIGN));
        assert!(reqs[0]
            .headers
            .iter()
            .any(|(k, _)| *k == headers::OK_ACCESS_TIMESTAMP));
        assert!(reqs[0]
            .headers
            .iter()
            .any(|(k, _)| *k == headers::OK_ACCESS_PASSPHRASE));

        // 私有 POST raw：请求体必须是 JSON 串，并参与签名
        assert_eq!(reqs[1].method, Method::POST);
        assert!(reqs[1].url.ends_with("/api/v5/private/post"));
        assert_eq!(reqs[1].body.as_deref(), Some(r#"{"name":"n","count":1}"#));
        assert!(reqs[1]
            .headers
            .iter()
            .any(|(k, _)| *k == headers::OK_ACCESS_SIGN));
    }

    #[tokio::test]
    async fn transport_error_is_propagated() {
        let transport = FakeTransport::default();
        transport.push_err(OkxError::Http("network".into()));

        let client = inner_client_with_base("https://example.com", transport);
        let err = client
            .get_public::<DummyData, ()>("/api/v5/mock", None)
            .await
            .expect_err("应传播传输层错误");
        assert!(matches!(err, OkxError::Http(_)));
    }

    #[tokio::test]
    async fn http_status_error_is_reported_as_http_error() {
        let transport = FakeTransport::default();
        transport.push_ok(StatusCode::INTERNAL_SERVER_ERROR, "oops");

        let client = inner_client_with_base("https://example.com", transport);
        let err = client
            .get_public::<DummyData, ()>("/api/v5/mock", None)
            .await
            .expect_err("HTTP 非 2xx 应返回错误");
        assert!(matches!(err, OkxError::Http(_)));
    }

    #[test]
    fn parse_body_invalid_json_returns_serde_error() {
        let transport = FakeTransport::default();
        let client = inner_client_with_base("https://example.com", transport);
        let err = client
            .parse_body::<DummyData>(StatusCode::OK, "not json")
            .expect_err("非法 JSON 应返回 Serde 错误");
        assert!(matches!(err, OkxError::Serde(_)));
    }

    #[test]
    fn parse_body_returns_api_error_for_failure_code() {
        let transport = FakeTransport::default();
        let client = inner_client_with_base("https://example.com", transport);
        let body = r#"{"code":"51000","msg":"failure","data":[]}"#;
        let err = client
            .parse_body::<DummyData>(StatusCode::OK, body)
            .expect_err("非 0 code 应返回 API 错误");

        match err {
            OkxError::Api { code, msg } => {
                assert_eq!(code, "51000");
                assert_eq!(msg, "failure");
            }
            other => panic!("错误类型不符: {other:?}"),
        }
    }

    #[test]
    fn parse_body_raw_returns_full_payload_on_http_success() {
        let transport = FakeTransport::default();
        let client = inner_client_with_base("https://example.com", transport);
        let body = r#"{"code":"51000","msg":"failure","data":[]}"#;

        let raw = client
            .parse_body_raw(StatusCode::OK, body)
            .expect("raw 响应在 HTTP 成功时应返回完整 JSON");
        assert_eq!(raw["code"], "51000");
    }

    #[test]
    fn new_accepts_proxy_url_when_configured() {
        let cfg = Config::new(Credentials::new("k", "s", "p"))
            .with_rest_url("https://example.com")
            .with_proxy("http://127.0.0.1:8888");

        let _client = OkxRestClient::new(cfg);
    }
}
