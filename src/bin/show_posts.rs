// #![feature(proc_macro_hygiene, decl_macro)]

// #[macro_use]
// extern crate rocket;

// #[get("/")]
// fn main_page() -> &'static str {
//     "Main page"
// }

// /get_killers?skin_color=black
// #[get("/get_killers?<skin_color>")]
// fn get_killers(skin_color: Option<String>) -> String {
//     skin_color
//         .map(|skin_color| format!("There were a few {} killers", skin_color))
//         .unwrap_or_else(|| "No skin color provided".into())
// }

// /get_victims?skin_color=white&page=1
// #[get("/get_victims?<skin_color>")]
// fn get_victims(skin_color: Option<String>) -> String {
//     skin_color
//         .map(|skin_color| format!("There were a few {} victims", skin_color))
//         .unwrap_or_else(|| "No skin color provided".into())
// }


extern crate ukubulala_srv;
extern crate diesel;

use std::{thread, time};
use self::ukubulala_srv::*;
use self::models::*;
use self::diesel::prelude::*;
use dotenv::dotenv;
use std::env;

fn main() {
    use ukubulala_srv::schema::homicides::dsl::*;

    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let connection = loop {
        match PgConnection::establish(&database_url) {
            Ok(conn) => break conn,
            Err(_) => {
                println!("Let's try once again, comrade");
                thread::sleep(time::Duration::from_secs(20));
            }
        }
    };
    
    let results = homicides.filter(record_id.gt(10))
        .limit(100)
        .load::<Homicide>(&connection)
        .expect("Error loading homicides");

    println!("Displaying {} posts", results.len());
    for homicide in results {
        println!("{}", homicide.city.unwrap());
        println!("----------\n");
        println!("{}", homicide.weapon.unwrap());
    }
}

// fn main() {
//     rocket::ignite()
//         .mount("/", routes![main_page, get_killers, get_victims])
//         .launch();
// }
