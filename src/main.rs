use std::collections::HashMap;
use std::fs;
use rocket::{get, launch, post, routes};
use rocket::http::ContentType;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::serde::{Deserialize, Serialize};
use rocket_dyn_templates::Template;
use rocket_include_static_resources::{static_resources_initializer, static_response_handler};
use scan_dir::ScanDir;
use crate::administrate::ji::Admin;

#[derive(Serialize, Deserialize)]
struct Ji
{
    title: String,
    message: Option<String>,
    projects: HashMap<String, String>,
}

mod administrate;

static_response_handler! {
    "/assets/favicon.ico" => favicon => "favicon",
    "/assets/favicon-16x16.png" => favicon_png => "favicon",
    "/assets/manifest.json" => favicon_json => "favicon",
}
fn directory(x: &str) -> String
{
    format!("{}/{}", std::env::var("JI_DIR").expect(""), x)
}

#[post("/build/<project>")]
fn build_project(project: &str) -> Flash<Redirect>
{
    Admin::new(directory(project).as_str()).run(vec!["build".to_string()]);
    Flash::success(Redirect::to("/"), format!("{} has been built successfully", project))
}

#[post("/check/<project>")]
fn check_project(project: &str) -> Flash<Redirect>
{
    Admin::new(directory(project).as_str()).run(vec!["check".to_string()]);
    Flash::success(Redirect::to("/"), format!("{} has been checked successfully", project))
}

#[post("/clean/<project>")]
fn clean_project(project: &str) -> Flash<Redirect>
{
    Admin::new(directory(project).as_str()).run(vec!["clean".to_string()]);
    Flash::success(Redirect::to("/"), format!("{} has been cleaned successfully", project))
}

#[post("/doc/<project>")]
fn doc_project(project: &str) -> Flash<Redirect>
{
    Admin::new(directory(project).as_str()).run(vec!["doc".to_string()]);
    Flash::success(Redirect::to("/"), format!("{} has been documented successfully", project))
}


#[post("/run/<project>")]
fn run_project(project: &str) -> Flash<Redirect>
{
    Admin::new(directory(project).as_str()).run(vec!["run".to_string()]);
    Flash::success(Redirect::to("/"), format!("{} has been launched successfully", project))
}

#[post("/test/<project>")]
fn test_project(project: &str) -> Flash<Redirect>
{
    Admin::new(directory(project).as_str()).run(vec!["test".to_string()]);
    Flash::success(Redirect::to("/"), format!("{} tests has been passed successfully", project))
}

#[post("/bench/<project>")]
fn bench_project(project: &str) -> Flash<Redirect>
{
    Admin::new(directory(project).as_str()).run(vec!["bench".to_string()]);
    Flash::success(Redirect::to("/"), format!("{} benchmark launched successfully", project))
}

#[post("/update/<project>")]
fn update_project(project: &str) -> Flash<Redirect>
{
    Admin::new(directory(project).as_str()).run(vec!["update".to_string()]);
    Flash::success(Redirect::to("/"), format!("{} has been updated successfully", project))
}


#[post("/clippy/<project>")]
fn clippy_project(project: &str) -> Flash<Redirect>
{
    Admin::new(directory(project).as_str()).run(vec!["clippy".to_string()]);
    Flash::success(Redirect::to("/"), format!("{} clippy no detect error", project))
}

#[post("/publish/<project>")]
fn publish_project(project: &str) -> Flash<Redirect>
{
    Admin::new(directory(project).as_str()).run(vec!["publish".to_string()]);
    Flash::success(Redirect::to("/"), format!("{} published no detect error", project))
}

#[post("/install/<project>")]
fn install_project(project: &str) -> Flash<Redirect>
{
    Admin::new(format!("{}/.cargo/bin", env!("HOME")).as_str()).run(vec!["install".to_string(), project.to_string()]);
    Flash::success(Redirect::to("/"), format!("{} installed successfully", project))
}

#[post("/uninstall/<project>")]
fn uninstall_project(project: &str) -> Flash<Redirect>
{
    Admin::new(format!("{}/.cargo/bin", env!("HOME")).as_str()).run(vec!["uninstall".to_string(), project.to_string()]);
    Flash::success(Redirect::to("/"), format!("{} uninstalled successfully", project))

}

#[get("/")]
fn index(flash: Option<FlashMessage>) -> Template {
    let project_dir = std::env::var("JI_DIR").expect("failed to find JI_DIR variable path");
    let mut projects: HashMap<String, String> = HashMap::new();
    ScanDir::dirs().read(project_dir, |iter| {
        for (entry, name) in iter {
            let p = entry.path().to_str().expect("").to_string().split_off(1);
            projects.insert(name, p);
        }
    }).unwrap();

    Template::render("index", Ji {
        title: "Ji".to_string(),
        message: flash.map(|flash| flash.message().to_string()),
        projects,
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
        )).attach(Template::fairing()).mount("/", routes![favicon, favicon_png,favicon_json]).mount("/", routes![index,css,js,build_project,check_project,doc_project,run_project,test_project,bench_project,update_project,clippy_project,publish_project,install_project,uninstall_project,clean_project])
}