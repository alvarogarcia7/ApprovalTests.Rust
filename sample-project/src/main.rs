#[macro_use]
extern crate library;
fn lib() -> String {
    "Hello, world!".to_string()
}
fn main() {
    println!("{}", lib());
}
#[cfg(test)]
mod tests {
    use crate::lib;

    use super::*;
    #[test]
    pub fn main_execution() {
        log_invocation!(lib());

        library::verify(lib());
    }
    // #[test]
    // pub fn another_execution() {
    //     log_invocation!(lib());
    //
    //     library::verify(lib());
    //     assert_eq!(lib(), "Hello, world!");
    // }
}
