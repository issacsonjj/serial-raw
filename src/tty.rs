use std::ffi::CString;
use std::mem::uninitialized;

use std::os::unix::prelude::*;
use std::os::unix::io::{AsRawFd, RawFd};
use std::io;
use std::path::Path;

use libc::{O_RDWR, tcgetattr, c_void, size_t, termios, tcdrain, TCSANOW, B9600, B19200, B38400, B115200};

pub struct TTYPort {
    fd: RawFd
}

impl TTYPort {
    pub fn open(path: &Path, speed: i32) -> Result<Self, String> {

        let cstr = match CString::new(path.as_os_str().as_bytes()) {
            Ok(s) => s,
            Err(_) => return Err("Invalid path argument".to_string()),
        };


        let fd = unsafe {
            libc::open(cstr.as_ptr(), O_RDWR, 0)
        };

        println!("fd = {}", fd);

        if fd < 0 {
            return Err(format!("open port {} fail", path.to_str().unwrap()));
        }

        let baudrate = match speed {
            9600 => B9600,
            19200 => B19200,
            38400 => B38400,
            115200 => B115200,
            _ => return Err(format!("Invalid baudrate {}", speed)),
        };

        let mut t: termios = unsafe {
            uninitialized()
        };

        unsafe {
            let ret = tcgetattr(fd, &mut t as *mut termios);

            if ret < 0 {
                println!("get attr fail: {}", ret);
                return Err("get attribute of fd fail".to_string());
            }

            libc::cfmakeraw(&mut t as *mut termios);
            let ret = libc::cfsetispeed(&mut t as *mut termios, baudrate);
            if ret < 0 {
                println!("set input speed fail");
                return Err("set input speed fail".to_string());
            }

            let ret = libc::cfsetospeed(&mut t as *mut termios, baudrate);
            if ret < 0 {
                println!("set ouput speed fail");
                return Err("set output speed fail".to_string());
            }

            let ret = libc::tcsetattr(fd, TCSANOW, &t as *const termios);
            if ret < 0 {
                println!("set terminal attribute fail");
                return Err("set terminal attribute fail".to_string());
            }
        }

        Ok(TTYPort{fd: fd})

    }
}

impl AsRawFd for TTYPort {
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}

impl io::Write for TTYPort {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let len = unsafe {
            libc::write(self.fd, buf.as_ptr() as *mut c_void, buf.len() as size_t)
        };

        if len >= 0 {
            Ok(len as usize)
        }
        else {
            Err(io::Error::last_os_error())
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let ret = unsafe {
            tcdrain(self.fd)
        };
        if ret == 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }
}

impl io::Read for TTYPort {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len = unsafe {
            libc::read(self.fd, buf.as_ptr() as *mut c_void, buf.len() as size_t)
        };

        if len >= 0 {
            Ok(len as usize)
        } else {
            Err(io::Error::last_os_error())
        }
    }
}

impl Drop for TTYPort {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.fd);
        }
    }
}