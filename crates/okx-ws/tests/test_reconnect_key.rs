#![allow(missing_docs)]

use okx_ws::{channel_from_key, channel_key_from, Channel};

#[test]
fn channel_key_roundtrip() {
    let channel = Channel::Tickers {
        inst_id: "BTC-USDT".into(),
    };
    let key = channel_key_from(&channel);
    let restored = channel_from_key(&key).expect("应解析回 Channel");
    match restored {
        Channel::Tickers { inst_id } => assert_eq!(inst_id, "BTC-USDT"),
        other => panic!("通道类型不匹配: {other:?}"),
    }
}
