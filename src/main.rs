#![feature(proc_macro_hygiene, decl_macro)]
#![recursion_limit = "2048"]

#[macro_use] extern crate rocket;

mod utilities;

use utilities::get_view;
use std::fs;
use rocket::response::content::RawHtml;
use rocket::fs::FileServer;

use typed_html::dom::DOMTree;
use typed_html::html;

#[get("/")]
fn index() -> RawHtml<String> {
    let contents = fs::read_to_string(get_view("index"))
        .expect("readable");
    return RawHtml(contents);
}

#[get("/recent-posts?<limit>")]
fn recent_posts(limit: usize) -> RawHtml<String> {
    let doc: DOMTree<String> = html!(
    <div class="post-list">
        {(1..=limit).map(|i| html!(
        <div class="post-listing">
            <div class="post-left">
            <span class="post-title">"Hello World"</span>
            <span class="post-date">"2021-01-01"</span>
            </div>
            <div class="post-middle">
            <span class="post-description">"This is my first blog post. I'm not sure what I'm going to write about yet, but I'm sure I'll figure it out."</span>
            </div>
            <div class="post-right">
            <button class="post-button">"Read More"</button>
            </div>
        </div>) 
    )}
    </div>);

    return RawHtml(doc.to_string());
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, recent_posts])
        .mount("/public", FileServer::from("./client/static"))
}
