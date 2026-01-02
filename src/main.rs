mod util;

use std::backtrace::BacktraceStatus::Disabled;
use crate::util::vector::Vector3;

use windows::{
    core::*,
    Win32::{Foundation::*,Graphics::Gdi::ValidateRect,System::LibraryLoader::GetModuleHandleA,UI::WindowsAndMessaging::*, },
};

const WINDOW_NAME: &str = "Computer Graphics";
// https://samrambles.com/guides/window-hacking-with-rust/creating-a-window-with-rust/index.html
fn main()->Result<()> {
    //win 32 acces raw pointers
    // Rust cant verify safety

    unsafe {
        let module = GetModuleHandleA(None).unwrap();
        let instance = HINSTANCE(module.0);
        let wc:WNDCLASSA = WNDCLASSA{
            lpfnWndProc: Some(window_proc),
            hInstance:instance,
            lpszClassName:s!("Window"),
            hCursor:LoadCursorW(None,IDC_ARROW)?,
            style: CS_HREDRAW|CS_VREDRAW,
            ..Default::default()
        };
        let atom = RegisterClassA(&wc);
        debug_assert!(atom != 0);
        let hwnd = CreateWindowExA(
            WINDOW_EX_STYLE::default(),

            s!("window"),
            s!("Computer Graphics"),
            WS_OVERLAPPEDWINDOW|WS_VISIBLE,
            CW_USEDEFAULT,CW_USEDEFAULT,CW_USEDEFAULT,CW_USEDEFAULT,
            None,
            None,
            Some(instance),
            None);
        let mut msg:MSG = MSG::default();
        while GetMessageA(&mut msg, None,0,0).into(){
            DispatchMessageA(&msg);
        }
        Ok(())
    }
}

extern "system" fn window_proc(hwnd:HWND,message:u32,wparam:WPARAM,lparam: LPARAM)->LRESULT{
    unsafe {
        match message {
            WM_PAINT => {
                println!("WM_PAINT");
                ValidateRect(Some(hwnd),None);
                LRESULT(0)
            },
            WM_DESTROY => {
                println!("Destroying window  ");
                PostQuitMessage(0);
                LRESULT(0)
            },
            _=>DefWindowProcA(hwnd,message,wparam,lparam)
        }
    }
}