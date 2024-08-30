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
        let mut line_has_been_found = 0;
        let mut selected = "";
        all_backtrace.split("\n").for_each(|line| {
            if line.contains(module_path!()) {
                if line_has_been_found == 0 {
                    let parts = line.trim().split(", ").collect::<Vec<&str>>();
                    let parts1 = parts[0].splitn(1, ":").collect::<Vec<&str>>();
                    let parts2 = parts[0].split("\"").collect::<Vec<&str>>();
                    selected = parts2[1];

                    // println!("Backtrace: {:?}", parts);
                    // println!("Backtrace: {:?}", parts1);
                    // println!("Backtrace: {:?}", parts2);
                    line_has_been_found = 1;
                }
            }
        });

        let test_file = (module_path!().to_string() + ".approved.txt").to_string();
        println!("Test file: {}", test_file);
        println!(
            "Invoked from file: {}, line: {}, module: {}, function: {}",
            file!(),
            line!(),
            module_path!(),
            selected
        );
    };
}
