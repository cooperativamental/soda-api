#[macro_use]
extern crate rocket;
mod default_template;
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    serde::{
        json::{serde_json::json, Json, Value},
        Deserialize,
    },
    Request, Response,
};
use soda_sol::*;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().attach(CORS).mount("/", routes![index, templates, get_project_files]);
    Ok(rocket.into())
}

#[get("/templates")]
fn templates() -> Value {
    json!({ "templates":[ default_template::default_template().metadata]  })
}

#[get("/")]
fn index() -> &'static str {
    r#" USAGE:
        GET https://soda.shuttleapp.rs/templates returns a list of templates
        POST https://soda.shuttleapp.rs/get_project_files/<template_id> generates a project from a template, been the template_id the index of the template in the list returned by the GET request
    "#
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct GenerateReq {
    idl: IDL,
}

#[post(
    "/get_project_files/<template_id>",
    data = "<generate_req>"
)]
fn get_project_files(template_id: usize, generate_req: Json<GenerateReq>) -> Value {
    let GenerateReq { idl } = generate_req.into_inner();
    let template = default_template::default_template();
    let files = generate_project(template, &idl);
    json!({ "files": files })
}
