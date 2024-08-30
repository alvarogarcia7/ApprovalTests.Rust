pub fn verify(string: String) {
    assert_eq!(string, "Hello, world!");
}

#[macro_export]
macro_rules! log_invocation {
    () => {
        let parts: Vec<&str> = file!().split("/").collect();
        let where_to_place_the_test_file = parts[..parts.len() - 1].to_vec().join("/");
        // println!("{:?}", where_to_place_the_test_file);
        let backtraceX = std::backtrace::Backtrace::capture();
        let all_backtrace = format!("{:#?}", backtraceX);
        let mut set_place = false;
        all_backtrace.split("\n").for_each(|line| {
            if line.contains(module_path!()) {
                if !set_place {
                    println!("Backtrace: {:?}", line);
                    set_place = true;
                }
            }
        });

        let test_file = (module_path!().to_string() + ".approved.txt").to_string();
        println!("Test file: {}", test_file);
        println!(
            "Invoked from file: {}, line: {}, module: {}",
            file!(),
            line!(),
            module_path!()
        );
    };
}
