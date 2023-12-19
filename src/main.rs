use crate::administrate::ji::Admin;
use crate::printable::ji::Impress;
use printers::printer::JobStatus;
use rocket::fs::NamedFile;
use rocket::http::ContentType;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, launch, post, routes, uri};
use rocket_dyn_templates::Template;
use rocket_include_static_resources::{static_resources_initializer, static_response_handler};
use scan_dir::ScanDir;
use sqlite::State;
use std::collections::HashMap;
use std::fs::{self};
use std::ops::Add;
use std::path::Path;
use std::process::Command;
use std::str;
use std::string::String;

mod administrate;
mod printable;

#[derive(Serialize, Deserialize)]
struct Tux {
    url: String,
    editor: String,
    title: String,
    project: String,
    message: Option<String>,
    projects: HashMap<String, String>,
}
#[derive(Serialize, Deserialize)]
struct TuxRun {
    url: String,
    editor: String,
    title: String,
    project: String,
    message: String,
    projects: HashMap<String, String>,
    log: String,
}

#[derive(Serialize, Deserialize)]
struct TuxManage {
    editor: String,
    title: String,
    project: String,
    readme: String,
    projects: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
struct TuxTimeline {
    title: String,
    projects: HashMap<String, String>,
    project: String,
}

#[derive(Serialize, Deserialize)]
struct TuxFailure {
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
fn directory(x: &str) -> String {
    format!("{}/{}", std::env::var("TUX_DIR").expect(""), x)
}

#[get("/build/<project>")]
fn build_project(project: &str) -> Template {
    let msg: String = match Admin::new(directory(project).as_str()).run(vec!["build".to_string()]) {
        true => format!("The project {} has been built successfully", project),
        false => format!("Failed to build the {} project", project),
    };

    let debug = fs::read_to_string("logs.txt").expect("msg");
    Template::render(
        "build",
        TuxRun {
            url: String::new().add("/build/").add(project),
            editor: editor(),
            title: format!("Build - {}", project),
            project: project.to_string(),
            message: msg,
            projects: projects(),
            log: debug,
        },
    )
}

#[post("/print/<filename>")]
fn print_file(filename: &str) -> Flash<Redirect> {
    match Impress::new().print(filename).status {
        JobStatus::SUCCESS => Flash::success(
            Redirect::to("/"),
            format!("The {} file has been successfully printed", filename).as_str(),
        ),
        JobStatus::FAILED => Flash::error(
            Redirect::to("/"),
            format!("Failed to printed {}", filename).as_str(),
        ),
    }
}

#[post("/clean-timeline/<project>")]
fn clean_timeline(project: &str) -> Flash<Redirect> {
    fs::remove_file(format!("{}/{}.db", directory(project), project).as_str())
        .expect("failed to remove database");
    Flash::success(
        Redirect::to("/"),
        format!("The {} project timeline has been cleaned", project).as_str(),
    )
}

#[get("/check/<project>")]
fn check_project(project: &str) -> Template {
    let msg: String = match Admin::new(directory(project).as_str()).run(vec!["check".to_string()]) {
        true => format!("The project {} has been checked successfully", project),
        false => format!("Failed to check the {} project", project),
    };

    let debug = fs::read_to_string("logs.txt").expect("msg");
    Template::render(
        "check",
        TuxRun {
            url: String::new().add("/check/").add(project),
            editor: editor(),
            title: format!("Check - {} ", project),
            project: project.to_string(),
            message: msg,
            projects: projects(),
            log: debug,
        },
    )
}

#[get("/clean/<project>")]
fn clean_project(project: &str) -> Template {
    let msg: String = match Admin::new(directory(project).as_str()).run(vec!["clean".to_string()]) {
        true => format!("The project {} has been cleaned successfully", project),
        false => format!("Failed to clean the {} project", project),
    };

    let debug = fs::read_to_string("logs.txt").expect("msg");
    Template::render(
        "clean",
        TuxRun {
            url: String::new().add("/clean/").add(project),
            editor: editor(),
            title: format!("Clean - {}", project),
            project: project.to_string(),
            message: msg,
            projects: projects(),
            log: debug,
        },
    )
}

#[get("/doc/<project>")]
fn doc_project(project: &str) -> Template {
    let msg: String = match Admin::new(directory(project).as_str()).run(vec!["doc".to_string()]) {
        true => format!("The project {} has been documented successfully", project),
        false => format!("Failed to build documentation for the {} project", project),
    };

    let debug = fs::read_to_string("logs.txt").expect("msg");
    Template::render(
        "doc",
        TuxRun {
            url: String::new().add("/doc/").add(project),
            editor: editor(),
            title: format!("Documentation - {}", project),
            project: project.to_string(),
            message: msg,
            projects: projects(),
            log: debug,
        },
    )
}

#[get("/bench/<project>")]
fn bench_project(project: &str) -> Template {
    let msg: String = match Admin::new(directory(project).as_str()).run(vec!["bench".to_string()]) {
        true => format!("The bench runned successfully for {} project", project),
        false => format!("Failed to run bench    for the {} project", project),
    };

    let debug = fs::read_to_string("logs.txt").expect("msg");
    Template::render(
        "bench",
        TuxRun {
            url: String::new().add("/bench/").add(project),
            editor: editor(),
            title: format!("Bench - {}", project),
            project: project.to_string(),
            message: msg,
            projects: projects(),
            log: debug,
        },
    )
}

#[get("/clippy/<project>")]
fn clippy_project(project: &str) -> Template {
    let msg: String = match Admin::new(directory(project).as_str()).run(vec!["clippy".to_string()])
    {
        true => format!("The project {} has been checked successfully", project),
        false => format!("Failure founded for the {} project", project),
    };

    let debug = fs::read_to_string("logs.txt").expect("msg");
    Template::render(
        "clippy",
        TuxRun {
            url: String::new().add("/clippy/").add(project),
            editor: editor(),
            title: format!("Clippy - {}", project),
            project: project.to_string(),
            message: msg,
            projects: projects(),
            log: debug,
        },
    )
}

#[get("/run/<project>")]
fn run_project(project: &str) -> Template {
    let msg: String = match Admin::new(directory(project).as_str()).run(vec!["run".to_string()]) {
        true => format!("The project {} has runned successfully", project),
        false => format!("Failed to run the {} project", project),
    };

    let debug = fs::read_to_string("logs.txt").expect("msg");
    Template::render(
        "run",
        TuxRun {
            url: String::new().add("/run/").add(project),
            editor: editor(),
            title: format!("Run - {}", project),
            project: project.to_string(),
            message: msg,
            projects: projects(),
            log: debug,
        },
    )
}

#[get("/test/<project>")]
fn test_project(project: &str) -> Template {
    let msg: String = match Admin::new(directory(project).as_str()).run(vec!["test".to_string()]) {
        true => format!("Test passes for the {} project", project),
        false => format!("Failure for the {} project", project),
    };

    let debug = fs::read_to_string("logs.txt").expect("msg");
    Template::render(
        "test",
        TuxRun {
            url: String::new().add("/test/").add(project),
            editor: editor(),
            title: format!("Test - {}", project),
            project: project.to_string(),
            message: msg,
            projects: projects(),
            log: debug,
        },
    )
}

#[get("/update/<project>")]
fn update_project(project: &str) -> Template {
    let msg: String = match Admin::new(directory(project).as_str()).run(vec!["update".to_string()])
    {
        true => format!("The project {} has been updated succcessfully", project),
        false => format!("Failed to update the {} project", project),
    };

    let debug = fs::read_to_string("logs.txt").expect("msg");
    Template::render(
        "update",
        TuxRun {
            url: String::new().add("/update/").add(project),
            editor: editor(),
            title: format!("Update - {}", project),
            project: project.to_string(),
            message: msg,
            projects: projects(),
            log: debug,
        },
    )
}

#[get("/publish/<project>")]
fn publish_project(project: &str) -> Template {
    let msg: String = match Admin::new(directory(project).as_str()).run(vec!["publish".to_string()])
    {
        true => format!(
            "The project has been published successfully at https://crates.io/creates/{}",
            project
        ),
        false => format!("Failed to publish the {} project", project),
    };

    let debug = fs::read_to_string("logs.txt").expect("msg");
    Template::render(
        "publish",
        TuxRun {
            url: String::new().add("/publish/").add(project),
            editor: editor(),
            title: format!("Publish - {}", project),
            project: project.to_string(),
            message: msg,
            projects: projects(),
            log: debug,
        },
    )
}

#[get("/uninstall/<project>")]
fn uninstall_project(project: &str) -> Template {
    let msg: String =
        match Admin::new(directory(project).as_str()).run(vec!["uninstall".to_string()]) {
            true => format!("The project {} has been uninstalled successfully", project),
            false => format!("Failed to uninstall the {} project", project),
        };

    let debug = fs::read_to_string("logs.txt").expect("msg");
    Template::render(
        "uninstall",
        TuxRun {
            url: String::new().add("/uninstall/").add(project),
            editor: editor(),
            title: format!("Uninstall - {}", project),
            project: project.to_string(),
            message: msg,
            projects: projects(),
            log: debug,
        },
    )
}

#[get("/install/<project>")]
fn install_project(project: &str) -> Template {
    let msg: String = match Admin::new(directory(project).as_str()).run(vec![
        "install".to_string(),
        "--path".to_string(),
        ".".to_string(),
    ]) {
        true => format!("The project {} has been installed successfully", project),
        false => format!("Failed to install the {} project", project),
    };

    let debug = fs::read_to_string("logs.txt").expect("msg");
    Template::render(
        "install",
        TuxRun {
            url: String::new().add("/uninstall/").add(project),
            editor: editor(),
            title: format!("Install - {}", project),
            project: project.to_string(),
            message: msg,
            projects: projects(),
            log: debug,
        },
    )
}

#[get("/delete/<project>")]
fn delete_repo(project: &str) -> Redirect {
    fs::remove_dir_all(directory(project).as_str()).expect("failed to remove the directory");
    Redirect::to(uri!("/add"))
}

#[get("/yank/<project>/<version>")]
fn yank_repo(project: &str, version: &str) -> Template {
    let repo = String::new().add(project).add("@").add(version);
    let msg: String =
        match Admin::new(directory(project).as_str()).run(vec!["yank".to_string(), repo]) {
            true => format!(
                "The project has been yanked successfully at https://crates.io/creates/{}",
                project
            ),
            false => format!("Failed to yanked the {} project", project),
        };

    let debug = fs::read_to_string("logs.txt").expect("msg");
    Template::render(
        "yank",
        TuxRun {
            url: String::new()
                .add("/yank/")
                .add(project)
                .add("/")
                .add(version),
            editor: editor(),
            title: format!("Yanked - {}", project),
            project: project.to_string(),
            message: msg,
            projects: projects(),
            log: debug,
        },
    )
}

fn projects() -> HashMap<std::string::String, std::string::String> {
    let project_dir = std::env::var("TUX_DIR").expect("failed to find TUX_DIR variable path");
    let mut projects: HashMap<String, String> = HashMap::new();
    ScanDir::dirs()
        .read(project_dir, |iter| {
            for (entry, name) in iter {
                let p = entry.path().to_str().expect("").to_string().split_off(1);
                projects.insert(name, p);
            }
        })
        .unwrap();
    projects
}

#[get("/")]
fn index(flash: Option<FlashMessage>) -> Template {
    Template::render(
        "index",
        Tux {
            title: "Tux".to_string(),
            message: flash.map(|flash| flash.message().to_string()),
            projects: projects(),
            project: "".to_string(),
            url: "".to_string(),
            editor: std::env::var("TUX_EDITOR").expect("failed to get tux editor preferences"),
        },
    )
}

#[post("/open/<editor>/<project>")]
fn open(editor: &str, project: &str) -> Flash<Redirect> {
    Command::new(editor)
        .arg(directory(project).as_str())
        .spawn()
        .expect("failed to open editor");
    Flash::success(
        Redirect::to("/"),
        format!("The {} project or has been opened", project).as_str(),
    )
}

#[get("/add")]
fn add() -> Template {
    Template::render(
        "add",
        TuxRun {
            title: "Add a new project".to_string(),
            project: "".to_string(),
            message: format!(""),
            url: String::new().add("/add"),
            projects: projects(),
            editor: editor(),
            log: format!(""),
        },
    )
}

#[get("/add/<project>/<t>")]
fn add_project(project: &str, t: &str) -> Flash<Redirect> {
    match t {
        "Binary" => {
            Command::new("cargo")
                .arg("new")
                .arg("--bin")
                .arg(project)
                .current_dir(std::env::var("TUX_DIR").expect("failed to find tux dir"))
                .spawn()
                .expect("failed to create bin project");
            Flash::success(
                Redirect::to("/"),
                format!("The {} binary has been created successfully", project).as_str(),
            )
        }
        "Library" => {
            Command::new("cargo")
                .arg("new")
                .arg("--lib")
                .arg(project)
                .current_dir(std::env::var("TUX_DIR").expect("failed to find tux dir"))
                .spawn()
                .expect("failed to create bin project");
            Flash::success(
                Redirect::to("/"),
                format!("The {} library has been created successfully", project).as_str(),
            )
        }
        _ => Flash::error(Redirect::to("/add"), "Bad request"),
    }
}

#[get("/clone/<project>")]
fn clone_project(project: &str) -> Redirect {
    match Admin::new("").clone(project) {
        true => Redirect::to(format!("/manage/{}", project)),
        false => Redirect::to(uri!("/add")),
    }
}

#[get("/fail/<task>/<project>")]
fn fail_normal(task: &str, project: &str) -> Template {
    Template::render(
        "fail",
        TuxFailure {
            title: "Tux failed".to_string(),
            project: project.to_string(),
            errors: fs::read_to_string("./logs.txt").expect("failed to parse file"),
            url: String::new().add(task).add("/").add(project),
            editor: std::env::var("TUX_EDITOR").expect("failed to get tux editor preferences"),
        },
    )
}

fn timeline_project(project: &str) -> HashMap<String, String> {
    let connection =
        sqlite::open(format!("{}/{}.db", directory(project), project).as_str()).unwrap();
    let mut p: HashMap<String, String> = HashMap::new();
    let query = "SELECT * FROM timeline";
    let mut statement = connection.prepare(query).unwrap();

    while let Ok(State::Row) = statement.next() {
        p.insert(
            statement.read::<String, _>("description").unwrap(),
            statement.read::<String, _>("endline").unwrap(),
        );
    }
    p
}

#[get("/time/<project>")]
fn timeline(project: &str) -> Template {
    let connection =
        sqlite::open(format!("{}/{}.db", directory(project), project).as_str()).unwrap();
    let query = "CREATE TABLE IF NOT EXISTS timeline (description TEXT,endline TEXT);";
    connection.execute(query).unwrap();

    Template::render(
        "time",
        TuxTimeline {
            title: format!("Timeline for {} project", project),
            projects: timeline_project(project),
            project: project.to_string(),
        },
    )
}

#[get("/add-timeline/<project>/<description>/<end>")]
fn add_timeline(project: &str, description: &str, end: &str) -> Template {
    let connection =
        sqlite::open(format!("{}/{}.db", directory(project), project).as_str()).unwrap();
    let insert = format!(
        "INSERT INTO timeline VALUES ('{}', '{}');",
        description, end
    );
    connection.execute(insert.as_str()).unwrap();

    Template::render(
        "time",
        TuxTimeline {
            title: format!("Timeline for {} project", project),
            projects: timeline_project(project),
            project: project.to_string(),
        },
    )
}

fn editor() -> String {
    std::env::var("TUX_EDITOR").expect("failed to get tux editor preferences")
}

#[get("/manage/<project>")]
fn manage(project: &str) -> Template {
    Template::render(
        "manage",
        TuxManage {
            title: format!("Manage {}", project),
            project: project.to_string(),
            projects: projects(),
            editor: editor(),
            readme: readme(&directory(project)),
        },
    )
}

#[get("/web/assets/img/rust-mascot.png")]
async fn rust_img() -> NamedFile {
    NamedFile::open(Path::new("web/assets/img/rust-mascot.png"))
        .await
        .expect("msg")
}
fn readme(p: &str) -> String {
    markdown::file_to_html(Path::new(format!("{}/README.md", p).as_str())).expect("msg")
}
#[get("/assets/css/app.css")]
fn css() -> (ContentType, &'static str) {
    (
        ContentType::CSS,
        fs::read_to_string("web/assets/css/app.css")
            .expect("")
            .leak(),
    )
}

#[get("/assets/manifest.json")]
fn render_manifest_json() -> (ContentType, &'static str) {
    (
        ContentType::JSON,
        fs::read_to_string("web/assets/manifest.json")
            .expect("")
            .leak(),
    )
}

#[get("/assets/js/ji.js")]
fn js() -> (ContentType, &'static str) {
    (
        ContentType::JavaScript,
        fs::read_to_string("web/assets/js/ji.js").expect("").leak(),
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(static_resources_initializer!(
            "favicon" => "web/assets/favicon.ico",
            "favicon-png" => "web/assets/favicon-16x16.png",
            "/assets/manifest.json" => "web/assets/manifest.json",
        ))
        .attach(Template::fairing())
        .mount("/", routes![favicon, favicon_png, render_manifest_json])
        .mount(
            "/",
            routes![
                index,
                css,
                js,
                run_project,
                build_project,
                check_project,
                doc_project,
                test_project,
                bench_project,
                update_project,
                clippy_project,
                publish_project,
                uninstall_project,
                clean_project,
                delete_repo,
                yank_repo,
                fail_normal,
                open,
                print_file,
                add,
                add_project,
                timeline,
                add_timeline,
                install_project,
                clean_timeline,
                clone_project,
                rust_img,
                manage,
            ],
        )
}
