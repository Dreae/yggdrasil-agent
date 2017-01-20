#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate yggdrasil_shared;
#[macro_use] extern crate lazy_static;
extern crate serde;
extern crate serde_json;
extern crate rocket;
extern crate rocket_contrib;
extern crate dotenv;

#[cfg(unix)]
extern crate libc;

#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate kernel32;

use dotenv::dotenv;

mod loader;
mod api;

fn main() {
  dotenv().ok();

  rocket::ignite().mount("/api/encrypted", routes![
    api::encrypted::get_server_info
  ]).launch();
}