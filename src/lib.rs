#![feature(plugin)]
#![feature(slice_patterns)]
#![cfg_attr(test, plugin(stainless))]

#![allow(dead_code)]

mod extract;
mod image;
mod structures;

#[cfg(test)]
mod test;
