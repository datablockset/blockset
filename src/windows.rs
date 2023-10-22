use std::{ffi::CStr, io, os::windows::raw::HANDLE, ptr::null_mut};

use crate::windows_api::{
    to_bool, CancelIoEx, CloseHandle, CreateFileA, CreationDisposition, GetLastError,
    GetOverlappedResult, ReadFile, WriteFile, ACCESS_MASK, BOOL, CREATE_ALWAYS, DWORD,
    ERROR_IO_PENDING, FILE_FLAG_OVERLAPPED, GENERIC_READ, GENERIC_WRITE, INVALID_HANDLE_VALUE,
    LPCVOID, LPVOID, OPEN_ALWAYS, OVERLAPPED,
};

struct Handle(HANDLE);

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.0);
        }
    }
}

impl Handle {
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
            let e = io::Error::last_os_error();
            println!("{}", e);
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
        assert!(!to_bool(result));
        let e = unsafe { GetLastError() };
        if e == ERROR_IO_PENDING {
            Ok(Operation {
                handle: self,
                overlapped,
            })
        } else {
            Err(e.to_error())
        }
    }

    pub fn read<'a>(
        &'a mut self,
        overlapped: &'a mut Overlapped,
        buffer: &'a mut [u8], // it's important that the buffer has the same life time as the overlapped!
    ) -> io::Result<Operation<'a>> {
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
struct Overlapped(OVERLAPPED);

struct Operation<'a> {
    handle: &'a mut Handle,
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
    fn get_result(&mut self, wait: bool) -> io::Result<usize> {
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
            Ok(result as usize)
        } else {
            Err(io::Error::last_os_error())
        }
    }
}

#[cfg(test)]
mod test {
    use std::ffi::CString;

    #[test]
    fn test() {
        use super::{Handle, Overlapped};
        //
        let x: CString = CString::new("_test.txt").unwrap();
        {
            let mut handle = Handle::create(&x).unwrap();
            let mut overlapped = Overlapped(Default::default());
            let mut operation = handle.write(&mut overlapped, b"Hello World!").unwrap();
            let result = operation.get_result(true).unwrap();
            assert_eq!(result, 12);
        }
        {
            let mut handle = Handle::open(&x).unwrap();
            let mut overlapped = Overlapped(Default::default());
            let mut buffer = [0u8; 1024];
            {
                let mut operation = handle.read(&mut overlapped, &mut buffer).unwrap();
                let result = operation.get_result(true).unwrap();
                assert_eq!(result, 12);
            }
            assert_eq!(&buffer[..12], b"Hello World!");
        }
    }
}
