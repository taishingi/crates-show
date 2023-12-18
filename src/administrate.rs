pub mod ji {
    use std::fs;
    use std::fs::File;
    use std::path::Path;
    use std::process::Command;

    pub struct Admin {
        directory: String,
    }

    impl Admin {
        pub fn new(dir: &str) -> Admin {
            Self {
                directory: dir.to_string(),
            }
        }

        pub fn run(self, args: Vec<String>) -> bool {
            if Path::new("logs.txt").is_file() {
                fs::remove_file("logs.txt").expect("failed to remove the file");
            }

            if Path::new("output.txt").is_file() {
                fs::remove_file("output.txt").expect("failed to remove the file");
            }

            let output = File::create("output.txt").expect("failed to create file");
            let f = File::create("logs.txt").expect("failed to create file");
            let output = Command::new("cargo")
                .stdout(output)
                .stderr(f)
                .args(args.clone())
                .current_dir(self.directory.as_str())
                .spawn()
                .expect("failed to run command")
                .wait()
                .expect("");

            return output.success();
        }
    }
}
