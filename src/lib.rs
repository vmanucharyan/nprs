#![feature(plugin)]
#![feature(slice_patterns)]
#![feature(inclusive_range_syntax)]
#![feature(deque_extras)]
#![cfg_attr(test, plugin(stainless))]
#![allow(dead_code)]

extern crate image as pd_image;

pub mod extract;
pub mod image;
pub mod structures;
pub mod ml;

#[cfg(test)]
mod test;
