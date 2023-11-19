pub mod ji
{
    use std::fs;
    use std::fs::File;
    use std::path::Path;
    use std::process::Command;

    use rocket::form::validate::Contains;
    use rocket::response::{Flash, Redirect};

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

        pub fn run(self, args: Vec<String>, message: &str, project: &str, task: &str, version: &str) -> Flash<Redirect>
        {
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
                .spawn().expect("failed to run command").wait().expect("");

            if !output.success()
            {
                if version.is_empty() {
                    return Flash::error(Redirect::to(format!("/fail/{}/{}", task, project)), fs::read_to_string("./logs.txt").expect("failed to read file content"));
                }
            }
            if args.clone().contains(&"bench".to_string()) {
                return Flash::success(Redirect::to(format!("/show-bench/{}", project)), fs::read_to_string("output.txt").expect("failed to read file content"));
            } else if args.clone().contains(&"test".to_string()) {
                return Flash::success(Redirect::to(format!("/show-test/{}", project)), fs::read_to_string("output.txt").expect("failed to read file content"));
            } else if args.clone().contains(&"run".to_string()) {
                return Flash::success(Redirect::to(format!("/show-run/{}", project)), fs::read_to_string("output.txt").expect("failed to read file content"));
            } else if args.clone().contains(&"clippy".to_string()) {
                return Flash::success(Redirect::to(format!("/show-clippy/{}", project)), fs::read_to_string("logs.txt").expect("failed to read file content"));
            } else if output.success() {
                return Flash::success(Redirect::to("/"), message);
            }
            Flash::error(Redirect::to(format!("/fail/{}/{}/{}", task, project, version)), fs::read_to_string("./logs.txt").expect("failed to read file content"))
        }
    }
}
