#![no_std]
#![no_main]

// example to show how to print strings to the Amiga CLI

// this example doesn't have proper start up code, so it won't work well when launched
// from Workbench (doesn't respond to WBStartup message and therefore may leak memory)

use core::ffi::c_void;
use amiga_sys;

#[unsafe(no_mangle)]
extern "C" fn _start() {
    unsafe {
        let execlib = amiga_sys::abs_exec_library();

        // OpenLibrary wants a null-terminated string
        let doslib = amiga_sys::OpenLibrary(execlib, b"dos.library\0".as_ptr(), 0);
        if doslib == core::ptr::null_mut() {
            // null means library not found
            return;
        }
        // get cli output file handle
        let out_handle = amiga_sys::Output(doslib);
        if out_handle == 0 {
            // 0 means no output stream (e.g. launched from workbench)
            amiga_sys::CloseLibrary(execlib, doslib);
            return;
        }

        // Amiga uses the ECMA-94 text encoding, which is the same as ISO-8859-1 and Latin1.
        // https://en.wikipedia.org/wiki/ISO/IEC_8859-1#History
        // This writes the string "Hello Amiga, 2²×3½÷2=7!\n", which is 24 bytes long.
        amiga_sys::Write(doslib, out_handle,
            b"Hello Amiga, 2\xb2\xd73\xbd\xf72=7!\n".as_ptr() as *const c_void, 24);

        // we must close libraries opened with OpenLibrary()
        amiga_sys::CloseLibrary(execlib, doslib);
    }
    return;
}

// panic handler

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn abort() -> ! {
    loop {}
}
