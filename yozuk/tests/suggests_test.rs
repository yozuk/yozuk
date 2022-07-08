mod common;
use common::yozuk_global;

#[test]
fn test_random_suggests() {
    assert_eq!(yozuk_global().random_suggests(0).len(), 0);
    assert_eq!(yozuk_global().random_suggests(10).len(), 10);
}
