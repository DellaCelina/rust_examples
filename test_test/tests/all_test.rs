extern crate test_test;

use test_test::test_mod;

#[test]
fn test() {
    let obj = test_mod::TestObj::new(1, 2);

    assert_eq!(obj.a, 1);
    assert_eq!(obj.b, 2);
}

#[test]
#[should_panic(expected = "a can't over 100")]
fn test_panic() {
    let obj = test_mod::TestObj::new(200, 2);
}
