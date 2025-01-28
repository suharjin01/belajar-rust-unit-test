#[test]
fn test_say_hello() {
    let result = hello::say_hello("Suharjin");
    assert_eq!(result, "Hello Suharjin!")
}