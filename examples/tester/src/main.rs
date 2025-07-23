#![feature(asm_experimental_arch)]
#![no_std]
#![no_main]

// tester to test various aspects of the amiga-sys bindings, such as calling library functions,
// function pointers, etc.

// this example doesn't have proper start up code, so it won't work well when launched
// from Workbench (doesn't respond to WBStartup message and therefore may leak memory)

use core::ffi::c_void;
use amiga_sys::*;

// static data loaded to chip ram
#[unsafe(link_section = ".MEMF_CHIP")]
static CHIP_RAM: [u8; 13] = *include_bytes!("../chipdata.txt");

// static data loaded to fast ram - what if there's no fast ram?
#[unsafe(link_section = ".MEMF_FAST")]
static FAST_RAM: &[u8] = include_bytes!("../fastdata.txt");

#[unsafe(no_mangle)]
extern "C" fn _start() {
    unsafe {
        let execlib = amiga_sys::abs_exec_library();

        let dos = OpenLibrary(execlib, b"dos.library\0".as_ptr(), 0);
        if dos == core::ptr::null_mut() {
            return;
        }
        let out = Output(dos);
        if out == 0 {
            CloseLibrary(execlib, dos);
            return;
        }

        Write(dos, out, b"Tester!\nnot-printed\0".as_ptr() as *const c_void, 8);

        let mem = AvailMem(execlib, 0);
        print(dos, out, b"Avail mem: ");
        print_u32_hex(dos, out, mem, true);

        print(dos, out, &CHIP_RAM);
        print(dos, out, FAST_RAM);
        if (CHIP_RAM.as_ptr() as u32) < 0x80_0000 {
            print(dos, out, b"chip ram address: ok: ");
        } else {
            print(dos, out, b"chip ram address: no chip?: ");
        }
        print_u32_hex(dos, out, CHIP_RAM.as_ptr() as u32, true);

        if (FAST_RAM.as_ptr() as u32) > 0x80_0000 {
            print(dos, out, b"fast ram address: ok: ");
        } else {
            print(dos, out, b"fast ram address: no fast?: ");
        }
        print_u32_hex(dos, out, FAST_RAM.as_ptr() as u32, true);

        test_mathieeesingbas(dos, out);
        test_mathieeedoubbas(dos, out);

        test_rawdofmt(dos);

        print(dos, out, b"\nLibrary versions:\n\n");
        check_library_status(dos, out, b"amigaguide.library\0");
        check_library_status(dos, out, b"asl.library\0");
        check_library_status(dos, out, b"bullet.library\0");
        check_library_status(dos, out, b"commodities.library\0");
        check_library_status(dos, out, b"datatypes.library\0");
        check_library_status(dos, out, b"diskfont.library\0");
        check_library_status(dos, out, b"dos.library\0");
        check_library_status(dos, out, b"exec.library\0");
        check_library_status(dos, out, b"expansion.library\0");
        check_library_status(dos, out, b"gadtools.library\0");
        check_library_status(dos, out, b"graphics.library\0");
        check_library_status(dos, out, b"icon.library\0");
        check_library_status(dos, out, b"iffparse.library\0");
        check_library_status(dos, out, b"intuition.library\0");
        check_library_status(dos, out, b"keymap.library\0");
        check_library_status(dos, out, b"layers.library\0");
        check_library_status(dos, out, b"locale.library\0");
        check_library_status(dos, out, b"lowlevel.library\0");
        check_library_status(dos, out, b"mathffp.library\0");
        check_library_status(dos, out, b"mathieeedoubbas.library\0");
        check_library_status(dos, out, b"mathieeedoubtrans.library\0");
        check_library_status(dos, out, b"mathieeesingbas.library\0");
        check_library_status(dos, out, b"mathieeesingtrans.library\0");
        check_library_status(dos, out, b"mathtrans.library\0");
        check_library_status(dos, out, b"nonvolatile.library\0");
        check_library_status(dos, out, b"realtime.library\0");
        check_library_status(dos, out, b"rexxsyslib.library\0");
        check_library_status(dos, out, b"translator.library\0");
        check_library_status(dos, out, b"utility.library\0");
        check_library_status(dos, out, b"workbench.library\0");

        test_intuition_open_window(dos, out);

        print(dos, out, b"\nResources:\n\n");

        check_resource_status(dos, out, b"battclock.resource\0");
        check_resource_status(dos, out, b"battmem.resource\0");
        check_resource_status(dos, out, b"card.resource\0");
        check_resource_status(dos, out, b"disk.resource\0");
        check_resource_status(dos, out, b"misc.resource\0");
        check_resource_status(dos, out, b"potgo.resource\0");

        let ciaa = OpenResource(execlib, "ciaa.resource\0".as_ptr());
        if ciaa == core::ptr::null_mut() {
            print(dos, out, b"ciaa.resource: missing\n");
        } else {
            // test that a ciaa function can be called
            let _old_mask = AbleICR(ciaa as *mut Library, 0);
            print(dos, out, b"ciaa.resource: found\n");
        }

        print(dos, out, b"\nDone.\n");

        CloseLibrary(execlib, dos);
    }
}

// tests

fn test_mathieeesingbas(dos: *mut Library, out: BPTR) {
    unsafe {
        let execlib = amiga_sys::abs_exec_library();
        let mathlib = OpenLibrary(execlib, b"mathieeesingbas.library\0".as_ptr(), 0);
        if mathlib == core::ptr::null_mut() {
            print(dos, out, b"MISSING: no mathieeesingbas.library\n");
            return;
        }
        let fl = IEEESPFlt(mathlib, 135);
        let fl2 = IEEESPFlt(mathlib, 2);
        let flr = IEEESPDiv(mathlib, fl, fl2);
        let r = IEEESPFix(mathlib, flr);
        compare(dos, out, r as i64, 67, b"mathieeesingbas");
        CloseLibrary(execlib, mathlib);
    }
}

fn test_mathieeedoubbas(dos: *mut Library, out: BPTR) {
    unsafe {
        let execlib = amiga_sys::abs_exec_library();
        let mathlib = OpenLibrary(execlib, b"mathieeedoubbas.library\0".as_ptr(), 0);
        if mathlib == core::ptr::null_mut() {
            print(dos, out, b"MISSING: no mathieeedoubbas.library\n");
            return;
        }
        let fl = IEEEDPFlt(mathlib, 139);
        let fl2 = IEEEDPFlt(mathlib, 2);
        let flr = IEEEDPDiv(mathlib, fl, fl2);
        let r = IEEEDPFix(mathlib, flr);
        compare(dos, out, r as i64, 69, b"mathieeedoubbas");
        CloseLibrary(execlib, mathlib);
    }
}

// TODO: use global asm or a naked functions for callbacks?
// this marks the function as extern "C", which may not be the correct
// way to create these callbacks, because "C" functions have prologues etc.
#[unsafe(no_mangle)]
pub extern "C" fn rawdofmt_callback() {
    use core::arch::asm;

    // read the function arguments from CPU registers d0 and a3
    let ch_d0: u32;
    let putchdata_a3: u32;
    unsafe {
        asm!(
            "",
            out("d0") ch_d0,
            out("a3") putchdata_a3,
        );
    }
    let ch = ch_d0 as u8;
    let dos = putchdata_a3 as *mut Library;
    unsafe {
        let out = Output(dos);
        if out == 0 {
            return;
        }
        let charr: &mut [u8; 1] = &mut [ ch ];
        Write(dos, out, charr.as_ptr() as *const c_void, charr.len() as i32);
    }
    // here we could write the return value to the CPU register d0, but RawDoFmt doesn't need it
}

/// Tests callback function pointer
fn test_rawdofmt(dos: *mut Library) {
    unsafe {
        let datastream: &mut [u8; 4] = &mut [ 0, 123, 9, 99 ];
        let execlib = amiga_sys::abs_exec_library();
        RawDoFmt(execlib, b"RawDoFmt callback: %d: ok\n\0".as_ptr(),
            datastream.as_ptr() as *mut c_void, rawdofmt_callback as FPTR, dos as *mut c_void);
    }
}

fn test_intuition_open_window(dos: *mut Library, out: BPTR) {
    unsafe {
        let execlib = amiga_sys::abs_exec_library();
        let intui = OpenLibrary(execlib, b"intuition.library\0".as_ptr(), 0);
        if intui == core::ptr::null_mut() {
            print(dos, out, b"ERROR: no intuition.library\n");
            return;
        }

        let mut title: [u8; 10] = *b"Close me!\0";
        let winconfig = NewWindow {
            LeftEdge: 420,
            TopEdge: 30,
            Width: 200,
            Height: 100,
            DetailPen: 0,
            BlockPen: 1,
            IDCMPFlags: IDCMP_CLOSEWINDOW,
            Flags: WFLG_SIZEGADGET | WFLG_DRAGBAR | WFLG_DEPTHGADGET | WFLG_CLOSEGADGET | WFLG_ACTIVATE,
            FirstGadget: core::ptr::null_mut(),
            CheckMark: core::ptr::null_mut(),
            Title: (&mut title).as_mut_ptr(),
            Screen: core::ptr::null_mut(),
            BitMap: core::ptr::null_mut(),
            MinWidth: 0,
            MinHeight: 0,
            MaxWidth: 600,
            MaxHeight: 400,
            Type: WBENCHSCREEN as u16,
        };
        let win = OpenWindow(intui, &winconfig);
        if win == core::ptr::null_mut() {
            print(dos, out, b"intuition open window: FAIL: can't open window\n");
            CloseLibrary(execlib, intui);
            return;
        }
        let mut active = true;
        while active {
            Wait(execlib, 1 << (*(*win).UserPort).mp_SigBit);
            let msg = GetMsg(execlib, (*win).UserPort);
            if msg == core::ptr::null_mut() {
                continue;
            }
            let msgclass = (* (msg as *mut IntuiMessage) ).Class;
            ReplyMsg(execlib, msg);
            if msgclass == IDCMP_CLOSEWINDOW {
                CloseWindow(intui, win);
                active = false;
            }
        }
        print(dos, out, b"intuition open window: ok\n");
        CloseLibrary(execlib, intui);
    }
}

fn check_library_status(dos: *mut Library, out: BPTR, libname: &[u8]) {
    unsafe {
        Write(dos, out, libname.as_ptr() as *const c_void, libname.len() as i32 - 1);
        let execlib = amiga_sys::abs_exec_library();
        let lib = OpenLibrary(execlib, libname.as_ptr(), 0);
        if lib == core::ptr::null_mut() {
            print(dos, out, b": missing\n");
            return;
        }
        print(dos, out, b": version ");
        let lib_version = (*lib).lib_Version;
        let lib_revision = (*lib).lib_Revision;
        print_u8(dos, out, lib_version as u8, false);
        print(dos, out, b".");
        print_u8(dos, out, lib_revision as u8, true);
        CloseLibrary(execlib, lib);
    }
}

fn check_resource_status(dos: *mut Library, out: BPTR, resname: &[u8]) {
    unsafe {
        Write(dos, out, resname.as_ptr() as *const c_void, resname.len() as i32 - 1);
        let execlib = amiga_sys::abs_exec_library();
        let lib = OpenResource(execlib, resname.as_ptr());
        if lib == core::ptr::null_mut() {
            print(dos, out, b": missing\n");
            return;
        }
        print(dos, out, b": found\n");
    }
}

// helpers

fn print(dos: *mut Library, out: BPTR, s: &[u8]) {
    unsafe {
        Write(dos, out, s.as_ptr() as *const c_void, s.len() as i32);
    }
}

fn to_hex(val: u8) -> u8 {
    if val < 10 {
        val + b'0'
    } else {
        val - 10 + b'a'
    }
}

fn print_u32_hex(dos: *mut Library, out: BPTR, val: u32, linefeed: bool) {
    unsafe {
        let bb: &mut [u8; 11] =
            &mut [ b'0', b'x', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b'\n' ];
        bb[2] = to_hex(((val >> 28) & 0x0f) as u8);
        bb[3] = to_hex(((val >> 24) & 0x0f) as u8);
        bb[4] = to_hex(((val >> 20) & 0x0f) as u8);
        bb[5] = to_hex(((val >> 16) & 0x0f) as u8);
        bb[6] = to_hex(((val >> 12) & 0x0f) as u8);
        bb[7] = to_hex(((val >> 8) & 0x0f) as u8);
        bb[8] = to_hex(((val >> 4) & 0x0f) as u8);
        bb[9] = to_hex(((val >> 0) & 0x0f) as u8);
        if linefeed {
            Write(dos, out, bb.as_ptr() as *const c_void, bb.len() as i32);
        } else {
            Write(dos, out, bb.as_ptr() as *const c_void, bb.len() as i32 - 1);
        }
    }
}

fn print_u8(dos: *mut Library, out: BPTR, val: u8, linefeed: bool) {
    unsafe {
        let bb: &mut [u8; 4] = &mut [ b' ', b' ', b' ', b'\n' ];
        if val >= 100 {
            bb[0] = (val/100 % 10) as u8 + b'0';
        }
        if val >= 10 {
            bb[1] = (val/10 % 10) as u8 + b'0';
        }
        bb[2] = (val % 10) as u8 + b'0';
        if linefeed {
            Write(dos, out, bb.as_ptr() as *const c_void, bb.len() as i32);
        } else {
            Write(dos, out, bb.as_ptr() as *const c_void, bb.len() as i32 - 1);
        }
    }
}

fn compare(dos: *mut Library, out: BPTR, lhs: i64, rhs: i64, msg: &[u8]) {
    print(dos, out, msg);
    if lhs != rhs {
        print(dos, out, b": FAIL\n");
    } else {
        print(dos, out, b": ok\n");
    }
}

// panic handler

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        let execlib = amiga_sys::abs_exec_library();
        Alert(execlib, AN_Unknown + 101);
    }
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn abort() -> ! {
    unsafe {
        let execlib = amiga_sys::abs_exec_library();
        Alert(execlib, AN_Unknown + 102);
    }
    loop {}
}
