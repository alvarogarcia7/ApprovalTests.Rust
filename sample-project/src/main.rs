fn lib() -> String {
    "Hello, world!".to_string()
}
fn main() {
    println!("{}", lib());
}
#[cfg(test)]
mod tests {
    use crate::lib;
    use library::log_invocation;

    #[test]
    pub fn main_execution() {
        log_invocation!(lib());

        library::verify(lib());
        assert_eq!(lib(), "Hello, world!");
    }
    #[test]
    pub fn another_execution() {
        log_invocation!(lib());

        library::verify(lib());
        assert_eq!(lib(), "Hello, world!");
    }
}
