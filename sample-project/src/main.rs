fn lib() -> String {
    return "Hello, world!".to_string();
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
        library::verify(lib());
        assert_eq!(lib(), "Hello, world!");
    }
}
