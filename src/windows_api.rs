#![cfg(target_os = "windows")]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{
    ops::BitOr,
    os::{raw::c_void, windows::raw::HANDLE},
};

// https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp/3f6cc0e2-1303-4088-a26b-fb9582f29197
type LPCSTR = *const u8;

// https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp/262627d8-3418-4627-9218-4ffe110850b2
type DWORD = u32;
type LPDWORD = *mut DWORD;

// https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp/c0b7741b-f577-4eed-aff3-2e909df10a4d
type LPVOID = *mut c_void;
type PVOID = *mut c_void;

// https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp/21eec394-630d-49ed-8b4a-ab74a1614611
type ULONG_PTR = usize;

// https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp/9d81be47-232e-42cf-8f0d-7a3b29bf2eb2
#[repr(transparent)]
struct BOOL(i32);
const FALSE: BOOL = BOOL(0);
const TRUE: BOOL = BOOL(1);
impl Into<bool> for BOOL {
    fn into(self) -> bool {
        self.0 != 0
    }
}

// https://learn.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-overlapped

#[repr(C)]
#[derive(Copy, Clone)]
struct OVERLAPPED_0_0 {
    Offset: DWORD,
    OffsetHigh: DWORD,
}

#[repr(C)]
union OVERLAPPED_0 {
    DUMMYSTRUCTNAME: OVERLAPPED_0_0,
    Pointer: PVOID,
}

#[repr(C)]
struct OVERLAPPED {
    Internal: ULONG_PTR,
    InternalHigh: ULONG_PTR,
    DUMMYUNIONNAME: OVERLAPPED_0,
    hEvent: HANDLE,
}

type LPOVERLAPPED = *mut OVERLAPPED;

// https://learn.microsoft.com/en-us/windows/win32/secauthz/access-mask
#[repr(transparent)]
struct ACCESS_MASK(DWORD);
const GENERIC_READ: ACCESS_MASK = ACCESS_MASK(0x80000000);
const GENERIC_WRITE: ACCESS_MASK = ACCESS_MASK(0x40000000);
impl BitOr for ACCESS_MASK {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        ACCESS_MASK(self.0 | rhs.0)
    }
}

// https://learn.microsoft.com/en-us/windows/win32/api/wtypesbase/ns-wtypesbase-security_attributes
#[repr(C)]
struct SECURITY_ATTRIBUTES {
    nLength: DWORD,
    lpSecurityDescriptor: LPVOID,
    bInheritHandle: BOOL,
}
type LPSECURITY_ATTRIBUTES = *mut SECURITY_ATTRIBUTES;

// https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea
#[link(name = "kernel32")]
extern "system" {
    pub fn CreateFileA(
        lpFileName: LPCSTR,                          // [in]
        dwDesiredAccess: ACCESS_MASK,                // [in]
        dwShareMode: DWORD,                          // [in]
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES, // [in, optional]
        dwCreationDisposition: DWORD,                // [in]
        dwFlagsAndAttributes: DWORD,                 // [in]
        hTemplateFile: HANDLE,                       // [in, optional]
    ) -> HANDLE;
}

// https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile
#[link(name = "kernel32")]
extern "system" {
    pub fn ReadFile(
        hFile: HANDLE,                // [in]
        lpBuffer: LPVOID,             // [out]
        nNumberOfBytesToRead: DWORD,  // [in]
        lpNumberOfBytesRead: LPDWORD, // [out, optional]
        lpOverlapped: LPOVERLAPPED,   // [in, out, optional]
    ) -> BOOL;
}

// https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle
#[link(name = "kernel32")]
extern "system" {
    pub fn CloseHandle(hObject: HANDLE, // [in]
    ) -> BOOL;
}

/*
use std::ops::BitOr;

#[repr(transparent)]
struct BOOL(i32);
const FALSE: BOOL = BOOL(0);
const TRUE: BOOL = BOOL(1);
impl Into<bool> for BOOL {
    fn into(self) -> bool {
        self.0 != 0
    }
}
type HANDLE = *mut u8;
type LPVOID = *mut u8;
type PVOID = *mut u8;
type DWORD = u32;
type LPDWORD = *mut DWORD;
type ULONG = u32;
// https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp/21eec394-630d-49ed-8b4a-ab74a1614611
type ULONG_PTR = usize;
#[repr(C)]
#[derive(Copy, Clone)]
struct OVERLAPPED_0_0 {
    Offset: DWORD,
    OffsetHigh: DWORD,
}
#[repr(C)]
union OVERLAPPED_0 {
    DUMMYSTRUCTNAME: OVERLAPPED_0_0,
    Pointer: PVOID,
}
// https://learn.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-overlapped
#[repr(C)]
struct OVERLAPPED {
    Internal: ULONG_PTR,
    InternalHigh: ULONG_PTR,
    DUMMYUNIONNAME: OVERLAPPED_0,
    hEvent: HANDLE,
}
type LPCSTR = *const u8;
#[repr(C)]
struct SECURITY_ATTRIBUTES {
    nLength: DWORD,
    lpSecurityDescriptor: LPVOID,
    bInheritHandle: BOOL,
}
type LPSECURITY_ATTRIBUTES = *mut SECURITY_ATTRIBUTES;
type LPOVERLAPPED = *mut OVERLAPPED;
// https://learn.microsoft.com/en-us/windows/win32/secauthz/access-mask-format
#[repr(transparent)]
struct ACCESS_MASK(DWORD);
const GENERIC_READ: ACCESS_MASK = ACCESS_MASK(0x80000000);
const GENERIC_WRITE: ACCESS_MASK = ACCESS_MASK(0x40000000);
impl BitOr for ACCESS_MASK {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        ACCESS_MASK(self.0 | rhs.0)
    }
}
#[derive(Default)]
#[repr(transparent)]
struct ShareMode(DWORD);
const FILE_SHARE_READ: ShareMode = ShareMode(0x00000001);
const FILE_SHARE_WRITE: ShareMode = ShareMode(0x00000002);
const FILE_SHARE_DELETE: ShareMode = ShareMode(0x00000004);
impl BitOr for ShareMode {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        ShareMode(self.0 | rhs.0)
    }
}
#[repr(transparent)]
struct CreationDisposition(DWORD);
const CREATE_NEW: CreationDisposition = CreationDisposition(1);
const CREATE_ALWAYS: CreationDisposition = CreationDisposition(2);
const OPEN_EXISTING: CreationDisposition = CreationDisposition(3);
const OPEN_ALWAYS: CreationDisposition = CreationDisposition(4);
const TRUNCATE_EXISTING: CreationDisposition = CreationDisposition(5);
#[derive(Default)]
#[repr(transparent)]
struct FlagsAndAttributes(DWORD);
//
const FILE_ATTRIBUTE_READONLY: FlagsAndAttributes = FlagsAndAttributes(0x0000_0001);
const FILE_ATTRIBUTE_HIDDEN: FlagsAndAttributes = FlagsAndAttributes(0x0000_0002);
const FILE_ATTRIBUTE_SYSTEM: FlagsAndAttributes = FlagsAndAttributes(0x0000_0004);
const FILE_ATTRIBUTE_DIRECTORY: FlagsAndAttributes = FlagsAndAttributes(0x0000_0010);
const FILE_ATTRIBUTE_ARCHIVE: FlagsAndAttributes = FlagsAndAttributes(0x0000_0020);
const FILE_ATTRIBUTE_DEVICE: FlagsAndAttributes = FlagsAndAttributes(0x0000_0040);
const FILE_ATTRIBUTE_NORMAL: FlagsAndAttributes = FlagsAndAttributes(0x0000_0080);
const FILE_ATTRIBUTE_TEMPORARY: FlagsAndAttributes = FlagsAndAttributes(0x0000_0100);
const FILE_ATTRIBUTE_SPARSE_FILE: FlagsAndAttributes = FlagsAndAttributes(0x0000_0200);
const FILE_ATTRIBUTE_REPARSE_POINT: FlagsAndAttributes = FlagsAndAttributes(0x0000_0400);
const FILE_ATTRIBUTE_COMPRESSED: FlagsAndAttributes = FlagsAndAttributes(0x0000_0800);
const FILE_ATTRIBUTE_OFFLINE: FlagsAndAttributes = FlagsAndAttributes(0x0000_1000);
const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED: FlagsAndAttributes = FlagsAndAttributes(0x0000_2000);
const FILE_ATTRIBUTE_ENCRYPTED: FlagsAndAttributes = FlagsAndAttributes(0x0000_4000);
const FILE_ATTRIBUTE_INTEGRITY_STREAM: FlagsAndAttributes = FlagsAndAttributes(0x0000_8000);
const FILE_ATTRIBUTE_VIRTUAL: FlagsAndAttributes = FlagsAndAttributes(0x0001_0000);
const FILE_ATTRIBUTE_NO_SCRUB_DATA: FlagsAndAttributes = FlagsAndAttributes(0x0002_0000);
const FILE_ATTRIBUTE_EA: FlagsAndAttributes = FlagsAndAttributes(0x0004_0000);
const FILE_ATTRIBUTE_PINNED: FlagsAndAttributes = FlagsAndAttributes(0x0008_0000);
const FILE_ATTRIBUTE_UNPINNED: FlagsAndAttributes = FlagsAndAttributes(0x0010_0000);
const FILE_ATTRIBUTE_RECALL_ON_OPEN: FlagsAndAttributes = FlagsAndAttributes(0x0004_0000); // FILE_ATTRIBUTE_EA
const FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS: FlagsAndAttributes = FlagsAndAttributes(0x0040_0000);
//
const FILE_FLAG_BACKUP_SEMANTICS: FlagsAndAttributes = FlagsAndAttributes(0x0200_0000);
const FILE_FLAG_DELETE_ON_CLOSE: FlagsAndAttributes = FlagsAndAttributes(0x0400_0000);
const FILE_FLAG_NO_BUFFERING: FlagsAndAttributes = FlagsAndAttributes(0x2000_0000);
const FILE_FLAG_OPEN_NO_RECALL: FlagsAndAttributes = FlagsAndAttributes(0x0010_0000); // FILE_ATTRIBUTE_UNPINNED
const FILE_FLAG_OPEN_REPARSE_POINT: FlagsAndAttributes = FlagsAndAttributes(0x0020_0000);
const FILE_FLAG_OVERLAPPED: FlagsAndAttributes = FlagsAndAttributes(0x40000000);
const FILE_FLAG_POSIX_SEMANTICS: FlagsAndAttributes = FlagsAndAttributes(0x01000000);
const FILE_FLAG_RANDOM_ACCESS: FlagsAndAttributes = FlagsAndAttributes(0x10000000);
const FILE_FLAG_SESSION_AWARE: FlagsAndAttributes = FlagsAndAttributes(0x00800000);
const FILE_FLAG_SEQUENTIAL_SCAN: FlagsAndAttributes = FlagsAndAttributes(0x08000000);
const FILE_FLAG_WRITE_THROUGH: FlagsAndAttributes = FlagsAndAttributes(0x80000000);
//
impl BitOr for FlagsAndAttributes {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        FlagsAndAttributes(self.0 | rhs.0)
    }
}
extern "system" {
    // https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea
    fn CreateFileA(
        lpFileName: LPCSTR,                          // [in]
        dwDesiredAccess: ACCESS_MASK,                // [in]
        dwShareMode: ShareMode,                      // [in]
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES, // [in, optional]
        dwCreationDisposition: CreationDisposition,  // [in]
        dwFlagsAndAttributes: FlagsAndAttributes,    // [in]
        hTemplateFile: HANDLE,                       // [in, optional]
    ) -> HANDLE;
    fn CloseHandle(hObject: HANDLE, // [in]
    ) -> BOOL;
    fn ReadFile(
        hFile: HANDLE,                // [in]
        lpBuffer: LPVOID,             // [out]
        nNumberOfBytesToRead: DWORD,  // [in]
        lpNumberOfBytesRead: LPDWORD, // [out, optional]
        lpOverlapped: LPOVERLAPPED,   // [in, out, optional]
    ) -> BOOL;
    fn WriteFile(
        hFile: HANDLE,                   // [in]
        lpBuffer: LPVOID,                // [in]
        nNumberOfBytesToWrite: DWORD,    // [in]
        lpNumberOfBytesWritten: LPDWORD, // [out, optional]
        lpOverlapped: LPOVERLAPPED,      // [in, out, optional]
    ) -> BOOL;
    fn GetOverlappedResult(
        hFile: HANDLE,                       // [in]
        lpOverlapped: LPOVERLAPPED,          // [in]
        lpNumberOfBytesTransferred: LPDWORD, // [out]
        bWait: BOOL,                         // [in]
    ) -> BOOL;
    fn CancelIoEx(
        hFile: HANDLE,              // [in]
        lpOverlapped: LPOVERLAPPED, // [in, optional]
    ) -> BOOL;
}
*/
