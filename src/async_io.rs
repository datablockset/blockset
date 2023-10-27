use std::{ffi::CStr, io};

pub enum OperationResult {
    Ok(usize),
    Pending,
    Err(io::Error),
}

pub trait AsyncOperation {
    fn get_result(&mut self) -> OperationResult;
}

pub trait AsyncFile {
    type Operation<'a>: AsyncOperation
    where
        Self: 'a;
    fn read<'a>(&'a mut self, buffer: &'a mut [u8]) -> io::Result<Self::Operation<'a>>;
    fn write<'a>(&'a mut self, buffer: &'a [u8]) -> io::Result<Self::Operation<'a>>;
}

pub trait AsyncIo {
    type File: AsyncFile;
    fn create(&self, path: &CStr) -> io::Result<Self::File>;
    fn open(&self, path: &CStr) -> io::Result<Self::File>;
}
