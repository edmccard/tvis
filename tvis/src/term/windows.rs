#![cfg(windows)]

use std::ptr;
use std::sync::atomic::Ordering;
use std::sync::mpsc::Sender;
use winapi;
use kernel32;
use tvis_util::Handle;
use tvis_util::size::get_size;
use input::Event;
use term::{Terminal, TERM, WinSize};
use {Error, Result};

pub struct Term {
    in_hndl: winapi::HANDLE,
    out_hndl: winapi::HANDLE,
    init_out_hndl: winapi::HANDLE,
    init_in_mode: Option<winapi::DWORD>,
    tx: Option<Sender<Box<Event>>>,
}

impl Term {
    pub(in term) fn connect(
        tx: Option<Sender<Box<Event>>>,
    ) -> Result<Box<Terminal>> {
        // TODO: make sure console is not redirected, etc.
        if TERM.compare_and_swap(false, true, Ordering::SeqCst) {
            panic!("TODO: better singleton panic message");
        }
        let mut term = Term {
            in_hndl: Handle::Stdin.win_handle(),
            out_hndl: ptr::null_mut(),
            init_out_hndl: Handle::Stdout.win_handle(),
            init_in_mode: None,
            tx,
        };
        term.set_mode()?;
        term.set_buffer()?;
        Ok(Box::new(term))
    }

    fn set_mode(&mut self) -> Result<()> {
        let mut init_in_mode: winapi::DWORD = 0;
        if 0 == unsafe {
            kernel32::GetConsoleMode(self.in_hndl, &mut init_in_mode)
        } {
            return Error::ffi_err("GetConsoleMode failed");
        }
        let in_mode = init_in_mode | winapi::ENABLE_MOUSE_INPUT |
            winapi::ENABLE_WINDOW_INPUT;
        let in_mode = in_mode & !winapi::ENABLE_PROCESSED_INPUT;
        if 0 == unsafe { kernel32::SetConsoleMode(self.in_hndl, in_mode) } {
            return Error::ffi_err("SetConsoleMode failed");
        }
        self.init_in_mode = Some(init_in_mode);
        Ok(())
    }

    fn set_buffer(&mut self) -> Result<()> {
        let hndl = unsafe {
            kernel32::CreateConsoleScreenBuffer(
                winapi::GENERIC_READ | winapi::GENERIC_WRITE,
                winapi::FILE_SHARE_READ | winapi::FILE_SHARE_WRITE,
                ptr::null(),
                winapi::CONSOLE_TEXTMODE_BUFFER,
                ptr::null_mut(),
            )
        };
        if hndl == winapi::INVALID_HANDLE_VALUE {
            return Error::ffi_err("CreateConsoleScreenBuffer failed");
        }

        ::input::ScreenSize::from_hndl(hndl)?;

        if 0 == unsafe { kernel32::SetConsoleActiveScreenBuffer(hndl) } {
            return Error::ffi_err("SetConsoleActiveScreenBuffer failed");
        }

        self.out_hndl = hndl;
        Ok(())
    }
}

impl Terminal for Term {
    fn get_size(&self) -> Result<WinSize> {
        match get_size(Handle::Stdout) {
            Some(ws) => Ok(ws),
            None => Error::ffi_err("GetConsoleScreenBufferInfo failed"),
        }
    }

    fn start_input(&mut self) -> Result<()> {
        ::input::start_threads(
            self.tx.take().expect("start_input may only be called once"),
        )
    }

    #[cfg(debug_assertions)]
    fn log(&mut self, text: &str) {
        let crlf = [13u8, 10u8];
        let mut count: winapi::DWORD = 0;
        unsafe {
            kernel32::WriteConsoleA(
                self.out_hndl,
                text.as_ptr() as *const _ as *const winapi::VOID,
                text.len() as winapi::DWORD,
                &mut count,
                ptr::null_mut(),
            );
            kernel32::WriteConsoleA(
                self.out_hndl,
                crlf.as_ptr() as *const _ as *const winapi::VOID,
                2,
                &mut count,
                ptr::null_mut(),
            );
        }
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        unsafe {
            if let Some(mode) = self.init_in_mode {
                kernel32::SetConsoleMode(self.in_hndl, mode);
            }
            kernel32::SetConsoleActiveScreenBuffer(self.init_out_hndl);
            // TODO: remember how to handle init_out_mode
        }
    }
}
