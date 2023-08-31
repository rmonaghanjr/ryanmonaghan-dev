#![feature(proc_macro_hygiene, decl_macro)]
#![recursion_limit = "2048"]

#[macro_use] extern crate rocket;

mod utilities;
mod posts;
mod latex;

use std::fs;

use utilities::{get_view, get_blog_page};
use rocket::response::content::RawHtml;
use rocket::fs::FileServer;
use typed_html::dom::DOMTree;
use typed_html::{html, text};
use chrono::{Utc, TimeZone, Datelike, Timelike};

#[get("/")]
fn index() -> RawHtml<String> {
    let contents = fs::read_to_string(get_view("index"))
        .expect("readable");
    return RawHtml(contents);
}

#[get("/recent-posts?<limit>")]
fn recent_posts(limit: usize) -> RawHtml<String> {
    let posts = posts::get_recent_posts(limit);

    let doc: DOMTree<String>;

    if posts.len() == 0 {
        doc = html!(<p>"No posts! Check back in a bit!"</p>);
    } else {
        doc = html!(
        <div class="post-list">
            {posts.into_iter().map(|post| {
                let date_time = Utc.timestamp_opt(post.date, 0).unwrap();
                let (is_pm, hour) = date_time.hour12();

                return html!(
                    <div class="post-listing">
                        <div class="post-left">
                            <span class="post-title">{ text!("{}", post.title) }</span>
                            <span class="post-date">{ text!("{}-{:02}-{:02} {}:{:02} {}",
                                date_time.year(),
                                date_time.month(),
                                date_time.day(),
                                hour,
                                date_time.minute(),
                                if is_pm { "PM" } else { "AM" }
                            )}</span>
                        </div>
                        <div class="post-middle">
                            <span class="post-description">{ text!("{}", post.description) }</span>
                        </div>
                        <div class="post-right">
                            <a href=post.link class="post-button">"Read"</a>
                        </div>
                    </div>)
                }
            )}
        </div>);
 
    }
    return RawHtml(doc.to_string());
}

#[get("/blog")]
fn all_posts() -> RawHtml<String> {
    let contents = fs::read_to_string(get_view("blog"))
        .expect("readable");
    return RawHtml(contents);
}

#[get("/blog/<post>")]
fn blog_post(post: String) -> RawHtml<String> {
    let contents = fs::read_to_string(get_blog_page(&post))
        .expect("readable");
    return RawHtml(contents);
}

#[get("/render_latex?<tex>")]
fn latex_renderer(tex: String) -> RawHtml<String> {
    let contents = latex::render_latex(&tex);
    return RawHtml(contents);
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, recent_posts, all_posts, blog_post, latex_renderer])
        .mount("/public", FileServer::from("./client/static"))
}
