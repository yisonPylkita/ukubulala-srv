#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn main_page() -> &'static str {
    "Main page"
}

// /get_killers?skin_color=black
#[get("/get_killers?<skin_color>")]
fn get_killers(skin_color: Option<String>) -> String {
    skin_color
        .map(|skin_color| format!("There were a few {} killers", skin_color))
        .unwrap_or_else(|| "No skin color provided".into())
}

// /get_victims?skin_color=white
#[get("/get_victims?<skin_color>")]
fn get_victims(skin_color: Option<String>) -> String {
    skin_color
        .map(|skin_color| format!("There were a few {} victims", skin_color))
        .unwrap_or_else(|| "No skin color provided".into())
}

fn main() {
    rocket::ignite()
        .mount("/", routes![main_page, get_killers, get_victims])
        .launch();
}
