mod common;
use common::yozuk_global;
use yozuk_sdk::prelude::*;

#[test]
fn test_random_suggestions() {
    assert_eq!(yozuk_global().random_suggestions(0).len(), 0);
    assert_eq!(yozuk_global().random_suggestions(10).len(), 10);
}

#[test]
fn test_suggestions() {
    assert_eq!(
        yozuk_global()
            .suggestions(5, &tk!(["Generate", "2", "UUIDs"]), &[])
            .len(),
        1
    );
}
