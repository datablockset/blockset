use std::{io, ffi::CStr};

use crate::async_io::{AsyncFile, AsyncIo};

#[cfg(target_family = "windows")]
use crate::windows::*;

#[cfg(target_family = "unix")]
use crate::unix::*;

struct AFile {
    file: File,
    overlapped: Overlapped,
}

impl AsyncFile for AFile {
    type Operation<'a> = Operation<'a>;

    fn read<'a>(&'a mut self, buffer: &'a mut [u8]) -> io::Result<Self::Operation<'a>> {
        self.file.read(&mut self.overlapped, buffer)
    }

    fn write<'a>(&'a mut self, buffer: &'a [u8]) -> io::Result<Self::Operation<'a>> {
        self.file.write(&mut self.overlapped, buffer)
    }
}

struct AIo();

impl AsyncIo for AIo {
    type File = AFile;

    fn create(&self, path: &CStr) -> io::Result<Self::File> {
        Ok(AFile {
            file: File::create(path)?,
            overlapped: Default::default(),
        })
    }

    fn open(&self, path: &CStr) -> io::Result<Self::File> {
        Ok(AFile {
            file: File::open(path)?,
            overlapped: Default::default(),
        })
    }
}

#[cfg(test)]
mod test {
    use std::{ffi::CString, thread::yield_now};

    use crate::async_io::{OperationResult, AsyncOperation};

    #[test]
    fn test() {
        use super::{File, Overlapped};
        //
        let x: CString = CString::new("_test.txt").unwrap();
        let origin = b"Hello World!";
        {
            let mut handle = File::create(&x).unwrap();
            let mut overlapped = Overlapped::default();
            let mut operation = handle.write(&mut overlapped, origin).unwrap();
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
            // let result = operation.get_result(true).unwrap();
            // assert_eq!(result, 12);
        }
        {
            let mut handle = File::open(&x).unwrap();
            let mut overlapped = Overlapped::default();
            let mut buffer = [0u8; 1024];
            {
                let mut operation = handle.read(&mut overlapped, &mut buffer).unwrap();
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
                // let result = operation.get_result(true).unwrap();
                // assert_eq!(result, 12);
            }
            assert_eq!(&buffer[..12], b"Hello World!");
        }
    }
}
