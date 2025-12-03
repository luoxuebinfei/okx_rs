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

#[test]
fn channel_key_roundtrip_for_advanced_channels() {
    let block = Channel::BlockTickers {
        inst_family: Some("BTC-USD".into()),
    };
    let rfqs = Channel::Rfqs {
        inst_family: Some("BTC-USD".into()),
    };
    let recur = Channel::AlgoRecurringBuy {
        algo_id: Some("123".into()),
    };

    for ch in [block, rfqs, recur] {
        let key = channel_key_from(&ch);
        let restored = channel_from_key(&key).expect("序列化后应能恢复");
        assert_eq!(restored.name(), ch.name());
    }
}
