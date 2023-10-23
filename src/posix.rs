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

enum OperationResult {
    Ok(usize),
    Pending,
    Err(io::Error),
}

impl Operation<'_> {
    fn get_result(&mut self) -> OperationResult {
        match unsafe { aio_error(&self.overlapped.0) } {
            0 => OperationResult::Ok(unsafe { aio_return(&mut self.overlapped.0) } as usize),
            e => {
                if e == libc::EINPROGRESS {
                    return OperationResult::Pending;
                }
                OperationResult::Err(io::Error::from_raw_os_error(e))
            }
        }
    }
}

impl File {
    fn internal_open(path: &CStr, oflag: i32) -> io::Result<Self> {
        let fd = unsafe { open(path.as_ptr(), oflag, 0o644) };
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
    pub fn write<'a>(
        &'a mut self,
        overlapped: &'a mut Overlapped,
        buffer: &'a [u8],
    ) -> io::Result<Operation<'a>> {
        *overlapped = Default::default();
        overlapped.0.aio_fildes = self.0;
        overlapped.0.aio_buf = buffer.as_ptr() as *mut _;
        overlapped.0.aio_nbytes = buffer.len();
        let r = unsafe { libc::aio_write(&mut overlapped.0) };
        if r == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(Operation {
                file: self,
                overlapped,
            })
        }
    }
    pub fn read<'a>(
        &'a mut self,
        overlapped: &'a mut Overlapped,
        buffer: &'a mut [u8],
    ) -> io::Result<Operation<'a>> {
        *overlapped = Default::default();
        overlapped.0.aio_fildes = self.0;
        overlapped.0.aio_buf = buffer.as_ptr() as *mut _;
        overlapped.0.aio_nbytes = buffer.len();
        let r = unsafe { libc::aio_read(&mut overlapped.0) };
        if r == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(Operation {
                file: self,
                overlapped,
            })
        }
    }
}

#[cfg(test)]
mod test {
    use std::{ffi::CString, thread::yield_now};

    use super::{File, OperationResult, Overlapped};

    #[test]
    fn test() {
        let x: CString = CString::new("_test_posix.txt").unwrap();
        let origin = "Hello, world!";
        {
            let mut file = File::create(&x).unwrap();
            let mut overlapped: Overlapped = Overlapped::default();
            let mut operation = file.write(&mut overlapped, origin.as_bytes()).unwrap();
            loop {
                match operation.get_result() {
                    OperationResult::Ok(bytes_written) => {
                        if bytes_written != origin.len() {
                            panic!();
                        }
                        break;
                    }
                    OperationResult::Pending => {
                        yield_now();
                    }
                    OperationResult::Err(e) => {
                        panic!("e: {}", e);
                    }
                }
            }
        }
        {
            let mut file = File::open(&x).unwrap();
            let mut overlapped: Overlapped = Overlapped::default();
            let mut buffer = [0u8; 1024];
            let mut len = 0;
            {
                let mut operation = file.read(&mut overlapped, &mut buffer).unwrap();
                loop {
                    match operation.get_result() {
                        OperationResult::Ok(bytes_read) => {
                            len = bytes_read;
                            break;
                        }
                        OperationResult::Pending => {
                            yield_now();
                        }
                        OperationResult::Err(e) => {
                            panic!("e: {}", e);
                        }
                    }
                }
            }
            assert_eq!(&buffer[..len], origin.as_bytes());
        }
    }
}
