#![cfg(target_os = "windows")]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
type BOOL = i32;
type HANDLE = *mut u8;
type LPVOID = *mut u8;
type PVOID = *mut u8;
type DWORD = u32;
type LPDWORD = *mut DWORD;
type ULONG = u32;
type ULONG_PTR = *mut ULONG;
#[repr(C)]
#[derive(Copy, Clone)]
struct DUMMYSTRUCTNAME {
    Offset: DWORD,
    OffsetHigh: DWORD,
}
#[repr(C)]
union DUMMYUNIONNAME {
    DUMMYSTRUCTNAME: DUMMYSTRUCTNAME,
    Pointer: PVOID,
}
#[repr(C)]
struct OVERLAPPED {
    Internal: ULONG_PTR,
    InternalHigh: ULONG_PTR,
    DUMMYUNIONNAME: DUMMYUNIONNAME,
    hEvent: HANDLE,
}
type LPOVERLAPPED = *mut OVERLAPPED;
type LPOVERLAPPED_COMPLETION_ROUTINE = unsafe extern "C" fn(
    dwErrorCode: DWORD,               // [in]
    dwNumberOfBytesTransfered: DWORD, // [in]
    lpOverlapped: LPOVERLAPPED,       // [in, out]
);
extern "C" {
    fn ReadFile(
        hFile: HANDLE,                // [in]
        lpBuffer: LPVOID,             // [out]
        nNumberOfBytesToRead: DWORD,  // [in]
        lpNumberOfBytesRead: LPDWORD, // [out, optional]
        lpOverlapped: LPOVERLAPPED,   // [in, out, optional]
    ) -> BOOL;
    fn ReadFileEx(
        hFile: HANDLE,                                        // [in]
        lpBuffer: LPVOID,                                     // [out, optional]
        nNumberOfBytesToRead: DWORD,                          // [in]
        lpOverlapped: LPOVERLAPPED,                           // [in, out]
        lpCompletionRoutine: LPOVERLAPPED_COMPLETION_ROUTINE, // [in]
    ) -> BOOL;
}
