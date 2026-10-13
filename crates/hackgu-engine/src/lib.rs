//! Automatically decompiled CyberConnect2 .hack//G.U. Engine Classes in Pure Rust.
//! Generated from binary RTTI, VTable, and iced-x86 disassembly.

#![allow(dead_code, non_camel_case_types, non_snake_case)]

pub trait CcClass {
    fn class_name(&self) -> &'static str;
    fn vtable_rva(&self) -> u32;
    fn virtual_method_count(&self) -> usize;
    fn base_classes(&self) -> &'static [&'static str];
}

pub mod battle;
pub mod core;
pub mod event;
pub mod geom;
pub mod gobj;
pub mod ui;
