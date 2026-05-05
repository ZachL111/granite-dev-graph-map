use granite_dev_graph_map::{classify, score, Signal};
#[test]
fn fixture_decisions() {
    let signal = Signal { demand: 70, capacity: 90, latency: 12, risk: 6, weight: 7 };
    assert_eq!(score(signal), 199);
    assert_eq!(classify(signal), "accept");
    let signal = Signal { demand: 73, capacity: 89, latency: 10, risk: 13, weight: 6 };
    assert_eq!(score(signal), 170);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 79, capacity: 83, latency: 15, risk: 19, weight: 6 };
    assert_eq!(score(signal), 131);
    assert_eq!(classify(signal), "review");
}
