#[cfg(unix)]
use libc::{dlopen, dlsym, RTLD_LAZY};

#[cfg(windows)]
use kernel32::{LoadLibraryA, GetProcAddress};

use std::ffi::CString;
use std::mem::transmute;

#[cfg(unix)]
pub fn load_plugin(name: &str) {
  unsafe {
    let module = dlopen(cstring!(name).as_ptr(), RTLD_LAZY);
    if module.is_null() {
      panic!("Failed to load plugin {}", name);
    }

    let loader = dlsym(module, cstring!(name).as_ptr());
    if loader.is_null() {
      panic!("{} is not a valid plugin, no load function found", name)
    }

    let loader_fn: extern "C" fn() -> () = transmute(loader);
    loader_fn();
  }
}

#[cfg(windows)]
pub fn load_plugin(name: &str) {
  unsafe {
    let module = LoadLibraryA(cstring!(name).as_ptr());
    if module.is_null() {
      panic!("Failed to load plugin {}", name);
    }

    let loader = GetProcAddress(module, cstring!("plugin_load").as_ptr());
    if loader.is_null() {
      panic!("{} is not a valid plugin, no load function found", name);
    }

    let loader_fn: extern "C" fn() -> () = transmute(loader);
    loader_fn();
  }
}