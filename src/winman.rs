extern crate winapi;
extern crate kernel32;
extern crate user32;

mod constants;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use winapi::minwindef::*;
use winapi::windef::*;
use winapi::winnt::*;
use winapi::winuser::*;

use constants::*;

type Win32Result<T> = Result<T, DWORD>;

pub fn main() {
	println!("Hello Windows!");

    // API demo
    unsafe {
        let foreground_window = user32::GetForegroundWindow();
        println!("{:?}", foreground_window);

        user32::ShowWindow(foreground_window, SW_HIDE);
        kernel32::Sleep(1000);
        user32::ShowWindow(foreground_window, SW_SHOW);
    }

    // Window creation
    unsafe {
        let hwnd = create_window(Some(window_proc)).expect("Window creation failed");
        let mut msg: MSG = MSG {
            hwnd: hwnd,
            message: 0,
            wParam: 0 as WPARAM,
            lParam: 0 as LPARAM,
            time: 0,
            pt: POINT { x: 0, y: 0 },
        };

        register_hotkeys(hwnd);

        while user32::GetMessageW(&mut msg, hwnd, 0, 0) > 0 {
            user32::TranslateMessage(&mut msg);
            user32::DispatchMessageW(&mut msg);
        }
    }

}

unsafe fn create_window(window_proc: WNDPROC) -> Win32Result<HWND> {
	let class_name: Vec<u16> = OsStr::new("MyMagicClassName").encode_wide().collect();

    let window_class = WNDCLASSEXW {
    	cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
    	style: 0,
    	lpfnWndProc: window_proc,
    	cbClsExtra: 0,
    	cbWndExtra: 0,
    	hInstance: 0 as HINSTANCE,
    	hIcon: 0 as HICON,
    	hCursor: 0 as HCURSOR,
    	hbrBackground: 0 as HBRUSH,
    	lpszMenuName: 0 as LPCWSTR,
    	lpszClassName: class_name.as_ptr(),
    	hIconSm: 0 as HICON,
    };

    if user32::RegisterClassExW(&window_class) == 0 {
        return Err(kernel32::GetLastError());
    }

    let hwnd = user32::CreateWindowExW(
        0,
        class_name.as_ptr(),
        0 as LPCWSTR,
        0,
        0,
        0,
        0,
        0,
        0 as HWND,
        0 as HMENU,
        0 as HINSTANCE,
        0 as LPVOID);

    if hwnd == 0 as HWND {
        return Err(kernel32::GetLastError());
    }

    Ok(hwnd)
}

unsafe fn register_hotkeys(hwnd: HWND) {
    // Virtual key codes: https://msdn.microsoft.com/en-us/library/windows/desktop/dd375731(v=vs.85).aspx
    // CTRL-ALT-Q to quit
    user32::RegisterHotKey(hwnd, 0, MOD_ALT | MOD_CONTROL, VK_Q);

    // ALT-SHIFT-1 to ALT-SHIFT-9 to grab windows,
    // ALT-1 to ALT-9 to switch windows
    for i in 1..9 {
        let vk_n = VK_0 + i;

        user32::RegisterHotKey(hwnd, 1, MOD_ALT | MOD_SHIFT, vk_n);
        user32::RegisterHotKey(hwnd, 2, MOD_ALT, vk_n);
    }
}

unsafe extern "system" fn window_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let lresult = match msg {
        WM_CREATE => Some(0),
        
        WM_DESTROY => {
            user32::PostQuitMessage(0);
            Some(0)
        },
        
        WM_COMMAND => {
            let command = LOWORD(wparam as DWORD);

            // Tray commands
            if command == 1 {
                user32::DestroyWindow(hwnd);
            }

            Some(0)
        },

        WM_HOTKEY => {
            let _modifiers = LOWORD(lparam as DWORD);
            let _vk = HIWORD(lparam as DWORD);

            // hotkey_manager.process_hotkey((modifiers, vk));
            Some(0)
        }
        
        user if user >= WM_USER => Some(0),
        
        _ => None
    };

    lresult.unwrap_or(user32::DefWindowProcW(hwnd, msg, wparam, lparam))
}