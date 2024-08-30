fn lib() -> String {
    "Hello, world!".to_string()
}
fn main() {
    println!("{}", lib());
}
#[cfg(test)]
mod tests {
    use library::log_invocation;
    use crate::lib;

    #[test]
    pub fn main_execution() {
        log_invocation!();

        library::verify(lib());
        assert_eq!(lib(), "Hello, world!");
    }
}
