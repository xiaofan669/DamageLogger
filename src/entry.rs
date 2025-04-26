use crate::{logging, overlay, subscribers};
use ctor::ctor;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use std::thread::{self};
use std::time::Duration;

#[ctor]
fn entry() {

    thread::spawn(|| unsafe {
        #[cfg(debug_assertions)]
        windows::Win32::System::Console::AllocConsole();    
        logging::MultiLogger::init();
        while GetModuleHandleW(windows::core::w!("GameAssembly")).is_err() ||
            GetModuleHandleW(windows::core::w!("UnityPlayer")).is_err() {
            thread::sleep(Duration::from_millis(10));
        }

        log::info!("Setting up...");
        overlay::initialize();
        subscribers::battle::subscribe().unwrap();
        log::info!("Finished setup.");
    });

    thread::spawn(|| {
        crate::server::start_server();
    });
}