use anyhow::Result;
use egui_directx11::{app::EguiDx11, input_manager::InputResult};
use retour::static_detour;
use std::{
    cell::OnceCell,
    ffi::c_void,
    mem::{self},
    ptr::null_mut,
    sync::Once,
};
use windows::{
    core::{w, Interface, HRESULT},
    Win32::{
        Foundation::{HMODULE, HWND, LPARAM, LRESULT, WPARAM},
        Graphics::{
            Direct3D::{D3D_DRIVER_TYPE_HARDWARE, D3D_FEATURE_LEVEL_10_1, D3D_FEATURE_LEVEL_11_0},
            Direct3D11::{
                D3D11CreateDeviceAndSwapChain, ID3D11Device, ID3D11DeviceContext,
                D3D11_CREATE_DEVICE_FLAG, D3D11_SDK_VERSION,
            },
            Dxgi::{
                Common::{
                    DXGI_FORMAT, DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_MODE_DESC,
                    DXGI_MODE_SCALING_UNSPECIFIED, DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
                    DXGI_RATIONAL, DXGI_SAMPLE_DESC,
                },
                IDXGISwapChain, IDXGISwapChain_Vtbl, DXGI_PRESENT, DXGI_SWAP_CHAIN_DESC,
                DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH, DXGI_SWAP_EFFECT_DISCARD,
                DXGI_USAGE_RENDER_TARGET_OUTPUT,
            },
        },
        UI::{
            Input::KeyboardAndMouse::VK_MENU,
            WindowsAndMessaging::{
                CallWindowProcW, CreateWindowExW, DefWindowProcW, DestroyWindow, RegisterClassExW,
                SetWindowLongPtrW, UnregisterClassW, CS_HREDRAW, CS_VREDRAW, GWLP_WNDPROC,
                WINDOW_EX_STYLE, WM_KEYDOWN, WNDCLASSEXW, WNDPROC, WS_OVERLAPPEDWINDOW,
            },
        },
    },
};

use crate::ui::app::{self, AppState};

// rdbo Kiero
// https://github.com/eugen15/directx-present-hook

static_detour! {
    pub static Present_Detour: unsafe extern "stdcall" fn(*const IDXGISwapChain_Vtbl, u32, DXGI_PRESENT) -> HRESULT;
    pub static Resize_Buffers_Detour: fn(
        *const IDXGISwapChain_Vtbl,
        u32,
        u32,
        u32,
        DXGI_FORMAT,
        u32
    ) -> HRESULT;

}
// This can be done in shorter calls
// Should we tho?
pub fn get_vtable() -> Box<[usize; 205]> {
    // Initializes a dummy swapchain to get the vtable
    unsafe {
        let window_class = WNDCLASSEXW {
            cbSize: mem::size_of::<WNDCLASSEXW>() as _,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(mem::transmute(DefWindowProcW as *const c_void)),
            lpszClassName: w!("veritas"),
            ..Default::default()
        };

        RegisterClassExW(&window_class);

        let window = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            window_class.lpszClassName,
            w!("veritas DirectX Window"),
            WS_OVERLAPPEDWINDOW,
            0,
            0,
            100,
            100,
            None,
            None,
            Some(window_class.hInstance),
            None,
        )
        .unwrap();

        let mut feature_levels = [D3D_FEATURE_LEVEL_10_1, D3D_FEATURE_LEVEL_11_0];

        let refresh_rate = DXGI_RATIONAL {
            Numerator: 60,
            Denominator: 1,
        };

        let buffer_desc = DXGI_MODE_DESC {
            Width: 100,
            Height: 100,
            RefreshRate: refresh_rate,
            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
            ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
            Scaling: DXGI_MODE_SCALING_UNSPECIFIED,
        };

        let sample_desc = DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        };

        let swap_chain_desc = DXGI_SWAP_CHAIN_DESC {
            BufferDesc: buffer_desc,
            SampleDesc: sample_desc,
            BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
            BufferCount: 1,
            OutputWindow: window,
            Windowed: true.into(),
            SwapEffect: DXGI_SWAP_EFFECT_DISCARD,
            Flags: DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH.0 as u32,
        };

        let mut swap_chain: Option<IDXGISwapChain> = None;
        let mut device: Option<ID3D11Device> = None;
        let mut context: Option<ID3D11DeviceContext> = None;

        D3D11CreateDeviceAndSwapChain(
            None,
            D3D_DRIVER_TYPE_HARDWARE,
            HMODULE(null_mut()),
            D3D11_CREATE_DEVICE_FLAG(0),
            Some(&feature_levels.clone()),
            D3D11_SDK_VERSION,
            Some(&swap_chain_desc),
            Some(&mut swap_chain),
            Some(&mut device),
            Some(feature_levels.as_mut_ptr()),
            Some(&mut context),
        )
        .unwrap();

        let mut vtable = Box::new([0usize; 205]);

        let swap_chain_ptr = &swap_chain.unwrap();
        let swap_chain_vtable = Interface::vtable(swap_chain_ptr);

        let device_ptr = &device.unwrap();
        let device_vtable = Interface::vtable(device_ptr);

        let context_ptr = &context.unwrap();
        let context_vtable = Interface::vtable(context_ptr);

        std::ptr::copy_nonoverlapping(mem::transmute(swap_chain_vtable), vtable.as_mut_ptr(), 18);

        std::ptr::copy_nonoverlapping(
            mem::transmute(&device_vtable),
            vtable[18..].as_mut_ptr(),
            43,
        );

        std::ptr::copy_nonoverlapping(
            mem::transmute(context_vtable),
            vtable[18 + 43..].as_mut_ptr(),
            144,
        );

        DestroyWindow(window).unwrap();
        UnregisterClassW(window_class.lpszClassName, Some(window_class.hInstance)).unwrap();

        vtable
    }
}

static mut APP: OnceCell<EguiDx11<AppState>> = OnceCell::new();
static mut OLD_WND_PROC: OnceCell<WNDPROC> = OnceCell::new();

pub fn present(
    swap_chain_vtbl: *const IDXGISwapChain_Vtbl,
    sync_interval: u32,
    flags: DXGI_PRESENT,
) -> HRESULT {
    unsafe {
        static INIT: Once = Once::new();
        INIT.call_once(|| {
            let state = AppState::default();
            let mut app =
                EguiDx11::init_with_state(mem::transmute(&(swap_chain_vtbl)), app::ui, state);

            // Example
            app.ui_state.set_keybind(egui::Key::M, Some(egui::Modifiers {
                ctrl: true,
                ..Default::default()
            }));

            OLD_WND_PROC
                .set(mem::transmute(SetWindowLongPtrW(
                    app.hwnd,
                    GWLP_WNDPROC,
                    hk_wnd_proc as usize as _,
                )))
                .unwrap();
            let _ = APP.set(app);
        });

        APP.get_mut()
            .unwrap()
            .present(mem::transmute(&(swap_chain_vtbl)));
        Present_Detour.call(swap_chain_vtbl, sync_interval, flags)
    }
}

pub fn resize_buffers(
    swap_chain_vtbl: *const IDXGISwapChain_Vtbl,
    buffer_count: u32,
    width: u32,
    height: u32,
    new_format: DXGI_FORMAT,
    swap_chain_flags: u32,
) -> HRESULT {
    unsafe {
        let resize_buffers = || {
            Resize_Buffers_Detour.call(
                swap_chain_vtbl,
                buffer_count,
                width,
                height,
                new_format,
                swap_chain_flags,
            )
        };
        if let Some(app) = APP.get_mut() {
            app.resize_buffers(mem::transmute(&(swap_chain_vtbl)), resize_buffers)
        } else {
            resize_buffers()
        }
    }
}

unsafe extern "stdcall" fn hk_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let app = APP.get_mut().unwrap();
    let input = app.wnd_proc(msg, wparam, lparam);
    // Has some issues like blocking closing the process window
    // Handle keybinding
    if let Some(keybind) = &app.ui_state.keybind {
        match input {
            InputResult::Key => {
                for e in &app.input_manager.events {
                    match e {
                        egui::Event::Key {
                            key,
                            physical_key: _,
                            pressed,
                            repeat: _,
                            modifiers,
                        } => {
                            // Add modifiers as well
                            if *key == keybind.key && *pressed  {
                                if let Some(keybind_modifiers) = keybind.modifiers {
                                    if modifiers.matches_exact(keybind_modifiers) {
                                        app.ui_state.show_menu = !app.ui_state.show_menu;

                                        // We simulate alt to get cursor
                                        return CallWindowProcW(
                                            *OLD_WND_PROC.get().unwrap(),
                                            hwnd,
                                            WM_KEYDOWN,
                                            WPARAM(VK_MENU.0 as _),
                                            LPARAM(0),
                                        );        
                                    }
                                }
                                else {
                                    app.ui_state.show_menu = !app.ui_state.show_menu;

                                    // We simulate alt to get cursor
                                    return CallWindowProcW(
                                        *OLD_WND_PROC.get().unwrap(),
                                        hwnd,
                                        WM_KEYDOWN,
                                        WPARAM(VK_MENU.0 as _),
                                        LPARAM(0),
                                    );    
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        };
    }

    return if app.ui_state.show_menu {
        LRESULT(1 as isize)
    } else {
        CallWindowProcW(*OLD_WND_PROC.get().unwrap(), hwnd, msg, wparam, lparam)
    };
}

pub fn subscribe() -> Result<()> {
    let vtable = get_vtable();
    unsafe {
        subscribe_function!(Present_Detour, vtable[8], present);
    }
    unsafe {
        subscribe_function!(
            Resize_Buffers_Detour,
            vtable[13],
            resize_buffers
        );
    }

    Ok(())
}
