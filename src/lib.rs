#![feature(plugin)]
#![feature(slice_patterns)]
#![cfg_attr(test, plugin(stainless))]

#![allow(dead_code)]

extern crate image as pd_image;

pub mod extract;
pub mod image;
pub mod structures;

#[cfg(test)]
mod test;
