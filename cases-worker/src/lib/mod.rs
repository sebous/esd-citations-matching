#![allow(dead_code)]

pub mod benchmark;
pub mod db;
mod document;
mod error;
pub mod logger;
pub mod regex;
pub mod util;

pub use document::Document;
pub use error::Error;
