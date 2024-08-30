fn lib() -> String {
    "Hello, world!".to_string()
}
fn main() {
    println!("{}", lib());
}
#[cfg(test)]
mod tests {
    use crate::lib;
    use library::{log_invocation, who_called_me};

    #[test]
    pub fn main_execution() {
        log_invocation!();

        // https://stackoverflow.com/questions/60714284/how-can-i-access-a-functions-calling-location-each-time-its-called
        who_called_me();

        library::verify(lib());
        assert_eq!(lib(), "Hello, world!");
    }
}
