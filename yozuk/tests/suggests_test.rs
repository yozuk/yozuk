mod common;
use common::yozuk_global;
use yozuk_sdk::prelude::*;

#[test]
fn test_random_suggests() {
    assert_eq!(yozuk_global().random_suggests(0).len(), 0);
    assert_eq!(yozuk_global().random_suggests(10).len(), 10);
}

#[test]
fn test_suggests() {
    assert_eq!(
        yozuk_global()
            .suggests(5, &tk!(["Generate", "2", "UUIDs"]), &[])
            .len(),
        5
    );
}
