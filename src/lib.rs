macro_rules! lazy_initialize_address {
    ($addr:expr) => {
        LazyLock::new(|| unsafe { std::mem::transmute($addr + *$crate::GAMEASSEMBLY_HANDLE) })
    };
}
mod battle;
mod entry;
mod subscribers;
mod models;
mod server;
mod kreide;
mod ui;
mod logging;
mod overlay;

use std::sync::LazyLock;
use windows::{
    core::w,
    Win32::System::LibraryLoader::GetModuleHandleW,
};

pub static GAMEASSEMBLY_HANDLE: LazyLock<usize> =
    LazyLock::new(|| unsafe { GetModuleHandleW(w!("GameAssembly")).expect("GameAssembly was not found in the game process").0 as usize });