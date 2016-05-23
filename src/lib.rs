#![feature(plugin)]
#![feature(slice_patterns)]
#![feature(inclusive_range_syntax)]
#![cfg_attr(test, plugin(stainless))]
#![allow(dead_code)]

extern crate image as pd_image;
extern crate rustc_serialize;
extern crate bincode;
extern crate flate2;

pub mod extract;
pub mod image;
pub mod structures;
pub mod ml;

#[cfg(test)]
mod test;
