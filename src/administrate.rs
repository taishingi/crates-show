pub mod ji
{
    use std::process::Command;

    pub struct Admin
    {
        directory: String,
    }

    impl Admin
    {
        pub fn new(dir: &str) -> Admin
        {
            Self {
                directory: dir.to_string(),
            }
        }

        pub fn run(self, args: Vec<String>) -> bool
        {
            assert!(Command::new("cargo").args(args).current_dir(self.directory.as_str()).spawn().expect("").wait().unwrap().success());
            true
        }
    }
}