#[macro_use]
extern crate rocket;

mod default_template;
use rocket::serde::{
    json::{serde_json::json, Json, Value},
    Deserialize,
};
use soda_sol::*;
use std::path;
use std::path::PathBuf;

const TEMPLATES: &'static [&'static str] = &[
    "default",
    "react_native_experimental",
    "seahorse_experimental",
];

#[get("/")]
fn index() -> Value {
   // read all the files in the template folder
    let mut templates = Vec::new();
    for template in TEMPLATES {
        let template = load_template(
            &path::Path::new(&format!("templates/{}.soda", template))
                .to_string_lossy(),
        )
        .unwrap();
        templates.push(template.metadata);
    }
    json!({ "templates": templates })
}

#[shuttle_runtime::main]
async fn rocket(
    #[shuttle_static_folder::StaticFolder(folder = "templates")] _static_folder: PathBuf,
) -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![index, new]);

    Ok(rocket.into())
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct GenerateReq {
    idl: IDL,
}

#[post(
    "/get_project_files/<template_id>",
    format = "json",
    data = "<generate_req>"
)]
fn new(template_id: usize, generate_req: Json<GenerateReq>) -> Value {
    if template_id >= TEMPLATES.len() {
        return json!({ "error": "Invalid template id" });
    } else {
        let GenerateReq { idl } = generate_req.into_inner();
        let template = load_template(
            &path::Path::new(&format!("templates/{}.soda", TEMPLATES[template_id]))
                .to_string_lossy(),
        )
        .unwrap();
        let files = generate_project(template, &idl);
        json!({ "files": files })
    }
}
