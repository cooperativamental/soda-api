#[macro_use] extern crate rocket;

mod default_template;
use rocket::serde::{Deserialize, json::{Json, serde_json::json, Value}};
use soda_sol::*;
use default_template::default_template;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![index, new]);

    Ok(rocket.into())
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct GenerateReq {
    idl: IDL,
}

#[post("/<template_id>", format = "json", data = "<generate_req>")]
fn new(template_id: &str, generate_req: Json<GenerateReq>)-> Value  {
    let GenerateReq { idl } = generate_req.into_inner();
    println!("idl: {:?}", idl);
    println!("template_id: {:?}", template_id);
    let template = default_template();
    let files = generate_project(template, &idl);
    json!({ "files": files })
 }