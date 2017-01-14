#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate yggdrasil_shared;
extern crate rocket;
extern crate rocket_contrib;

#[cfg(unix)]
extern crate libc;

#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate kernel32;

mod loader;

fn main() {
  loader::plugin_loader::load_plugin("test.dll");
}