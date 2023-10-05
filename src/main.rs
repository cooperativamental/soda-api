#[macro_use]
extern crate rocket;
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
use bincode::deserialize;
pub struct CORS;

static DEFAULT_TEMPLATE: &'static [u8] = include_bytes!("anchor.soda");
static FLUTTER_TEMPLATE: &'static [u8] = include_bytes!("flutter.soda");
static REACT_NATIVE_TEMPLATE: &'static [u8] = include_bytes!("react_native.soda");
static SEAHORSE_TEMPLATE: &'static [u8] = include_bytes!("seahorse.soda");
static NEXTJS_TEMPLATE: &'static [u8] = include_bytes!("nextjs.soda");
static TEMPLATES_LIST: &'static [&[u8]; 5] = &[DEFAULT_TEMPLATE, FLUTTER_TEMPLATE, REACT_NATIVE_TEMPLATE, SEAHORSE_TEMPLATE, NEXTJS_TEMPLATE];
    

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
    let templates: Vec<TemplateMetadata> = TEMPLATES_LIST.map(|template|deserialize::<Template>(&template).unwrap().metadata).to_vec();
    json!({ "templates":templates  })
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
    let template = deserialize::<Template>(&TEMPLATES_LIST[template_id]).unwrap();
    let files = generate_project(template, &idl);
    json!({ "files": files })
}
