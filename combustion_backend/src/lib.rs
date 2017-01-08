//! Combustion backend
//!
//! This crate contains most graphics API specific code.
//!
//! Though most of it just focuses on OpenGL, support for Vulkan and DX11 are planned... eventually.
#![feature(test)]
#![allow(unused_imports, unknown_lints)]

extern crate test;

extern crate libc;
#[cfg(all(feature = "dx11", target_os = "windows"))]
extern crate winapi;

#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate lazy_static;
extern crate nalgebra;
extern crate image;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate glfw;
extern crate void;
extern crate fnv;

#[macro_use]
extern crate combustion_common as common;

pub mod window;

#[cfg(feature = "gl")]
pub mod gl;

#[cfg(feature = "vulkan")]
pub mod vulkan;

#[cfg(all(feature = "dx11", target_os = "windows"))]
pub mod dx11;

#[cfg(target_os = "windows")]
pub mod win32;

pub mod command;