#![crate_name = "static_file"]
#![deprecated = "use https://github.com/iron/static instead"]
#![deny(missing_doc)]
#![feature(phase)]

//! Static file-serving handler.

#[phase(plugin)]
extern crate regex_macros;
extern crate regex;
extern crate time;

extern crate http;
extern crate iron;
#[phase(plugin, link)]
extern crate log;
extern crate mount;


pub use cache_handler::StaticWithCache;
pub use static_handler::Static;

mod cache_handler;
mod errors;
mod requested_path;
mod static_handler;
