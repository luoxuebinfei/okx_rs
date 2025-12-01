#![allow(missing_docs)]

use okx_ws::{WsClient, WsMessage};
use tokio_tungstenite::tungstenite::{Error as WsError, Message};

#[test]
fn handle_message_text_and_ping() {
    let text = Message::Text("{\"event\":\"subscribe\"}".into());
    let parsed = WsClient::handle_message(Ok(text))
        .expect("应返回 Some")
        .unwrap();
    match parsed {
        WsMessage::Event { event, .. } => assert!(matches!(event, okx_ws::WsEvent::Subscribe)),
        other => panic!("期望 Event，得到: {other:?}"),
    }

    let ping = Message::Ping(vec![1, 2, 3].into());
    let pong = WsClient::handle_message(Ok(ping))
        .expect("应返回 Some")
        .unwrap();
    assert!(matches!(pong, WsMessage::Pong));
}

#[test]
fn handle_message_pong_close_and_error() {
    // Pong 应被忽略
    let pong = Message::Pong(Vec::<u8>::new().into());
    assert!(WsClient::handle_message(Ok(pong)).is_none());

    // Close 转为连接关闭错误
    let close = Message::Close(None);
    let err = WsClient::handle_message(Ok(close))
        .expect("应返回 Some")
        .unwrap_err();
    assert!(matches!(err, okx_core::OkxError::ConnectionClosed));

    // Tungstenite 错误转为 WebSocket 错误
    let err_msg = WsClient::handle_message(Err(WsError::ConnectionClosed))
        .expect("应返回 Some")
        .unwrap_err();
    assert!(matches!(err_msg, okx_core::OkxError::WebSocket(_)));
}
