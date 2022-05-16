#![cfg(feature = "yozuk-skill-jwt")]

mod common;
use common::cmd;
use yozuk_sdk::prelude::*;

#[test]
fn decode() {
    const TOKEN: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.gkHRA068x3WFRsN3gJbKBcLp3z8CfXbuBcW8J3lfLVs";
    assert_eq!(
        cmd(tk!([TOKEN])),
        CommandArgs::new().add_args(["yozuk-skill-jwt", TOKEN])
    );
}
