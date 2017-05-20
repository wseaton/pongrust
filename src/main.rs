#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use rocket_contrib::Template;


#[derive(Serialize)]
struct Nothing { 
    x: i32,
    y: i32,
 }


#[get("/")]
fn index() -> Template {
    let mut nothing = Nothing { x: 0, y: 0 };

    // modifying some values for fun
    nothing.x = 1;
    nothing.y = 2;
    
    Template::render("index", &nothing)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}