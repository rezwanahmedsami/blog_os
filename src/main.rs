#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;
mod vga_buffer;
/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
// static TITLE: &[u8] = b"Rust OS - By Rezwan ahmed sami!!!";
// static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {

    // let vga_buffer = 0xb8000 as *mut u8;

    // for (i, &byte) in TITLE.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }

    // // write `Hello, World!` to the VGA buffer in 2nd line
    // let vga_buffer = 0xb8000 as *mut u8;
    // let mut hello = HELLO;
    // for i in 0..HELLO.len() {
    //     unsafe {
    //         *vga_buffer.offset((i + 80) as isize * 2) = hello[i];
    //         *vga_buffer.offset((i + 80) as isize * 2 + 1) = 0xb;
    //     }
    // }

    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    // write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

    println!("Hello World{}", "!");
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    loop {}
}