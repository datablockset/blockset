#![cfg(target_family = "unix")]

use std::{ffi::CStr, io, mem::zeroed, thread::yield_now};

use libc::{aio_cancel, aio_error, aio_return, aiocb, close, open, AIO_NOTCANCELED};

struct File(i32);

impl Drop for File {
    fn drop(&mut self) {
        unsafe {
            close(self.0);
        }
    }
}

struct Overlapped(aiocb);

impl Default for Overlapped {
    fn default() -> Self {
        Self(unsafe { zeroed() })
    }
}

struct Operation<'a> {
    file: &'a mut File,
    overlapped: &'a mut Overlapped,
}

impl Drop for Operation<'_> {
    fn drop(&mut self) {
        let mut e = unsafe { aio_cancel(self.file.0, &mut self.overlapped.0) };
        while e == AIO_NOTCANCELED {
            yield_now();
            e = unsafe { aio_error(&self.overlapped.0) };
        }
    }
}

impl Operation<'_> {
    fn get_result(&mut self) -> io::Result<usize> {
        match unsafe { aio_error(&self.overlapped.0) } {
            0 => Ok(unsafe { aio_return(&mut self.overlapped.0) } as usize),
            _ => Err(io::Error::last_os_error()),
        }
    }
}

impl File {
    fn internal_open(path: &CStr, oflag: i32) -> io::Result<Self> {
        let fd = unsafe { open(path.as_ptr(), oflag) };
        if fd == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(Self(fd))
        }
    }
    pub fn create(path: &CStr) -> io::Result<Self> {
        File::internal_open(path, libc::O_WRONLY | libc::O_CREAT | libc::O_TRUNC)
    }
    pub fn open(path: &CStr) -> io::Result<Self> {
        File::internal_open(path, libc::O_RDONLY)
    }
}

#[cfg(test)]
mod test {
    use std::{ffi::CString, mem::zeroed, thread::sleep, time::Duration};

    use libc::{aio_error, aio_return, aiocb, open, O_CREAT, O_TRUNC, O_WRONLY};

    #[test]
    fn test() {
        let x: CString = CString::new("_test_posix.txt").unwrap();
        let fd = unsafe { open(x.as_ptr(), O_WRONLY | O_CREAT | O_TRUNC) };
        if fd == -1 {
            panic!();
        }
        let buffer = "Hello, world!";
        let mut aiocb: aiocb = unsafe { zeroed() };
        aiocb.aio_fildes = fd;
        aiocb.aio_buf = buffer.as_ptr() as *mut _;
        aiocb.aio_nbytes = buffer.len();

        let r = unsafe { libc::aio_write(&mut aiocb) };
        if r == -1 {
            panic!();
        }

        loop {
            match unsafe { aio_error(&mut aiocb) } {
                libc::EINPROGRESS => {
                    sleep(Duration::from_millis(100));
                }
                0 => {
                    let bytes_written = unsafe { aio_return(&mut aiocb) };
                    if bytes_written != buffer.len() as isize {
                        panic!();
                    }
                    break;
                }
                _ => panic!(),
            }
        }

        unsafe { libc::close(fd) };
    }
}
