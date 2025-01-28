pub fn say_hello(name: &str) -> String {
    format!("Hello {}!", name)
}


// Test Module di Library
#[cfg(test)]
mod tests {
    use super::say_hello;

    #[test]
    fn test_say_hello() {
        let hello = say_hello("Suharjin");
        assert_eq!(hello, "Hello Suharjin!")
    }
}