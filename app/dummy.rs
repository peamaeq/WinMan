use std::default::Default;

use win32::constants::*;
use win32::types::{HWND,UINT,WORD,NOTIFYICONDATA,WNDCLASSEXA,WNDPROC,HMENU,
                   HINSTANCE,LPVOID,LPCSTR,LPCWSTR,ULONG_PTR,HICON};
use win32::window::{PostQuitMessage,GetModuleHandleA,
                    Shell_NotifyIcon,RegisterClassExA,CreateWindowExA,GetLastError,LoadImageW,
                    GetSystemMetrics};

use app::window::{Win32Result,Win32Window,OnWindowMessage};

// resource.h
static IDI_ICON1: UINT = 103;

#[allow(dead_code)]
#[allow(non_snake_case_functions)]
fn MAKEINTRESOURCEA(i: UINT) -> LPCSTR {
    ((i as WORD) as ULONG_PTR) as LPCSTR
}

#[allow(non_snake_case_functions)]
fn MAKEINTRESOURCEW(i: UINT) -> LPCWSTR {
    ((i as WORD) as ULONG_PTR) as LPCWSTR
}

pub struct DummyWindow {
    hInstance: HINSTANCE,
    hWnd: HWND,
    nid: Option<NOTIFYICONDATA>,
}

impl DummyWindow {
    pub fn register_systray_icon(&mut self) {
        match self.nid {
            None => {
                let mut nid: NOTIFYICONDATA = Default::default();

                nid.uID = 0x29A;
                nid.uCallbackMessage = 1234;
                nid.uFlags = NIF_ICON | NIF_MESSAGE | NIF_TIP;
                nid.hWnd = self.hWnd;
                nid.hIcon = LoadImageW(
                    self.hInstance,
                    MAKEINTRESOURCEW(IDI_ICON1),
                    1, // IMAGE_ICON
                    GetSystemMetrics(49), // SM_CXSMICON
                    GetSystemMetrics(50), // SM_CYSMICON
                    0 // LR_DEFAULTCOLOR
                    ) as HICON;

                Shell_NotifyIcon(NIM_ADD, &mut nid);

                self.nid = Some(nid);
            }
            Some(_) => { }
        }
    }

    pub fn deregister_systray_icon(&mut self) {
        match self.nid {
            Some(mut nid) => {
                Shell_NotifyIcon(NIM_DELETE, &mut nid);
                self.nid = None;
            }
            None => { }
        }
    }
}

impl Win32Window for DummyWindow {
    fn create(hInstance: Option<HINSTANCE>, wndProc: WNDPROC) -> Win32Result<DummyWindow> {
        let hInstance = hInstance.unwrap_or(GetModuleHandleA(0 as LPCSTR));
        let mut wc: WNDCLASSEXA = Default::default();

        wc.lpfnWndProc = wndProc;
        wc.lpszClassName = "MyMagicClassName".to_c_str().as_ptr();

        if RegisterClassExA(&wc) == 0 {
            return Err(GetLastError());
        }

        let hWnd = CreateWindowExA(
            0,
            "MyMagicClassName".to_c_str().as_ptr(),
            0 as LPCSTR,
            0,
            0,
            0,
            0,
            0,
            0 as HWND,
            0 as HMENU,
            hInstance,
            0 as LPVOID);

        if hWnd == 0 as HWND {
            return Err(GetLastError());
        }

        Ok(DummyWindow {
            hInstance: hInstance,
            hWnd: hWnd,
            nid: None,
        })
    }

    fn get_hwnd(&self) -> HWND {
        self.hWnd
    }

    fn get_hinstance(&self) -> Option<HINSTANCE> {
        Some(self.hInstance)
    }
}

impl OnWindowMessage for DummyWindow {
    fn on_create(&mut self) -> bool {
        self.register_systray_icon();
        true
    }

    fn on_destroy(&mut self) -> bool {
        PostQuitMessage(0);
        true
    }
}