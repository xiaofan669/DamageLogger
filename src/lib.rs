#![allow(non_snake_case)]
#![recursion_limit = "256"]
#![feature(let_chains)]
#[macro_use]
extern crate rust_i18n;

mod battle;
mod entry;
mod kreide;
mod logging;
mod models;
mod overlay;
mod server;
mod subscribers;
mod ui;

use std::sync::LazyLock;
use windows::{core::w, Win32::System::LibraryLoader::GetModuleHandleW};

pub static GAMEASSEMBLY_HANDLE: LazyLock<usize> = LazyLock::new(|| unsafe {
    GetModuleHandleW(w!("GameAssembly"))
        .expect("GameAssembly was not found in the game process")
        .0 as usize
});

pub static UNITYPLAYER_HANDLE: LazyLock<usize> = LazyLock::new(|| unsafe {
    GetModuleHandleW(w!("UnityPlayer"))
        .expect("UnityPlayer was not found in the game process")
        .0 as usize
});

rust_i18n::i18n!();
