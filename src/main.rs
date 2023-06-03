#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::fs;
use rocket::response::content::RawHtml;
use rocket::fs::FileServer;

#[get("/")]
fn index() -> RawHtml<String> {
    let contents = fs::read_to_string("./html/views/index.html")
        .expect("readable");
    return RawHtml(contents);
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/public", FileServer::from("./static"))
}
