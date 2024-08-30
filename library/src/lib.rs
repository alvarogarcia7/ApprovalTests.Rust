pub fn verify(string: String) {
    assert_eq!(string, "Hello, world!");
}

#[macro_export]
macro_rules! log_invocation {
    () => {
        let parts: Vec<&str> = file!().split("/").collect();
        let where_to_place_the_test_file = parts[..parts.len() - 1].to_vec().join("/");
        // println!("{:?}", where_to_place_the_test_file);

        let test_file = (module_path!().to_string() + ".approved.txt").to_string();
        println!("Test file: {}", test_file);
        println!(
            "Invoked from file: {}, line: {}, module: {}, function: {}",
            file!(),
            line!(),
            module_path!(),
            std::any::type_name::<fn()>()
        );
    };
}
