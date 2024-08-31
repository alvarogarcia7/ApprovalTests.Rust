use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

mod diff_files;

pub fn verify(string: String) {
    assert_eq!(string, "Hello, world!");
}

#[macro_export]
macro_rules! log_invocation {
    ($func:expr) => {
        let result = $func;
        println!("Invoked function returns: {}", result);
        let parts: Vec<&str> = file!().split("/").collect();
        let where_to_place_the_test_file = parts[1..parts.len() - 1].to_vec().join("/");
        println!(
            "where_to_place_the_test_file: {:?}",
            where_to_place_the_test_file
        );
        let mut caller_file_name = parts[parts.len() - 1..]
            .to_vec()
            .join("/")
            .split(".")
            .collect::<Vec<&str>>()[0]
            .to_string();
        // let mut caller_file_path = parts.join("/").split(".").collect::<Vec<&str>>()[0];
        // println!(
        //     "where_to_place_the_test: {:?}",
        //     caller_file_path.to_string()
        // );
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
                    caller_file_name += "::";
                    caller_file_name += parts2[1];

                    // println!("Backtrace: {:?}", parts);
                    // println!("Backtrace: {:?}", parts1);
                    // println!("Backtrace: {:?}", parts2);
                    line_has_been_found = 1;
                }
            }
        });

        // let approved_file: String = (caller_file_name.to_string().clone() + ".approved.txt");
        // println!("Test file: {}", approved_file.to_string().clone());
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

        // let file_path: String = where_to_place_the_test_file + "/" + test_file.as_str();
        // let file_path: &str = file_path.as_str();
        // match create_file_with_content(file_path, result.as_str()) {
        //     Ok(_) => println!("File created successfully."),
        //     Err(e) => eprintln!("Failed to create file: {}", e),
        // }

        use std::io::{self, BufRead};
        //
        // fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
        // where
        //     P: AsRef<Path>,
        // {
        //     println!("Reading file: {:?}", filename.as_ref());
        //     let file = File::open(filename)?;
        //     let buf = io::BufReader::new(file);
        //     buf.lines().collect()
        // }
        //
        // pub(crate) fn diff_files(file1: &str, file2: &str) -> io::Result<()> {
        //     println!("Comparing files: {:?} and {:?}", file1, file2);
        //     Ok(())
        // }

        // let received_file = "A";
        // // (caller_file_path.to_string() + caller_file_name.as_str() + ".received.txt").as_str();
        // // let string = (caller_file_path.to_string() + received_file).as_str();
        // let string = "B";
        // if let Err(e) = diff_files(approved_file, string) {
        //     eprintln!("Error: {}", e);
        // }
    };
}
#[allow(dead_code)]
fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    println!(
        "From {:?}, reading file: {:?}",
        std::env::current_dir(),
        filename.as_ref()
    );
    println!("Reading file: {:?}", filename.as_ref());
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);
    buf.lines().collect()
}

#[allow(dead_code)]
pub(crate) fn diff_files(file1: &str, file2: &str) -> io::Result<()> {
    let lines1 = read_lines(file1)?.clone();
    let lines2 = read_lines(file2)?.clone();

    let max_len = std::cmp::max(lines1.len(), lines2.len());

    for i in 0..max_len {
        let default = &String::new();
        let line1 = lines1.get(i).unwrap_or(default);
        let line2 = lines2.get(i).unwrap_or(default);

        if line1 != line2 {
            println!("Difference at line {}:", i + 1);
            println!("File1: {}", line1);
            println!("File2: {}", line2);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::diff_files;

    #[test]
    pub fn diff_the_same_file() {
        diff_files(
            "./data/main::sample_project::tests::another_execution.approved.txt",
            "./data/main::sample_project::tests::another_execution.approved.txt",
        )
        .unwrap();
    }

    #[test]
    pub fn diff_different_files_with_the_same_contents() {
        diff_files(
            "./data/main::sample_project::tests::another_execution.approved.txt",
            "./data/main::sample_project::tests::another_execution_copy.approved.txt",
        )
        .unwrap();
    }
}
