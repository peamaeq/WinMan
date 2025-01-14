use std::collections::{HashMap, VecDeque};

use kernel32;
use user32;
use winapi::minwindef::*;
use winapi::windef::*;

use utils;
use utils::Win32Result;

#[derive(Clone)]
pub struct Window {
    hwnd: HWND,
    title: Option<String>,
}

impl Window {
	pub fn new(hwnd: HWND, title: String) -> Self {
		Window {
			hwnd: hwnd,
			title: Some(title),
		}
	}

	pub fn hwnd(&self) -> HWND {
		self.hwnd
	}

	pub fn title(&self) -> Option<&str> {
		match self.title {
			Some(ref t) => Some(&t),
			None => None
		}
	}
}

pub struct WindowSet {
	windows: VecDeque<Window>
}

impl WindowSet {
	pub fn new() -> Self {
		WindowSet {
			windows: VecDeque::new()
		}
	}

	pub fn add(&mut self, window: Window) {
		self.remove(&window);
		self.windows.push_front(window);
	}
	
	pub fn remove(&mut self, window: &Window) -> Option<Window> {
		let index = self.windows
		                .iter()
		                .position(|w| w.hwnd == window.hwnd);

        match index {
        	Some(index) => {
        		self.windows.remove(index)
        	},
        	None => None
        }
	}

	pub fn cycle(&mut self) -> Option<Window> {
		if let Some(back) = self.windows.pop_back() {
			self.windows.push_front(back);
		}

		match self.windows.front() {
			Some(window) => Some(window.clone()),
			None => None
		}
	}
}

pub struct Config {
    windows: HashMap<UINT, WindowSet>
}

impl Config {
	pub fn new() -> Self {
		Config {
			windows: HashMap::new()
		}
	}

	pub fn track_window(&mut self, vk: UINT, window: Window) {
		let mut window_set = self.windows
		                         .entry(vk)
		                         .or_insert(WindowSet::new());

		window_set.add(window);		
	}

	pub fn get_windows(&mut self, vk: UINT) -> Option<&mut WindowSet> {
		self.windows.get_mut(&vk)
	}

	pub fn clear_windows(&mut self, vk: UINT) {
		self.windows.remove(&vk);
	}
}

pub fn get_foreground_window() -> Win32Result<Window> {
	let hwnd = {
		let hwnd = unsafe { user32::GetForegroundWindow() };

		if hwnd == 0 as HWND {
			return Err(unsafe { kernel32::GetLastError() });
		}

		hwnd
	};

	let title = utils::api_wrappers::get_window_text(hwnd).unwrap_or("".to_string());
	let window = Window::new(hwnd, title);

	Ok(window)
}

pub fn set_foreground_window(hwnd: HWND) -> Win32Result<()> {
	unsafe {
		use winapi::*;
		use user32;

		let mut placement: winuser::WINDOWPLACEMENT = ::std::mem::zeroed();
		placement.length = ::std::mem::size_of::<winuser::WINDOWPLACEMENT>() as u32;
		user32::GetWindowPlacement(hwnd, &mut placement);

		let sw = match placement.showCmd as i32 {
			SW_SHOWMAXIMIZED => SW_SHOWMAXIMIZED,
			SW_SHOWMINIMIZED => SW_RESTORE,
			_ => SW_NORMAL,
		};

		user32::ShowWindow(hwnd, sw);
		if user32::SetForegroundWindow(hwnd) == 0 {
			return Err(kernel32::GetLastError());
		}
	}

	Ok(())
}
