#![cfg(not(target_os = "windows"))]

#[cfg(test)]
mod test {
    use std::{ffi::CString, mem::zeroed, thread::sleep, time::Duration};

    use libc::{aio_error, aio_return, aiocb, open, O_CREAT, O_TRUNC, O_WRONLY};

    #[test]
    fn test() {
        let x: CString = CString::new("_test_posix.txt").unwrap();
        let fd = unsafe { open(x.as_ptr(), O_WRONLY | O_CREAT | O_TRUNC, 0o644) };
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
