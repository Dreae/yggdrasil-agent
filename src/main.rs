#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate serde_derive;
extern crate rocket;
extern crate rocket_contrib;

fn main() {
  rocket::ignite().mount("/api", routes![]).launch();
}