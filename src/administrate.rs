pub mod ji {
    use std::fs;
    use std::fs::File;
    use std::ops::Add;
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

        pub fn clone(self, repo: &str) -> bool {
            if Path::new("logs.txt").is_file() {
                fs::remove_file("logs.txt").expect("failed to remove file");
            }

            if Path::new(repo).exists() {
                return false;
            }

           

            let f = File::create("logs.txt").expect("failed to create the file");

            let r = String::new()
                .add("git@")
                .add(
                    std::env::var("CRATES_PROVIDER")
                        .expect("Fail to find CRATES_PROVIDER")
                        .as_str(),
                )
                .add(":")
                .add(
                    std::env::var("CRATES_PROVIDER_USERNAME")
                        .expect("Fail to find CRATES_PROVIDER_USERNAME")
                        .as_str(),
                )
                .add("/")
                .add(repo);

            Command::new("git")
                .arg("clone")
                .arg(r.as_str())
                .stderr(f)
                .current_dir(
                    std::env::var("CRATES_DIR")
                        .expect("No found CRATES_DIR")
                        .as_str(),
                )
                .spawn()
                .expect("Git error")
                .wait()
                .expect("msg")
                .success()
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

            output.success()
        }
    }
}
