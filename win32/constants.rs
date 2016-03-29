use win32::types::*;

pub static SW_HIDE: c_int = 0;
pub static SW_MAXIMIZE: c_int = 3;
pub static SW_SHOWMAXIMIZED: c_int = 3;
pub static SW_SHOW: c_int = 5;
pub static SW_MINIMIZE: c_int = 6;
pub static SW_RESTORE: c_int = 9;

pub static WM_CREATE: UINT = 0x0001;
pub static WM_DESTROY: UINT = 0x0002;
pub static WM_COMMAND: UINT = 0x0111;
pub static WM_CONTEXTMENU: UINT = 0x007B;
pub static WM_LBUTTONDBLCLK: UINT = 0x0203;
pub static WM_RBUTTONDOWN: UINT = 0x0204;
pub static WM_HOTKEY: UINT = 0x0312;
pub static WM_USER: UINT = 0x0400;

pub static MOD_ALT: UINT = 0x0001;
pub static MOD_CONTROL: UINT = 0x0002;
pub static MOD_SHIFT: UINT = 0x0004;
pub static MOD_WIN: UINT = 0x0008;
pub static MOD_NOREPEAT: UINT = 0x4000;

pub static VK_0: UINT = 0x30;
pub static VK_1: UINT = 0x31;
pub static VK_2: UINT = 0x32;
pub static VK_3: UINT = 0x33;
pub static VK_4: UINT = 0x34;
pub static VK_5: UINT = 0x35;
pub static VK_6: UINT = 0x36;
pub static VK_7: UINT = 0x37;
pub static VK_8: UINT = 0x38;
pub static VK_9: UINT = 0x39;
pub static VK_Q: UINT = 0x51;

pub static NIM_ADD: DWORD = 0x00000000;
pub static NIM_MODIFY: DWORD = 0x00000001;
pub static NIM_DELETE: DWORD = 0x00000002;
pub static NIM_SETFOCUS: DWORD = 0x00000003;
pub static NIM_SETVERSION: DWORD = 0x00000004;

pub static NIF_MESSAGE: UINT = 0x00000001;
pub static NIF_ICON: UINT = 0x00000002;
pub static NIF_TIP: UINT = 0x00000004;
pub static NIF_STATE: UINT = 0x00000008;
pub static NIF_INFO: UINT = 0x00000010;
pub static NIF_GUID: UINT = 0x00000020;
pub static NIF_REALTIME: UINT = 0x00000040;
pub static NIF_SHOWTIP: UINT = 0x00000080;