use std::any::Any;
use std::{mem, ptr, thread};
use std::sync::mpsc::Sender;

use libc::{self, c_int};

use input::InputEvent;
use {Error, Event, Result};


static mut SIGNAL_FDS: Option<(c_int, c_int)> = None;

pub(crate) fn start_threads(tx: Sender<Box<Event>>) -> Result<()> {
    init_pipes()?;
    thread::spawn(|| unsafe {
        let mut set: libc::sigset_t = mem::uninitialized();
        libc::sigfillset(&mut set);
        libc::pthread_sigmask(libc::SIG_BLOCK, &set, ptr::null_mut());
        raw_event_loop(tx);
    });
    init_signals();
    Ok(())
}

fn init_pipes() -> Result<()> {
    unsafe {
        let mut fds: [c_int; 2] = mem::uninitialized();
        if -1 == libc::pipe(fds.as_mut_ptr()) {
            return Error::ffi_err("pipe failed");
        }
        let flags = libc::fcntl(fds[1], libc::F_GETFL);
        libc::fcntl(fds[1], libc::F_SETFL, flags | libc::O_NONBLOCK);
        SIGNAL_FDS = Some((fds[0], fds[1]));
    }
    Ok(())
}

fn init_signals() {
    extern "C" fn handler(signum: c_int) {
        let val = match signum {
            libc::SIGWINCH => 1,
            libc::SIGTERM => 2,
            libc::SIGINT => 3,
            libc::SIGQUIT => 4,
            _ => return,
        };
        unsafe {
            let signal_fds = SIGNAL_FDS.unwrap();
            let valptr = &val as *const _ as *const libc::c_void;
            libc::write(signal_fds.1, valptr, 1);
        }
    }

    unsafe {
        let mut sa: libc::sigaction = mem::uninitialized();
        let mut set: libc::sigset_t = mem::uninitialized();
        libc::sigemptyset(&mut set);
        sa.sa_sigaction = mem::transmute(handler as extern "C" fn(c_int));
        sa.sa_flags = libc::SA_RESTART;
        sa.sa_mask = set;
        libc::sigaction(libc::SIGWINCH, &sa, ptr::null_mut());
        libc::sigaction(libc::SIGTERM, &sa, ptr::null_mut());
        libc::sigaction(libc::SIGINT, &sa, ptr::null_mut());
        libc::sigaction(libc::SIGQUIT, &sa, ptr::null_mut());
    }
}

unsafe fn raw_event_loop(tx: Sender<Box<Event>>) {
    let mut reader = Reader { buf: [0u8; 1024] };
    let signal_fd = SIGNAL_FDS.unwrap().0;
    let mut read_fds: libc::fd_set = mem::uninitialized();
    // TODO: change calculation if other non-zero fds are used.
    let nfds: c_int = signal_fd + 1;
    loop {
        libc::FD_ZERO(&mut read_fds);
        libc::FD_SET(0, &mut read_fds);
        libc::FD_SET(signal_fd, &mut read_fds);
        let ready = libc::select(
            nfds + 1,
            &mut read_fds,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
        if ready == -1 {
            // TODO: panic?
            return;
        }
        if libc::FD_ISSET(0, &mut read_fds) {
            let event = match reader.read_stdin() {
                None => return,
                Some(event) => event,
            };
            if tx.send(event).is_err() {
                return;
            }
        }
        if libc::FD_ISSET(signal_fd, &mut read_fds) {
            let event = match reader.read_signal(signal_fd) {
                None => return,
                Some(event) => event,
            };
            if tx.send(event).is_err() {
                return;
            }
        }
    }
}

struct Reader {
    buf: [u8; 1024],
}

impl Reader {
    fn read_signal(&self, signal_fd: c_int) -> Option<Box<Event>> {
        let mut buf = 0u8;
        let bufptr = &mut buf as *mut _ as *mut libc::c_void;
        if 1 > unsafe { libc::read(signal_fd, bufptr, 1) } {
            return None;
        }
        match buf {
            1 => Some(Box::new(InputEvent::Repaint)),
            2 => None,
            3 => Some(Box::new(InputEvent::Interrupt)),
            4 => Some(Box::new(InputEvent::Break)),
            _ => unreachable!(),
        }
    }

    fn read_stdin(&mut self) -> Option<Box<Event>> {
        let bufptr = self.buf.as_mut_ptr() as *mut libc::c_void;
        let len = unsafe { libc::read(0, bufptr, 1024) };
        if len < 1 {
            return None;
        }

        println!("KEY DATA: {:?}\r", &self.buf[0..len as usize]);
        if len == 1 && self.buf[0] == 27 {
            return None;
        }
        Some(Box::new(InputEvent::Key))
    }
}
