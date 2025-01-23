fn main() {
    println!("Hello, world!");
}

#[test]
fn test_simple() {
    println!("Hello Test");
}


fn add(a: i32, b:i32) -> i32 {
    a + b
}

#[test]
fn test_add() {
    let result = add(5, 6);
    assert_eq!(result, 11, "5 + 6 Should be 11")
}