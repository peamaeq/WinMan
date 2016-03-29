// https://github.com/klutzy/rust-windows/blob/master/ll/types.rs

// Defines all types in [Windows Data Types][win-type] except for some funny families:
// - `TCHAR`
// - `POINTER_{32, 64, SIGNED, UNSIGNED}`
// - `LONGLONG` (just because I'm tired)
//
// See Also [Windows Coding Conventions][win-conv] for deciphering typenames.
//
// [win-type]: http://msdn.microsoft.com/en-us/library/windows/desktop/aa383751%28v=vs.85%29.aspx
// [win-conv]: http://msdn.microsoft.com/en-us/library/windows/desktop/ff381404%28v=vs.85%29.aspx

// FIXME: a lot of types are also defined in liblibc.
// Here we only re-export c types and redefine windows-specific types.

pub use libc::types::os::arch::c95::{c_int, c_uint, c_long, c_ulong, c_short, c_ushort,
                                     c_float, c_double, c_char, c_uchar, c_schar, wchar_t};
pub use libc::types::os::arch::c99::{c_longlong, c_ulonglong};
pub use libc::types::common::c95::c_void;

use std::default::Default;
use std::mem::size_of;

// windef.h
pub type ATOM = WORD;
pub type BOOL = c_int;
pub type BYTE = u8;
pub type COLORREF = DWORD;
pub type FLOAT = c_float;
pub type HACCEL = HANDLE;
pub type HBITMAP = HANDLE;
pub type HBRUSH = HANDLE;
pub type HCOLORSPACE = HANDLE;
pub type HCURSOR = HICON;
pub type HDC = HANDLE;
pub type HDESK = HANDLE;
pub type HENHMETAFILE = HANDLE;
pub type HFILE = c_int;
pub type HFONT = HANDLE;
pub type HGDIOBJ = HANDLE;
pub type HGLOBAL = HANDLE;
pub type HHOOK = HANDLE;
pub type HICON = HANDLE;
pub type HINSTANCE = HANDLE;
pub type HKEY = HANDLE;
pub type HKL = HANDLE;
pub type HLOCAL = HANDLE;
pub type HMENU = HANDLE;
pub type HMETAFILE = HANDLE;
pub type HMODULE = HINSTANCE;
pub type HMONITOR = HANDLE; // WINVER >= 0x0500
pub type HPALETTE = HANDLE;
pub type HPEN = HANDLE;
pub type HRGN = HANDLE;
pub type HRSRC = HANDLE;
pub type HWINSTA = HANDLE;
pub type HWND = HANDLE;
pub type INT = c_int;
pub type LPARAM = LONG_PTR;
pub type LRESULT = LONG_PTR;
pub type UCHAR = c_uchar;
pub type UINT = c_uint;
pub type ULONG = c_ulong;
pub type USHORT = c_ushort;
pub type WORD = c_ushort;
pub type WPARAM = UINT_PTR;

// `LP-`: long pointer
pub type LPBOOL = *mut BOOL;
pub type LPBYTE = *mut BYTE;
pub type LPCOLORREF = *mut DWORD;
pub type LPCVOID = *const c_void;
pub type LPDWORD = *mut DWORD;
pub type LPHANDLE = *mut HANDLE;
pub type LPINT = *mut c_int;
pub type LPLONG = *mut c_long;
pub type LPVOID = *mut c_void;
pub type LPWORD = *mut WORD;

// `P-`: pointer
pub type PBOOL = *mut BOOL;
pub type PBYTE = *mut BYTE;
pub type PDWORD = *mut DWORD;
pub type PFLOAT = *mut FLOAT;
pub type PHKEY = *mut HKEY;
pub type PINT = *mut c_int;
pub type PUCHAR = *mut UCHAR;
pub type PUINT = *mut UINT;
pub type PULONG = *mut ULONG;
pub type PUSHORT = *mut USHORT;
pub type PWORD = *mut WORD;

// winnt.h
pub type BOOLEAN = BYTE;
pub type CCHAR = c_char;
pub type CHAR = c_char;
pub type HANDLE = PVOID;
pub type HRESULT = LONG;
pub type LANGID = WORD;
pub type LONG = c_long;
pub type SHORT = c_short;
//pub type USN = LONGLONG;
pub type VOID = c_void;
pub type WCHAR = wchar_t;

pub type LPCSTR = *const CHAR;
pub type LPCWSTR = *const WCHAR;
pub type LPSTR = *mut CHAR;
pub type LPWSTR = *mut WCHAR;

pub type PBOOLEAN = *mut BOOLEAN;
pub type PCHAR = *mut CHAR;
pub type PCSTR = *const CHAR;
pub type PCWSTR = *const WCHAR;
pub type PDWORDLONG = *mut DWORDLONG;
pub type PHANDLE = *mut HANDLE;
pub type PLCID = PDWORD;
pub type PLONG = *mut LONG;
pub type PSHORT = *mut SHORT;
pub type PSTR = *mut CHAR;
pub type PVOID = *mut c_void;
pub type PWCHAR = *mut WCHAR;
pub type PWSTR = *mut WCHAR;

// intsafe.h
pub type DWORD = c_ulong;
pub type DWORDLONG = c_ulonglong;

// basetsd.h
pub type DWORD_PTR = ULONG_PTR;

#[cfg(target_arch = "x86")] pub type HALF_PTR = c_short;
#[cfg(target_arch = "x86_64")] pub type HALF_PTR = c_int;

#[cfg(target_arch = "x86")] pub type INT_PTR = c_int;
#[cfg(target_arch = "x86_64")] pub type INT_PTR = i64;

#[cfg(target_arch = "x86")] pub type LONG_PTR = c_long;
#[cfg(target_arch = "x86_64")] pub type LONG_PTR = i64;

pub type SIZE_T = ULONG_PTR;
pub type SSIZE_T = LONG_PTR;

#[cfg(target_arch = "x86")] pub type UHALF_PTR = c_ushort;
#[cfg(target_arch = "x86_64")] pub type UHALF_PTR = c_uint;

#[cfg(target_arch = "x86")] pub type UINT_PTR = c_uint;
#[cfg(target_arch = "x86_64")] pub type UINT_PTR = u64;

#[cfg(target_arch = "x86")] pub type ULONG_PTR = c_ulong;
#[cfg(target_arch = "x86_64")] pub type ULONG_PTR = u64;

pub type PDWORD_PTR = *mut ULONG_PTR;
pub type PHALF_PTR = *mut HALF_PTR;
pub type PINT_PTR = *mut INT_PTR;
pub type PLONG_PTR = *mut LONG_PTR;
pub type PSIZE_T = *mut SIZE_T;
pub type PSSIZE_T = *mut SSIZE_T;
pub type PUHALF_PTR = *mut UHALF_PTR;
pub type PUINT_PTR = *mut UINT_PTR;
pub type PULONG_PTR = *mut ULONG_PTR;

pub type DWORD32 = c_uint;
pub type DWORD64 = c_ulonglong;
pub type INT8 = c_char;
pub type INT16 = c_short;
pub type INT32 = c_int;
pub type INT64 = c_longlong;
pub type LONG32 = c_int;
pub type LONG64 = c_longlong;
pub type UINT8 = c_uchar;
pub type UINT16 = c_ushort;
pub type UINT32 = c_uint;
pub type UINT64 = c_ulonglong;
pub type ULONG32 = c_uint;
pub type ULONG64 = c_ulonglong;

pub type PDWORD32 = *mut DWORD32;
pub type PDWORD64 = *mut DWORD64;
pub type PINT8 = *mut INT8;
pub type PINT16 = *mut INT16;
pub type PINT32 = *mut INT32;
pub type PINT64 = *mut INT64;
pub type PLONG32 = *mut LONG32;
pub type PLONG64 = *mut LONG64;
pub type PUINT8 = *mut UINT8;
pub type PUINT16 = *mut UINT16;
pub type PUINT32 = *mut UINT32;
pub type PUINT64 = *mut UINT64;
pub type PULONG32 = *mut ULONG32;
pub type PULONG64 = *mut ULONG64;

// ddeml.h
pub type HCONV = HANDLE;
pub type HCONVLIST = HANDLE;
pub type HDDEDATA = HANDLE;
pub type HSZ = HANDLE;

// shellapi.h
pub type HDROP = HANDLE;

// winuser.h
pub type HDWP = HANDLE;

// winnls.h
pub type LCID = DWORD;
pub type LCTYPE = DWORD;
pub type LGRPID = DWORD;

// winsvc.h
pub type SC_HANDLE = HANDLE;
pub type SC_LOCK = LPVOID;
pub type SERVICE_STATUS_HANDLE = HANDLE;

// winternl.h
#[allow(uppercase_variables)]
pub struct UNICODE_STRING {
    pub Length: USHORT,
    pub MaximumLength: USHORT,
    pub Buffer: PWSTR,
}
type PUNICODE_STRING = *mut UNICODE_STRING;
type PCUNICODE_STRING = *const UNICODE_STRING;

// not specified
pub type QWORD = u64; // unsigned __int64

// additional types used in common now

#[deriving(Default)]
pub struct RECT {
    pub left: LONG,
    pub top: LONG,
    pub right: LONG,
    pub bottom: LONG,
}
pub type PRECT = *mut RECT;
pub type LPRECT = *mut RECT;
pub type LPCRECT = *const RECT;

#[deriving(Default)]
pub struct POINT {
    pub x: LONG,
    pub y: LONG,
}
pub type PPOINT = *mut POINT;
pub type LPPOINT = *mut POINT;

pub struct MSG {
    pub hwnd: HWND,
    pub message: UINT,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: DWORD,
    pub pt: POINT,
}
pub type PMSG = *mut MSG;
pub type LPMSG = *mut MSG;

impl Default for MSG {
    fn default() -> MSG {
        MSG {
            hwnd: 0 as HWND,
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: Default::default(),
        }
    }
}

pub struct GUID {
    pub Data1: DWORD,
    pub Data2: WORD,
    pub Data3: WORD,
    pub Data4: [BYTE, ..8],
}

impl Default for GUID {
    fn default() -> GUID {
        GUID {
            Data1: 0,
            Data2: 0,
            Data3: 0,
            Data4: [0, ..8],
        }
    }
}

// TCHAR is weird: http://msdn.microsoft.com/en-us/library/bb415628.aspx
// Union is not implemented in Rust
pub struct NOTIFYICONDATA {
    pub cbSize: DWORD,
    pub hWnd: HWND,
    pub uID: UINT,
    pub uFlags: UINT,
    pub uCallbackMessage: UINT,
    pub hIcon: HICON,
    pub szInfo: [c_char, ..256],     // TCHAR[256]
    pub uTimeout_uVersion: UINT,     // union
    pub szInfoTitle: [c_char, ..64], // TCHAR[64]
    pub dwInfoFlags: DWORD,
    pub guidItem: GUID,
    pub hBalloonIcon: HICON,
}
pub type PNOTIFYICONDATA = *mut NOTIFYICONDATA;

impl Default for NOTIFYICONDATA {
    fn default() -> NOTIFYICONDATA {
        NOTIFYICONDATA {
            cbSize: size_of::<NOTIFYICONDATA>() as DWORD,
            hWnd: 0 as HWND,
            uID: 0,
            uFlags: 0,
            uCallbackMessage: 0,
            hIcon: 0 as HICON,
            szInfo: [0, ..256],
            uTimeout_uVersion: 0,
            szInfoTitle: [0, ..64],
            dwInfoFlags: 0,
            guidItem: Default::default(),
            hBalloonIcon: 0 as HICON,
        }
    }
}

// extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT
pub type WNDPROC = *const c_void;

pub struct WNDCLASSEXA {
    pub cbSize: UINT,
    pub style: UINT,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: c_int,
    pub cbWndExtra: c_int,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPCSTR,
    pub lpszClassName: LPCSTR,
    pub hIconSm: HICON,
}

impl Default for WNDCLASSEXA {
    fn default() -> WNDCLASSEXA {
        WNDCLASSEXA {
            cbSize: size_of::<WNDCLASSEXA>() as UINT,
            style: 0,
            lpfnWndProc: 0 as WNDPROC,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: 0 as HINSTANCE,
            hIcon: 0 as HICON,
            hCursor: 0 as HCURSOR,
            hbrBackground: 0 as HBRUSH,
            lpszMenuName: 0 as LPCSTR,
            lpszClassName: 0 as LPCSTR,
            hIconSm: 0 as HICON,
        }
    }
}

pub struct WNDCLASSEXW {
    pub cbSize: UINT,
    pub style: UINT,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: c_int,
    pub cbWndExtra: c_int,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPCWSTR,
    pub lpszClassName: LPCWSTR,
    pub hIconSm: HICON,
}

impl Default for WNDCLASSEXW {
    fn default() -> WNDCLASSEXW {
        WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as UINT,
            style: 0,
            lpfnWndProc: 0 as WNDPROC,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: 0 as HINSTANCE,
            hIcon: 0 as HICON,
            hCursor: 0 as HCURSOR,
            hbrBackground: 0 as HBRUSH,
            lpszMenuName: 0 as LPCWSTR,
            lpszClassName: 0 as LPCWSTR,
            hIconSm: 0 as HICON,
        }
    }
}