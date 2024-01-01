use crate::administrate::ji::Admin;
use cargo_metadata::camino::Utf8PathBuf;
use cargo_metadata::{CargoOpt, Metadata, MetadataCommand};
use rocket::http::ContentType;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, launch, routes, uri};
use rocket_dyn_templates::Template;
use rocket_include_static_resources::{static_resources_initializer, static_response_handler};
use scan_dir::ScanDir;
use sqlite::State;
use std::collections::HashMap;
use std::fs::{self};
use std::ops::Add;
use std::path::{Path, PathBuf};
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
    message: Option<String>,
    projects: HashMap<String, String>,
    dependencies: Vec<String>,
    authors: Vec<String>,
    repository: String,
    license: String,
    log: String,
    crates: String,
    description: String,
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
    format!("{}/{}", std::env::var("CRATES_DIR").expect(""), x)
}

#[get("/build/<project>")]
fn build_project(project: &str) -> Flash<Redirect> {
    let msg: String = match Admin::new(directory(project).as_str()).run(vec!["build".to_string()]) {
        true => format!("The project {} has been built successfully", project),
        false => format!("Failed to build the {} project", project),
    };
    Flash::success(Redirect::to(format!("/manage/{}", project)), msg)
}

#[get("/check/<project>")]
fn check_project(project: &str) -> Flash<Redirect> {
    let msg: String = match Admin::new(directory(project).as_str()).run(vec!["check".to_string()]) {
        true => format!("The project {} has been checked successfully", project),
        false => format!("Failed to check the {} project", project),
    };

    Flash::success(Redirect::to(format!("/manage/{}", project)), msg)
}

#[get("/clean/<project>")]
fn clean_project(project: &str) -> Flash<Redirect> {
    let msg: String = match Admin::new(directory(project).as_str()).run(vec!["clean".to_string()]) {
        true => format!("The project {} has been cleaned successfully", project),
        false => format!("Failed to clean the {} project", project),
    };

    Flash::success(Redirect::to(format!("/manage/{}", project)), msg)
}

#[get("/doc/<project>")]
fn doc_project(project: &str) -> Flash<Redirect> {
    let msg: String = match Admin::new(directory(project).as_str()).run(vec!["doc".to_string()]) {
        true => format!("The project {} has been documented successfully", project),
        false => format!("Failed to build documentation for the {} project", project),
    };

    Flash::success(Redirect::to(format!("/manage/{}", project)), msg)
}

#[get("/bench/<project>")]
fn bench_project(project: &str) -> Flash<Redirect> {
    let msg: String = match Admin::new(directory(project).as_str()).run(vec!["bench".to_string()]) {
        true => format!("The bench run successfully for {} project", project),
        false => format!("Failed to run bench    for the {} project", project),
    };

    Flash::success(Redirect::to(format!("/manage/{}", project)), msg)
}

#[get("/clippy/<project>")]
fn clippy_project(project: &str) -> Flash<Redirect> {
    let msg: String = match Admin::new(directory(project).as_str()).run(vec!["clippy".to_string()])
    {
        true => format!("The project {} has been checked successfully", project),
        false => format!("Failure founded for the {} project", project),
    };

    Flash::success(Redirect::to(format!("/manage/{}", project)), msg)
}

#[get("/audit/<project>")]
fn audit_project(project: &str) -> Flash<Redirect> {
    let msg: String = match Admin::new(directory(project).as_str()).run(vec![
        "audit".to_string(),
        "--color".to_string(),
        "never".to_string(),
    ]) {
        true => format!("No vulnerabilities founded for the {} project", project),
        false => format!("vulnerabilities found for the {} project", project),
    };

    Flash::success(Redirect::to(format!("/manage/{}", project)), msg)
}

#[get("/run/<project>")]
fn run_project(project: &str) -> Flash<Redirect> {
    let msg: String = match Admin::new(directory(project).as_str()).run(vec!["run".to_string()]) {
        true => format!("The project {} has run successfully", project),
        false => format!("Failed to run the {} project", project),
    };

    Flash::success(Redirect::to(format!("/manage/{}", project)), msg)
}

#[get("/test/<project>")]
fn test_project(project: &str) -> Flash<Redirect> {
    let msg: String = match Admin::new(directory(project).as_str()).run(vec!["test".to_string()]) {
        true => format!("Test passes for the {} project", project),
        false => format!("Failure for the {} project", project),
    };
    Flash::success(Redirect::to(format!("/manage/{}", project)), msg)
}

#[get("/update/<project>")]
fn update_project(project: &str) -> Flash<Redirect> {
    let msg: String = match Admin::new(directory(project).as_str()).run(vec!["update".to_string()])
    {
        true => format!("The project {} has been updated successfully", project),
        false => format!("Failed to update the {} project", project),
    };

    Flash::success(Redirect::to(format!("/manage/{}", project)), msg)
}

#[get("/publish/<project>")]
fn publish_project(project: &str) -> Flash<Redirect> {
    let msg: String = match Admin::new(directory(project).as_str()).run(vec!["publish".to_string()])
    {
        true => format!(
            "The project has been published successfully at https://crates.io/creates/{}",
            project
        ),
        false => format!("Failed to publish the {} project", project),
    };

    Flash::success(Redirect::to(format!("/manage/{}", project)), msg)
}

#[get("/uninstall/<project>")]
fn uninstall_project(project: &str) -> Flash<Redirect> {
    let msg: String =
        match Admin::new(directory(project).as_str()).run(vec!["uninstall".to_string()]) {
            true => format!("The project {} has been uninstalled successfully", project),
            false => format!("Failed to uninstall the {} project", project),
        };

    Flash::success(Redirect::to(format!("/manage/{}", project)), msg)
}

#[get("/install/<project>")]
fn install_project(project: &str) -> Flash<Redirect> {
    let msg: String = match Admin::new(directory(project).as_str()).run(vec![
        "install".to_string(),
        "--path".to_string(),
        ".".to_string(),
    ]) {
        true => format!("The project {} has been installed successfully", project),
        false => format!("Failed to install the {} project", project),
    };

    Flash::success(Redirect::to(format!("/manage/{}", project)), msg)
}

#[get("/delete/<project>")]
fn delete_repo(project: &str) -> Flash<Redirect> {
    if Path::new(&directory(project)).is_dir() {
        fs::remove_dir_all(directory(project).as_str()).expect("failed to remove the directory");
        return Flash::success(
            Redirect::to(uri!("/")),
            format!("{} has been deleted successfully", project),
        );
    }
    return Flash::success(Redirect::to(uri!("/")), format!("{} not exist", project));
}

#[get("/yank/<project>/<version>")]
fn yank_repo(project: &str, version: &str) -> Flash<Redirect> {
    let repo = String::new().add(project).add("@").add(version);
    match Admin::new(directory(project).as_str()).run(vec!["yank".to_string(), repo]) {
        true => Flash::success(
            Redirect::to(format!("/manage/{}", project)),
            format!(
                "The project has been yanked successfully at https://crates.io/creates/{}",
                project
            ),
        ),
        false => Flash::error(
            Redirect::to(format!("/manage/{}", project)),
            format!("The project yank task has failed for {} project", project),
        ),
    }
}

fn projects(project: &str) -> HashMap<std::string::String, std::string::String> {
    let project_dir = std::env::var("CRATES_DIR").expect("failed to find CRATES_DIR variable path");
    let mut projects: HashMap<String, String> = HashMap::new();
    if project.is_empty() {
        let project_dir =
            std::env::var("CRATES_DIR").expect("failed to find CRATES_DIR variable path");
        let mut projects: HashMap<String, String> = HashMap::new();
        ScanDir::dirs()
            .read(project_dir, |iter| {
                for (entry, name) in iter {
                    let p = entry.path().to_str().expect("").to_string().split_off(1);
                    projects.insert(name, p);
                }
            })
            .unwrap();
        return projects;
    }

    ScanDir::dirs()
        .read(project_dir, |iter| {
            for (entry, name) in iter {
                let p = entry.path().to_str().expect("").to_string().split_off(1);
                if !p.contains(project) {
                    projects.insert(name, p);
                }
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
            projects: projects(""),
            project: "".to_string(),
            url: "".to_string(),
            editor: editor(),
        },
    )
}

#[get("/open/<project>")]
fn open(project: &str) -> Redirect {
    Command::new(editor().as_str())
        .arg(directory(project).as_str())
        .spawn()
        .expect("failed to open editor");
    Redirect::to(format!("/manage/{}", project))
}

#[get("/add")]
fn add() -> Template {
    Template::render(
        "add",
        TuxRun {
            title: "Add a new project".to_string(),
            project: "".to_string(),
            message: String::new(),
            url: String::new().add("/add"),
            projects: projects(""),
            editor: editor(),
            log: String::new(),
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
            editor: editor(),
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
    std::env::var("CRATES_EDITOR").expect("failed to get tux editor preferences")
}

#[get("/manage/<project>")]
fn manage(project: &str, flash: Option<FlashMessage>) -> Template {
    let m = metadata(project).unwrap();
    let mut deps: Vec<String> = Vec::new();
    let d = m.root_package().expect("msg").dependencies.clone();
    let c: String = m.root_package().expect("msg").name.to_string();
    let mut crates = String::new();

    if c.contains('-') {
        let parts: Vec<&str> = c.split('-').collect();
        for &p in parts.iter() {
            crates.push(' ');
            crates.push_str(p);
        }
    } else if c.contains('_') {
        let parts: Vec<&str> = c.split('_').collect();
        for &p in parts.iter() {
            crates.push(' ');
            crates.push_str(p);
        }
    } else {
        crates.push_str(c.as_str());
    }

    if !Path::new("logs.txt").is_file() {
        fs::File::create("logs.txt").expect("failed to create logs file");
    }

    if !Path::new("output.txt").is_file() {
        fs::File::create("output.txt").expect("failed to create output file");
    }

    let mut lg: String = fs::read_to_string("logs.txt").expect("failed to parse log");
    lg.push_str("\n");

    lg.push_str(fs::read_to_string("output.txt").expect("msg").as_str());

    for x in d.iter() {
        deps.push(format!(
            "<a href=\"https://crates.io/crates/{}\">{}</a>",
            x.clone().name,
            x.clone().name,
        ));
    }
    let l: Utf8PathBuf = m
        .root_package()
        .expect("msg")
        .clone()
        .license_file
        .expect("msg");
    let msg = match flash {
        Some(x) => x.message().to_string(),
        None => "".to_string(),
    };

    Template::render(
        "manage",
        TuxManage {
            title: format!("Manage {}", project),
            project: project.to_string(),
            projects: projects(project),
            editor: editor(),
            readme: readme(m.root_package().expect("msg").readme().expect("").as_str()),
            dependencies: deps,
            authors: m.root_package().expect("msg").authors.clone(),
            repository: m
                .root_package()
                .expect("msg")
                .clone()
                .repository
                .expect("msg")
                .to_string(),
            license: fs::read_to_string(l).expect("msg"),
            message: Some(msg),
            log: lg,
            crates,
            description: m
                .root_package()
                .expect("msg")
                .clone()
                .description
                .expect("msg")
                .to_string(),
        },
    )
}

#[get("/clean-logs/<project>")]
fn clean_logs(project: &str) -> Flash<Redirect> {
    fs::remove_file("logs.txt").expect("no log file present");
    fs::File::create("logs.txt").expect("failed to recreate the file");
    Flash::success(
        Redirect::to(format!("/manage/{}", project)),
        "Log is now empty",
    )
}

fn readme(p: &str) -> String {
    let c = fs::read_to_string(p).expect("Fail to find file");
    if p.contains("md") {
        return rustmark::parse(c.as_str(), false).2.to_string();
    }
    c
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

fn metadata(project: &str) -> Result<Metadata, cargo_metadata::Error> {
    MetadataCommand::new()
        .manifest_path("./Cargo.toml")
        .current_dir(PathBuf::from(directory(project).as_str()))
        .features(CargoOpt::AllFeatures)
        .exec()
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
                add,
                add_project,
                timeline,
                add_timeline,
                install_project,
                audit_project,
                clone_project,
                manage,
                clean_logs,
            ],
        )
}
