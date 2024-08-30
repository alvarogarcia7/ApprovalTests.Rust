pub fn verify(string: String) {
    assert_eq!(string, "Hello, world!");
}


#[macro_export]
macro_rules! log_invocation {
    () => {
        let parts: Vec<&str> = file!().split("/").collect();
        let parts2 = parts[..parts.len() - 1].to_vec();
        println!("{:?}", parts2);
        println!(
            "Invoked from file: {}, line: {}, module: {}",
            file!(),
            line!(),
            module_path!()
        );
    };
}

