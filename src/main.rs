#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod util;
mod types; 
mod actions;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello")]
fn myfn() -> &'static str {
    "you die"
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index, myfn])
    .launch();
}



