use granite_dev_graph_map::domain_review::{review_lane, review_score, DomainCase};

#[test]
fn domain_review_case_is_stable() {
    let case = DomainCase { signal: 47, slack: 20, drag: 10, confidence: 53 };
    assert_eq!(review_score(case), 137);
    assert_eq!(review_lane(case), "watch");
}
