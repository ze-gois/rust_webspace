#![no_std]

extern crate alloc;

#[cfg(target_arch = "wasm32")]
pub mod canvas;
#[cfg(target_arch = "wasm32")]
pub mod dom;
pub mod html;

pub fn ok() {}
