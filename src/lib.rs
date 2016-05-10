#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

#![allow(dead_code)]

mod extract;
mod image;
mod structures;

#[cfg(test)]
mod test;
