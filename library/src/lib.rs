pub fn verify(string: String) {
    assert_eq!(string, "Hello, world!");
}

#[macro_export]
macro_rules! log_invocation {
    () => {
        let parts: Vec<&str> = file!().split("/").collect();
        let where_to_place_the_test_file = parts[1..parts.len() - 1].to_vec().join("/");
        // println!("{:?}", where_to_place_the_test_file);
        let backtraceX = std::backtrace::Backtrace::capture();
        let all_backtrace = format!("{:#?}", backtraceX);
        let mut line_has_been_found = 0;
        let mut caller_function_name = "";
        all_backtrace.split("\n").for_each(|line| {
            if line.contains(module_path!()) {
                if line_has_been_found == 0 {
                    let parts = line.trim().split(", ").collect::<Vec<&str>>();
                    let parts1 = parts[0].splitn(1, ":").collect::<Vec<&str>>();
                    let parts2 = parts[0].split("\"").collect::<Vec<&str>>();
                    caller_function_name = parts2[1];

                    // println!("Backtrace: {:?}", parts);
                    // println!("Backtrace: {:?}", parts1);
                    // println!("Backtrace: {:?}", parts2);
                    line_has_been_found = 1;
                }
            }
        });

        let test_file: String = (caller_function_name.to_string() + ".approved.txt");
        println!("Test file: {}", test_file.to_string());
        println!(
            "Invoked from file: {}, line: {}, module: {}, function: {}",
            file!(),
            line!(),
            module_path!(),
            caller_function_name
        );

        use std::fs::File;
        use std::io::Write;
        use std::path::Path;

        fn create_file_with_content(file_path: &str, content: &str) -> std::io::Result<()> {
            let path = Path::new(file_path);
            println!(
                "From {:?}, creating file: {:?}",
                std::env::current_dir(),
                path
            );
            let mut file = File::create(&path)?;
            file.write_all(content.as_bytes())?;
            Ok(())
        }

        let file_path: String = where_to_place_the_test_file + "/" + test_file.as_str();
        let file_path: &str = file_path.as_str();
        let content = "Hello, world!";
        match create_file_with_content(file_path, content) {
            Ok(_) => println!("File created successfully."),
            Err(e) => eprintln!("Failed to create file: {}", e),
        }
    };
}
