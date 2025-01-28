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

// Panic
fn start_application(host: &str, port: u16) {
    if host == "localhost" {
        panic!("You can't localhost use host!")
    } else {
        println!("Satrting application on {}:{}", host, port)
    }
}

#[test]
#[should_panic]
fn test_start_application() {
    start_application("localhost", 8888);
}

// Ignore
#[test]
#[ignore ]
fn test_ignored() {
    println!("This test is ignored")
}

// Test Result
#[test]
fn test_add_again() -> Result<(), String> {
    let result = add(1, 2);
    if result == 3 {
        Ok(())
    } else {
        Err("1 + 2 should be 3".to_string())
    }
}