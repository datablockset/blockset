use std::os::windows::raw::HANDLE;

struct Handle(HANDLE);

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.0);
        }
    }
}

impl Handle {
    fn create_file(file_name: &str) -> Self {
        Self(unsafe {
            CreateFileA(
                file_name.as_bytes().as_ptr(),
                GENERIC_READ | GENERIC_WRITE,
                ShareMode::default(),
                null_mut(),
                CREATE_NEW,
                FlagsAndAttributes::default(),
                null_mut(),
            )
        })
    }
    fn read_file<'a, 'b, 'c, T>(
        &'a mut self,
        overlapped: &'b mut Overlapped,
        buffer: &'c mut [u8],
    ) -> io::Result<Operation<'a, 'b, 'c>> {
        to_result(unsafe {
            ReadFile(
                self.0,
                buffer.as_mut_ptr() as LPVOID,
                buffer.len() as DWORD,
                null_mut(),
                &mut overlapped.0,
            )
        })?;
        Ok(Operation {
            handle: self,
            overlapped,
            _buffer: buffer,
        })
    }
    fn write_file<'a, 'b, 'c>(
        &'a mut self,
        overlapped: &'b mut Overlapped,
        buffer: &'c [u8],
    ) -> io::Result<Operation<'a, 'b, 'c>> {
        to_result(unsafe {
            WriteFile(
                self.0,
                buffer.as_ptr() as LPVOID,
                buffer.len() as DWORD,
                null_mut(),
                &mut overlapped.0,
            )
        })?;
        Ok(Operation {
            handle: self,
            overlapped,
            _buffer: &mut [],
        })
    }
}

struct Overlapped(OVERLAPPED);

struct Operation<'a, 'b, 'c> {
    handle: &'a mut Handle,
    overlapped: &'b mut Overlapped,
    _buffer: &'c mut [u8],
}

impl Drop for Operation<'_, '_, '_> {
    fn drop(&mut self) {
        unsafe {
            CancelIoEx(self.handle.0, &mut self.overlapped.0);
        }
        let _ = self.get_result();
    }
}

fn to_result(v: BOOL) -> io::Result<()> {
    if v.into() {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

impl<'a, 'b, 'c> Operation<'a, 'b, 'c> {
    fn get_result(&mut self) -> io::Result<usize> {
        let mut result: DWORD = 0;
        to_result(unsafe {
            GetOverlappedResult(self.handle.0, &mut self.overlapped.0, &mut result, FALSE)
        })?;
        Ok(result as usize)
    }
}
