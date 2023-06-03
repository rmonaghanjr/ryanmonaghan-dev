#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::content::RawHtml;

#[get("/")]
fn index() -> RawHtml<String> {
    return RawHtml("<p>Hello, World!</p>".to_string());
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
}
