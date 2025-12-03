//! HTTP client for OKX REST API.
//!
//! Source: Based on official Python SDK structure
//! - <https://github.com/okxapi/python-okx>

use std::time::Duration;

use reqwest::{Client, Method, Proxy, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use tracing::{debug, instrument};

use okx_core::{types::ApiResponse, Config, OkxError, Result, Signer};

/// REST API client for OKX exchange.
///
/// Handles HTTP requests with automatic authentication signing.
#[derive(Debug, Clone)]
pub struct OkxRestClient {
    /// HTTP client
    http: Client,
    /// Client configuration
    config: Config,
    /// Request signer
    signer: Signer,
}

impl OkxRestClient {
    /// Create a new REST client with the given configuration.
    ///
    /// If a proxy URL is configured, it will be used for all requests.
    ///
    /// # Panics
    ///
    /// Panics if the HTTP client cannot be built.
    #[must_use]
    pub fn new(config: Config) -> Self {
        let mut builder = Client::builder().timeout(Duration::from_secs(config.timeout_secs()));

        // Configure proxy if specified
        if let Some(proxy_url) = config.proxy_url() {
            let proxy = Proxy::all(proxy_url).expect("Invalid proxy URL");
            builder = builder.proxy(proxy);
        }

        let http = builder.build().expect("Failed to build HTTP client");
        let signer = Signer::new(config.credentials().clone());

        Self {
            http,
            config,
            signer,
        }
    }

    /// Create a new REST client with a custom HTTP client.
    #[must_use]
    pub fn with_http_client(config: Config, http: Client) -> Self {
        let signer = Signer::new(config.credentials().clone());
        Self {
            http,
            config,
            signer,
        }
    }

    /// Get the client configuration.
    #[must_use]
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Make a GET request to a public endpoint (no authentication).
    ///
    /// # Arguments
    ///
    /// * `path` - API path (e.g., "/api/v5/market/ticker")
    /// * `params` - Optional query parameters
    #[instrument(skip(self, params), fields(path = %path))]
    pub async fn get_public<T, P>(&self, path: &str, params: Option<&P>) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
        P: Serialize + ?Sized,
    {
        let url = self.build_url(path, params)?;
        debug!("GET (public) {}", url);

        let headers = Signer::generate_public_headers(self.config.is_simulated());
        let response = self.send_request(Method::GET, &url, headers, None).await?;
        self.parse_response(response).await
    }

    /// Make a GET request to a private endpoint (with authentication).
    ///
    /// # Arguments
    ///
    /// * `path` - API path (e.g., "/api/v5/account/balance")
    /// * `params` - Optional query parameters
    #[instrument(skip(self, params), fields(path = %path))]
    pub async fn get<T, P>(&self, path: &str, params: Option<&P>) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
        P: Serialize + ?Sized,
    {
        let url = self.build_url(path, params)?;
        let request_path = self.extract_request_path(&url);
        debug!("GET (private) {}", url);

        let headers =
            self.signer
                .generate_headers("GET", &request_path, "", self.config.is_simulated());
        let response = self.send_request(Method::GET, &url, headers, None).await?;
        self.parse_response(response).await
    }

    /// Make a POST request to a private endpoint (with authentication).
    ///
    /// # Arguments
    ///
    /// * `path` - API path (e.g., "/api/v5/trade/order")
    /// * `body` - Request body
    #[instrument(skip(self, body), fields(path = %path))]
    pub async fn post<T, B>(&self, path: &str, body: &B) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let url = format!("{}{}", self.config.rest_url(), path);
        let body_str = serde_json::to_string(body)?;
        debug!("POST {} body={}", url, body_str);

        let headers =
            self.signer
                .generate_headers("POST", path, &body_str, self.config.is_simulated());
        let response = self
            .send_request(Method::POST, &url, headers, Some(body_str))
            .await?;
        self.parse_response(response).await
    }

    /// Build URL with optional query parameters.
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

    /// Extract request path (including query string) from full URL.
    fn extract_request_path(&self, url: &str) -> String {
        url.strip_prefix(self.config.rest_url())
            .unwrap_or(url)
            .to_string()
    }

    /// Send HTTP request with headers.
    async fn send_request(
        &self,
        method: Method,
        url: &str,
        headers: Vec<(&'static str, String)>,
        body: Option<String>,
    ) -> Result<Response> {
        let mut request = self.http.request(method, url);

        for (name, value) in headers {
            request = request.header(name, value);
        }

        if let Some(body) = body {
            request = request.body(body);
        }

        request
            .send()
            .await
            .map_err(|e| OkxError::Http(e.to_string()))
    }

    /// Parse API response and extract data.
    async fn parse_response<T: DeserializeOwned>(&self, response: Response) -> Result<Vec<T>> {
        let status = response.status();
        let text = response
            .text()
            .await
            .map_err(|e| OkxError::Http(format!("Failed to read response: {e}")))?;

        debug!("Response status={} body={}", status, text);

        self.parse_body(status, &text)
    }

    /// Parse response body with status code.
    fn parse_body<T: DeserializeOwned>(&self, status: StatusCode, text: &str) -> Result<Vec<T>> {
        if !status.is_success() {
            return Err(OkxError::Http(format!(
                "HTTP error: status={}, body={}",
                status, text
            )));
        }

        let api_response: ApiResponse<T> = serde_json::from_str(text)?;

        if api_response.is_success() {
            Ok(api_response.data)
        } else {
            Err(OkxError::api(api_response.code, api_response.msg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use okx_core::{types::Ticker, Config, Credentials};
    use reqwest::StatusCode;
    use serde::{Deserialize, Serialize};
    use std::{
        io::{Read, Write},
        net::{Shutdown, TcpListener, TcpStream},
        thread,
        time::Duration,
    };

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct QueryParams {
        inst_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        limit: Option<String>,
    }

    #[derive(Debug, Deserialize)]
    struct DummyData {
        value: i32,
    }

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct SpecialParams {
        keyword: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        inst_type: Option<String>,
    }

    #[derive(Debug, Serialize)]
    struct OptionalOnly {
        #[serde(skip_serializing_if = "Option::is_none")]
        foo: Option<String>,
    }

    fn client_with_base(base_url: &str) -> OkxRestClient {
        let config = Config::new(Credentials::new("k", "s", "p"))
            .with_rest_url(base_url.to_string())
            .with_timeout_secs(1);
        OkxRestClient::new(config)
    }

    #[test]
    fn build_url_and_extract_request_path_cover_optional_params() {
        let client = client_with_base("https://example.com");
        let params = QueryParams {
            inst_id: "BTC-USDT".into(),
            limit: Some("10".into()),
        };

        let url = client
            .build_url("/api/v5/mock", Some(&params))
            .expect("应构造 URL");
        assert_eq!(
            url,
            "https://example.com/api/v5/mock?instId=BTC-USDT&limit=10"
        );

        let request_path = client.extract_request_path(&url);
        assert_eq!(request_path, "/api/v5/mock?instId=BTC-USDT&limit=10");

        let base_only = client
            .build_url::<()>("/api/v5/mock", None::<&()>)
            .expect("无参数也应成功");
        assert_eq!(base_only, "https://example.com/api/v5/mock");

        // 非本项目 base 前缀时应原样返回
        let full_url = "https://other.com/keep";
        assert_eq!(client.extract_request_path(full_url), full_url);
    }

    #[test]
    fn build_url_encodes_special_chars_and_vector_params() {
        let client = client_with_base("https://example.com");
        let params = SpecialParams {
            keyword: "BTC/USDT test".into(),
            inst_type: Some("SWAP,SPOT".into()),
        };

        let url = client
            .build_url("/api/v5/search", Some(&params))
            .expect("应成功编码参数");

        assert_eq!(
            url,
            "https://example.com/api/v5/search?keyword=BTC%2FUSDT+test&instType=SWAP%2CSPOT"
        );
    }

    #[test]
    fn build_url_ignores_empty_query() {
        let client = client_with_base("https://example.com");
        let params = OptionalOnly { foo: None };

        let url = client
            .build_url("/api/v5/empty", Some(&params))
            .expect("空参数也应成功");
        assert_eq!(url, "https://example.com/api/v5/empty");
    }

    #[test]
    fn spawn_response_server_serves_body() {
        let Some((base_url, handle)) = (0..3).find_map(|_| {
            let server = spawn_response_server("200 OK", r#"{"value":1}"#);
            if server.is_none() {
                thread::sleep(Duration::from_millis(10));
            }
            server
        }) else {
            eprintln!("无法绑定本地端口，跳过验证响应内容");
            return;
        };

        let addr = base_url.strip_prefix("http://").expect("需含 http 前缀");
        let mut stream = TcpStream::connect(addr).expect("应能连接到本地服务");
        stream
            .write_all(b"GET / HTTP/1.1\r\nHost: test\r\n\r\n")
            .expect("应能写入请求");

        let mut buf = String::new();
        stream.read_to_string(&mut buf).expect("应能读取响应体");
        assert!(buf.contains("200 OK"));
        assert!(buf.contains(r#"{"value":1}"#));

        handle.join().expect("响应线程应正常结束");
    }

    #[test]
    fn parse_body_parses_success_response() {
        let client = client_with_base("https://example.com");
        let body = r#"{"code":"0","msg":"","data":[{"value":7}]}"#;

        let data = client
            .parse_body::<DummyData>(StatusCode::OK, body)
            .expect("成功响应应解析为数据");

        assert_eq!(data.len(), 1);
        assert_eq!(data[0].value, 7);
    }

    #[test]
    fn parse_body_invalid_json_returns_serde_error() {
        let client = client_with_base("https://example.com");
        let err = client
            .parse_body::<DummyData>(StatusCode::OK, "not json")
            .expect_err("非法 JSON 应返回 Serde 错误");

        assert!(matches!(err, OkxError::Serde(_)));
    }

    #[test]
    fn parse_body_returns_api_error_for_failure_code() {
        let body = r#"{"code":"51000","msg":"failure","data":[]}"#;
        let client = client_with_base("https://example.com");
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

    #[tokio::test]
    async fn parse_response_via_get_public_hits_success_path() {
        // 简易本地 HTTP 服务，返回成功响应，确保走到 parse_response 分支
        let body = r#"{"code":"0","msg":"","data":[{"value":42}]}"#;
        let Some((base_url, handle)) = spawn_ok_server("200 OK", body) else {
            eprintln!("无法绑定本地端口，跳过本用例");
            return;
        };

        let client = client_with_base(&base_url);
        let params = QueryParams {
            inst_id: "BTC-USDT".into(),
            limit: None,
        };

        let data = client
            .get_public::<DummyData, _>("/api/v5/mock", Some(&params))
            .await
            .expect("应成功解析响应");

        assert_eq!(data[0].value, 42);
        handle.join().expect("本地服务线程应正常结束");
    }

    #[test]
    fn parse_body_propagates_http_status_errors() {
        let body = r#"{"code":"0","msg":"","data":[]}"#;
        let client = client_with_base("https://example.com");
        let err = client
            .parse_body::<DummyData>(StatusCode::INTERNAL_SERVER_ERROR, body)
            .expect_err("HTTP 非 2xx 应返回错误");

        match err {
            OkxError::Http(msg) => assert!(
                msg.contains("500"),
                "HTTP 错误信息应包含状态码，当前: {msg}"
            ),
            other => panic!("错误类型不符: {other:?}"),
        }
    }

    #[test]
    fn parse_body_validates_ticker_price_ordering() {
        let body = r#"{"code":"0","msg":"","data":[{"instType":"SPOT","instId":"BTC-USDT","last":"30123.456789","askPx":"30124.12","bidPx":"30122.34","ts":"1700000000000"}]}"#;
        let client = client_with_base("https://example.com");
        let tickers = client
            .parse_body::<Ticker>(StatusCode::OK, body)
            .expect("Ticker 解析应成功");

        let ticker = &tickers[0];
        let last: f64 = ticker.last.parse().expect("成交价应为数值");
        let ask: f64 = ticker.ask_px.parse().expect("卖一价应为数值");
        let bid: f64 = ticker.bid_px.parse().expect("买一价应为数值");
        assert!(bid <= last && last <= ask, "价格应满足买<=成交<=卖");
        assert_eq!(ticker.last, "30123.456789");
    }

    #[tokio::test]
    async fn get_public_returns_http_error_on_rate_limit() {
        let body = r#"{"code":"50011","msg":"too many requests","data":[]}"#;
        let Some((base_url, handle)) = spawn_response_server("429 Too Many Requests", body) else {
            eprintln!("无法绑定本地端口，跳过限流用例");
            return;
        };

        let client = client_with_base(&base_url);
        let err = client
            .get_public::<DummyData, _>("/api/v5/mock", None::<&()>)
            .await
            .expect_err("限流应返回 HTTP 错误");

        match err {
            OkxError::Http(msg) => {
                assert!(msg.contains("429"));
                assert!(msg.contains("too many requests"));
            }
            other => panic!("错误类型不符: {other:?}"),
        }

        handle.join().expect("本地服务线程应正常结束");
    }

    #[tokio::test]
    async fn get_public_times_out_when_server_hangs() {
        let Some((base_url, handle)) = spawn_hanging_server(Duration::from_millis(1200)) else {
            eprintln!("无法绑定本地端口，跳过超时用例");
            return;
        };

        let client = client_with_base(&base_url);
        let start = std::time::Instant::now();
        let err = client
            .get_public::<DummyData, _>("/api/v5/mock", None::<&()>)
            .await
            .expect_err("服务端挂起应触发超时错误");

        match err {
            OkxError::Http(msg) => {
                let hit = ["timed out", "deadline", "elapsed", "error sending request"]
                    .iter()
                    .any(|key| msg.contains(key));
                assert!(hit, "超时/发送错误信息不符: {msg}");
            }
            other => panic!("错误类型不符: {other:?}"),
        }
        assert!(
            start.elapsed() < Duration::from_secs(3),
            "超时错误不应拖延过长"
        );
        handle.join().expect("本地挂起线程应正常结束");
    }

    #[test]
    fn spawn_hanging_server_accepts_and_closes_after_delay() {
        let Some((base_url, handle)) = (0..3).find_map(|_| {
            let server = spawn_hanging_server(Duration::from_millis(30));
            if server.is_none() {
                thread::sleep(Duration::from_millis(10));
            }
            server
        }) else {
            eprintln!("无法绑定本地端口，跳过挂起校验");
            return;
        };

        let addr = base_url.strip_prefix("http://").expect("需含 http 前缀");
        let mut stream = TcpStream::connect(addr).expect("应能连接到挂起服务");
        stream.write_all(b"ping").expect("应能写入触发读取");

        // 等待服务端休眠并关闭
        thread::sleep(Duration::from_millis(80));
        let mut buf = [0u8; 8];
        let _ = stream.read(&mut buf);

        handle.join().expect("挂起线程应正常结束");
    }

    #[tokio::test]
    async fn get_public_propagates_network_error() {
        let client = client_with_base("http://127.0.0.1:9");
        let err = client
            .get_public::<DummyData, _>("/api/v5/mock", None::<&()>)
            .await
            .expect_err("无监听端口应触发网络错误");

        match err {
            OkxError::Http(msg) => assert!(
                msg.contains("127.0.0.1"),
                "网络错误应包含主机信息，当前: {msg}"
            ),
            other => panic!("错误类型不符: {other:?}"),
        }
    }

    fn spawn_response_server(status: &str, body: &str) -> Option<(String, thread::JoinHandle<()>)> {
        let listener = TcpListener::bind("127.0.0.1:0").ok()?;
        let addr = listener.local_addr().ok()?;
        let status = status.to_string();
        let body = body.to_string();

        let handle = thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.accept() {
                let mut buf = [0u8; 512];
                let _ = stream.read(&mut buf); // 忽略请求内容
                let resp = format!(
                    "HTTP/1.1 {status}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
                    body.as_bytes().len()
                );
                let _ = stream.write_all(resp.as_bytes());
            }
        });

        Some((format!("http://{}", addr), handle))
    }

    fn spawn_ok_server(status: &str, body: &str) -> Option<(String, thread::JoinHandle<()>)> {
        spawn_response_server(status, body)
    }

    fn spawn_hanging_server(delay: Duration) -> Option<(String, thread::JoinHandle<()>)> {
        let listener = TcpListener::bind("127.0.0.1:0").ok()?;
        let addr = listener.local_addr().ok()?;

        let handle = thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.accept() {
                let mut buf = [0u8; 512];
                let _ = stream.read(&mut buf); // 读取后保持挂起
                thread::sleep(delay);
                let _ = stream.shutdown(Shutdown::Both);
            }
        });

        Some((format!("http://{}", addr), handle))
    }
}
