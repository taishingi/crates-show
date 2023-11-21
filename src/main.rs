use std::collections::HashMap;
use std::fs;
use std::ops::Add;
use std::process::Command;
use std::string::String;

use printers::printer::JobStatus;
use rocket::{get, launch, post, routes};
use rocket::http::ContentType;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::serde::{Deserialize, Serialize};
use rocket_dyn_templates::Template;
use rocket_include_static_resources::{static_resources_initializer, static_response_handler};
use scan_dir::ScanDir;

use crate::administrate::ji::Admin;
use crate::printable::ji::Impress;

mod administrate;
mod printable;

#[derive(Serialize, Deserialize)]
struct Tux
{
    url: String,
    editor: String,
    title: String,
    project: String,
    message: Option<String>,
    projects: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
struct TuxFailure
{
    project: String,
    title: String,
    errors: String,
    url: String,
    editor: String,
}

static_response_handler! {
    "/assets/favicon.ico" => favicon => "favicon",
    "/assets/favicon-16x16.png" => favicon_png => "favicon",
    "/assets/manifest.json" => favicon_json => "favicon",
}
fn directory(x: &str) -> String
{
    format!("{}/{}", std::env::var("TUX_DIR").expect(""), x)
}

#[post("/build/<project>")]
fn build_project(project: &str) -> Flash<Redirect>
{
    Admin::new(directory(project).as_str()).run(vec!["build".to_string()], format!("{} has been built successfully", project).as_str(), project, "build", "")
}

#[post("/print/<filename>")]
fn print_file(filename: &str) -> Flash<Redirect>
{
    match Impress::new().print(filename).status {
        JobStatus::SUCCESS => {
            Flash::success(Redirect::to("/"), format!("The {} file has been successfully printed", filename).as_str())
        }
        JobStatus::FAILED => {
            Flash::error(Redirect::to("/"), format!("Failed to printed {}", filename).as_str())
        }
    }
}

#[post("/check/<project>")]
fn check_project(project: &str) -> Flash<Redirect>
{
    Admin::new(directory(project).as_str()).run(vec!["check".to_string()], format!("{} has been checked successfully", project).as_str(), project, "check", "")
}

#[post("/clean/<project>")]
fn clean_project(project: &str) -> Flash<Redirect>
{
    Admin::new(directory(project).as_str()).run(vec!["clean".to_string()], format!("{} has been cleaned successfully", project).as_str(), project, "clean", "")
}

#[post("/doc/<project>")]
fn doc_project(project: &str) -> Flash<Redirect>
{
    Admin::new(directory(project).as_str()).run(vec!["doc".to_string(), "--open".to_string()], format!("{} has been documented successfully", project).as_str(), project, "doc", "")
}

#[get("/show-test/<project>")]
fn test_project_result(project: &str, flash: Option<FlashMessage>) -> Template
{
    Template::render("test", Tux {
        title: format!("{} test results", project).to_string(),
        project: project.to_string(),
        message: flash.map(|flash| flash.message().to_string()),
        projects: HashMap::new(),
        url: String::new().add("/show-test").add("/").add(project),
        editor: std::env::var("TUX_EDITOR").expect("Failed to get tux editor preferences"),
    })
}

#[get("/show-bench/<project>")]
fn bench_project_result(project: &str, flash: Option<FlashMessage>) -> Template
{
    Template::render("bench", Tux {
        title: format!("{} bench results", project).to_string(),
        project: project.to_string(),
        message: flash.map(|flash| flash.message().to_string()),
        projects: HashMap::new(),
        url: String::new().add("/show-bench").add("/").add(project),
        editor: std::env::var("TUX_EDITOR").expect("failed to get tux editor preferences"),
    })
}

#[get("/show-run/<project>")]
fn run_project_result(project: &str, flash: Option<FlashMessage>) -> Template
{
    Template::render("run", Tux {
        title: format!("{} run results", project).to_string(),
        project: project.to_string(),
        message: flash.map(|flash| flash.message().to_string()),
        projects: HashMap::new(),
        url: String::new().add("/show-run").add("/").add(project),
        editor: std::env::var("TUX_EDITOR").expect("failed to get tux editor preferences"),
    })
}

#[get("/show-clippy/<project>")]
fn clippy_project_result(project: &str, flash: Option<FlashMessage>) -> Template
{
    Template::render("clippy", Tux {
        title: format!("{} clippy results", project).to_string(),
        project: project.to_string(),
        message: flash.map(|flash| flash.message().to_string()),
        projects: HashMap::new(),
        url: String::new().add("/show-clippy").add("/").add(project),
        editor: std::env::var("TUX_EDITOR").expect("failed to get tux editor preferences"),
    })
}

#[post("/run/<project>")]
fn run_project(project: &str) -> Flash<Redirect>
{
    Admin::new(directory(project).as_str()).run(vec!["run".to_string()], format!("{} has been launched successfully", project).as_str(), project, "run", "")
}

#[post("/test/<project>")]
fn test_project(project: &str) -> Flash<Redirect>
{
    Admin::new(directory(project).as_str()).run(vec!["test".to_string()], format!("{} no errors detected", project).as_str(), project, "test", "")
}

#[post("/bench/<project>")]
fn bench_project(project: &str) -> Flash<Redirect>
{
    Admin::new(directory(project).as_str()).run(vec!["bench".to_string()], format!("{} bench no error detected", project).as_str(), project, "bench", "")
}

#[post("/update/<project>")]
fn update_project(project: &str) -> Flash<Redirect>
{
    Admin::new(directory(project).as_str()).run(vec!["update".to_string()], format!("{} has been updated successfully", project).as_str(), project, "update", "")
}


#[post("/clippy/<project>")]
fn clippy_project(project: &str) -> Flash<Redirect>
{
    Admin::new(directory(project).as_str()).run(vec!["clippy".to_string()], format!("{} clippy no errors detected by clippy", project).as_str(), project, "clippy", "")
}

#[post("/publish/<project>")]
fn publish_project(project: &str) -> Flash<Redirect>
{
    Admin::new(directory(project).as_str()).run(vec!["publish".to_string()], format!("{} published successfully", project).as_str(), project, "publish", "")
}

#[post("/install/<project>")]
fn install_project(project: &str) -> Flash<Redirect>
{
    Admin::new(directory(format!("{}/.cargo/bin", env!("HOME")).as_str()).as_str()).run(vec!["install".to_string()], format!("{} has been installed successfully", project).as_str(), project, "install", "")
}

#[post("/uninstall/<project>")]
fn uninstall_project(project: &str) -> Flash<Redirect>
{
    Admin::new(format!("{}/.cargo/bin", env!("HOME")).as_str()).run(vec!["uninstall".to_string()], format!("{} has been uninstall successfully", project).as_str(), project, "uninstall", "")
}

#[post("/delete-repo/<project>")]
fn delete_repo(project: &str) -> Flash<Redirect>
{
    fs::remove_dir_all(directory(project).as_str()).expect("failed to remove the directory");
    Flash::success(Redirect::to("/"), format!("{} has been deleted successfully", project))
}

#[get("/yank/<project>/<version>")]
fn yank_repo(project: &str, version: &str) -> Flash<Redirect>
{
    let repo = String::new().add(project).add("@").add(version);
    Admin::new(directory(project).as_str()).run(vec!["yank".to_string(), repo], format!("The project {} version {} has been yanked successfully", project, version).as_str(), project, "yank", version)
}

#[post("/yank/<project>/<version>")]
fn yank_repo_post(project: &str, version: &str) -> Flash<Redirect>
{
    let repo = String::new().add(project).add("@").add(version);
    Admin::new(directory(project).as_str()).run(vec!["yank".to_string(), repo], format!("The project {} version {} has been yanked successfully", project, version).as_str(), project, "yank", version)
}

#[get("/")]
fn index(flash: Option<FlashMessage>) -> Template {
    let project_dir = std::env::var("TUX_DIR").expect("failed to find TUX_DIR variable path");
    let mut projects: HashMap<String, String> = HashMap::new();
    ScanDir::dirs().read(project_dir, |iter| {
        for (entry, name) in iter {
            let p = entry.path().to_str().expect("").to_string().split_off(1);
            projects.insert(name, p);
        }
    }).unwrap();

    Template::render("index", Tux {
        title: "Tux".to_string(),
        message: flash.map(|flash| flash.message().to_string()),
        projects,
        project: "".to_string(),
        url: "".to_string(),
        editor: std::env::var("TUX_EDITOR").expect("failed to get tux editor preferences"),
    })
}

#[post("/open/<editor>/<project>")]
fn open(editor: &str, project: &str) -> Flash<Redirect>
{
    Command::new(editor).arg(directory(project).as_str()).spawn().expect("failed to open editor");
    Flash::success(Redirect::to("/"), format!("The {} project or has been opened", project).as_str())
}

#[get("/fail/<task>/<project>/<version>")]
fn fail_with_version(task: &str, project: &str, version: &str) -> Template {
    Template::render("fail", TuxFailure {
        title: "Tux failed".to_string(),
        project: project.to_string(),
        errors: fs::read_to_string("./logs.txt").expect("failed to parse file"),
        url: String::new().add(task).add("/").add(project).add("/").add(version),
        editor: std::env::var("TUX_EDITOR").expect("failed to get tux editor preferences"),
    })
}

#[get("/fail/<task>/<project>")]
fn fail_normal(task: &str, project: &str) -> Template {
    Template::render("fail", TuxFailure {
        title: "Tux failed".to_string(),
        project: project.to_string(),
        errors: fs::read_to_string("./logs.txt").expect("failed to parse file"),
        url: String::new().add(task).add("/").add(project),
        editor: std::env::var("TUX_EDITOR").expect("failed to get tux editor preferences"),
    })
}

#[get("/assets/css/ji.css")]
fn css() -> (ContentType, &'static str) {
    (ContentType::CSS, fs::read_to_string("web/assets/css/ji.css").expect("").leak())
}

#[get("/assets/js/ji.js")]
fn js() -> (ContentType, &'static str) {
    (ContentType::JavaScript, fs::read_to_string("web/assets/js/ji.js").expect("").leak())
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(static_resources_initializer!(
            "favicon" => "web/assets/favicon.ico",
            "favicon-png" => "web/assets/favicon-16x16.png",
            "/assets/manifest.json" => "web/assets/manifest.json",
        )).attach(Template::fairing()).mount("/", routes![favicon, favicon_png,favicon_json]).mount("/", routes![index,css,js,build_project,check_project,doc_project,run_project,test_project,bench_project,update_project,clippy_project,publish_project,install_project,uninstall_project,clean_project,delete_repo,yank_repo,fail_normal,fail_with_version,yank_repo_post,open,test_project_result,bench_project_result,run_project_result,clippy_project_result,print_file])
}
