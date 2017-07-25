#![cfg(windows)]

use std::ptr;
use std::sync::atomic::Ordering;
use std::sync::mpsc::Sender;

use tvis_util::Handle;

use win32;
use {SCREEN, Screen, Error, Event, Result};

pub struct ConsoleScreen {
    in_hndl: win32::Handle,
    out_hndl: win32::Handle,
    init_out_hndl: win32::Handle,
    init_in_mode: Option<u32>,
}

impl ConsoleScreen {
    pub fn init(tx: Sender<Box<Event>>) -> Result<Box<Screen>> {
        // TODO: make sure console is not redirected, etc.
        if SCREEN.compare_and_swap(false, true, Ordering::SeqCst) {
            panic!("TODO: better singleton panic message");
        }
        let mut screen = ConsoleScreen {
            in_hndl: Handle::Stdin.win_handle(),
            out_hndl: ptr::null_mut(),
            init_out_hndl: Handle::Stdout.win_handle(),
            init_in_mode: None,
        };
        screen.set_mode()?;
        screen.set_buffer()?;
        ::input::start_threads(tx)?;
        Ok(Box::new(screen))
    }

    fn set_mode(&mut self) -> Result<()> {
        let mut init_in_mode = 0u32;
        if 0 ==
            unsafe { win32::GetConsoleMode(self.in_hndl, &mut init_in_mode) }
        {
            return Error::ffi_err("GetConsoleMode failed");
        }
        let in_mode = init_in_mode | win32::ENABLE_MOUSE_INPUT |
            win32::ENABLE_WINDOW_INPUT;
        if 0 == unsafe { win32::SetConsoleMode(self.in_hndl, in_mode) } {
            return Error::ffi_err("SetConsoleMode failed");
        }
        self.init_in_mode = Some(init_in_mode);
        Ok(())
    }

    fn set_buffer(&mut self) -> Result<()> {
        let hndl = unsafe {
            win32::CreateConsoleScreenBuffer(
                win32::GENERIC_READ | win32::GENERIC_WRITE,
                win32::FILE_SHARE_READ | win32::FILE_SHARE_WRITE,
                ptr::null(),
                win32::CONSOLE_TEXTMODE_BUFFER,
                ptr::null_mut(),
            )
        };
        if hndl == win32::INVALID_HANDLE_VALUE {
            return Error::ffi_err("CreateConsoleScreenBuffer failed");
        }

        ::input::ScreenSize::from_hndl(hndl)?;

        if 0 == unsafe { win32::SetConsoleActiveScreenBuffer(hndl) } {
            return Error::ffi_err("SetConsoleActiveScreenBuffer failed");
        }

        self.out_hndl = hndl;
        Ok(())
    }
}

impl Screen for ConsoleScreen {}

impl Drop for ConsoleScreen {
    fn drop(&mut self) {
        unsafe {
            if let Some(mode) = self.init_in_mode {
                win32::SetConsoleMode(self.in_hndl, mode);
            }
            win32::SetConsoleActiveScreenBuffer(self.init_out_hndl);
            // TODO: remember how to handle init_out_mode
        }
    }
}
