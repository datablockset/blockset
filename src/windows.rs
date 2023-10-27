#![cfg(target_family = "windows")]
#![cfg(not(tarpaulin_include))]
use std::{ffi::CStr, io, os::windows::raw::HANDLE, ptr::null_mut};

use io_trait::{OperationResult, AsyncOperation};

use crate::windows_api::{
    to_bool, CancelIoEx, CloseHandle, CreateFileA, CreationDisposition, GetLastError,
    GetOverlappedResult, ReadFile, WriteFile, ACCESS_MASK, BOOL, CREATE_ALWAYS, DWORD,
    ERROR_IO_PENDING, FILE_FLAG_OVERLAPPED, GENERIC_READ, GENERIC_WRITE, INVALID_HANDLE_VALUE,
    LPCVOID, LPVOID, OPEN_ALWAYS, OVERLAPPED,
};

pub struct File(HANDLE);

impl Drop for File {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.0);
        }
    }
}

fn to_operation_result(v: BOOL) -> OperationResult {
    if to_bool(v) {
        OperationResult::Ok(0)
    } else {
        let e = unsafe { GetLastError() };
        if e == ERROR_IO_PENDING {
            OperationResult::Pending
        } else {
            OperationResult::Err(e.to_error())
        }
    }
}

impl File {
    fn create_file(
        file_name: &CStr,
        desired_access: ACCESS_MASK,
        creation_disposition: CreationDisposition,
    ) -> io::Result<Self> {
        let result = unsafe {
            CreateFileA(
                file_name.as_ptr(),
                desired_access,
                0,
                null_mut(),
                creation_disposition,
                FILE_FLAG_OVERLAPPED,
                null_mut(),
            )
        };
        if result == INVALID_HANDLE_VALUE {
            Err(io::Error::last_os_error())
        } else {
            Ok(Self(result))
        }
    }
    pub fn create(file_name: &CStr) -> io::Result<Self> {
        Self::create_file(file_name, GENERIC_WRITE, CREATE_ALWAYS)
    }
    pub fn open(file_name: &CStr) -> io::Result<Self> {
        Self::create_file(file_name, GENERIC_READ, OPEN_ALWAYS)
    }

    fn create_operation<'a>(
        &'a mut self,
        overlapped: &'a mut Overlapped,
        result: BOOL,
    ) -> io::Result<Operation<'a>> {
        if let OperationResult::Err(e) = to_operation_result(result) {
            Err(e)
        } else {
            Ok(Operation {
                handle: self,
                overlapped,
            })
        }
    }

    pub fn read<'a>(
        &'a mut self,
        overlapped: &'a mut Overlapped,
        buffer: &'a mut [u8], // it's important that the buffer has the same life time as the overlapped!
    ) -> io::Result<Operation<'a>> {
        *overlapped = Default::default();
        let result = unsafe {
            ReadFile(
                self.0,
                buffer.as_mut_ptr() as LPVOID,
                buffer.len() as DWORD,
                null_mut(),
                &mut overlapped.0,
            )
        };
        self.create_operation(overlapped, result)
    }

    pub fn write<'a>(
        &'a mut self,
        overlapped: &'a mut Overlapped,
        buffer: &'a [u8], // it's important that the buffer has the same life time as the overlapped!
    ) -> io::Result<Operation<'a>> {
        *overlapped = Default::default();
        let result = unsafe {
            WriteFile(
                self.0,
                buffer.as_ptr() as LPCVOID,
                buffer.len() as DWORD,
                null_mut(),
                &mut overlapped.0,
            )
        };
        self.create_operation(overlapped, result)
    }
}

#[derive(Default)]
pub struct Overlapped(OVERLAPPED);

pub struct Operation<'a> {
    handle: &'a mut File,
    overlapped: &'a mut Overlapped,
}

impl Drop for Operation<'_> {
    fn drop(&mut self) {
        unsafe {
            CancelIoEx(self.handle.0, &mut self.overlapped.0);
        }
        let _ = self.get_result(true);
    }
}

impl Operation<'_> {
    fn get_result(&mut self, wait: bool) -> OperationResult {
        let mut result: DWORD = 0;
        let r = unsafe {
            GetOverlappedResult(
                self.handle.0,
                &mut self.overlapped.0,
                &mut result,
                wait.into(),
            )
        };
        if r.into() {
            OperationResult::Ok(result as usize)
        } else {
            let e = unsafe { GetLastError() };
            if e == ERROR_IO_PENDING {
                OperationResult::Pending
            } else {
                OperationResult::Err(e.to_error())
            }
        }
    }
}

impl AsyncOperation for Operation<'_> {
    fn get_result(&mut self) -> OperationResult {
        self.get_result(false)
    }
}
