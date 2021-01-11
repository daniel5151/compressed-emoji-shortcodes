#![no_std]
#![no_main]

extern crate libc;

extern "C" {
    pub static stdin: *mut libc::FILE;
}

fn print_str(s: &str) {
    unsafe {
        libc::write(1, s.as_ptr() as _, s.len());
        libc::write(1, "\n".as_ptr() as _, 1);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

fn rust_main() -> Result<(), i32> {
    let mut buf: [u8; 32] = [0; 32];

    let mut emoji_buf = [0; 8];
    loop {
        unsafe {
            libc::fgets(buf.as_mut_ptr() as *mut _, 32, stdin);
        }
        let end = buf.iter().position(|b| *b == 0).ok_or(1)?;
        let s = unsafe { core::slice::from_raw_parts(buf.as_ptr(), end - 1) };
        let s = core::str::from_utf8(s).map_err(|_| 1)?;

        if s == ".exit" {
            break;
        }

        match maximally_compressed_gh_shortcodes::lookup(s, &mut emoji_buf) {
            Some(e) => print_str(e),
            None => print_str("not found"),
        }
    }

    Ok(())
}

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    if let Err(e) = rust_main() {
        return e as isize;
    }

    0
}
