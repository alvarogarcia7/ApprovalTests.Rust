pub fn verify(string: String) {
    assert_eq!(string, "Hello, world!");
}


#[macro_export]
macro_rules! log_invocation {
    () => {
        println!(
            "Invoked from file: {}, line: {}, module: {}",
            file!(),
            line!(),
            module_path!()
        );
    };
}
