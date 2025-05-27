use crate::{
    kreide, logging, overlay, server, subscribers, GAMEASSEMBLY_HANDLE, UNITYPLAYER_HANDLE,
};
use ctor::ctor;
use std::thread::{self};
use std::time::Duration;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;

#[ctor]
fn entry() {
    thread::spawn(|| unsafe {
        #[cfg(debug_assertions)]
        windows::Win32::System::Console::AllocConsole();
        logging::MultiLogger::init();
        while GetModuleHandleW(windows::core::w!("GameAssembly")).is_err()
            || GetModuleHandleW(windows::core::w!("UnityPlayer")).is_err()
        {
            thread::sleep(Duration::from_millis(10));
        }
        kreide::il2cpp::init(*GAMEASSEMBLY_HANDLE, *UNITYPLAYER_HANDLE);
        kreide::il2cpp::misc::unlock_fps();

        log::info!("Build: {}", env!("TARGET_BUILD"));
        log::info!("Setting up...");
        overlay::initialize().unwrap();
        subscribers::battle::subscribe().unwrap();
        log::info!("Finished setup.");
        server::start_server();
    });
}
