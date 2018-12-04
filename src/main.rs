#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world - TA DB"
}

#[get("/num/<x>")]
fn num(x:u64) -> String {
    format!("Die Zahl ist {}",x)
}

fn main() {
    rocket::ignite().mount("/", routes![index,num]).launch();
}
