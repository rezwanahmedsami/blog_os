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
    

    // // array of some texts and sentances
    let texts = [
        "Hello World", 
        "My name is rezwan", 
        "I am a software engineer",
        "I am a rustacean",
        "I am a rust developer",
        "I am a web developer",
        "I am a full stack developer"
        
        ];
    for text in texts.iter() {
        println!("{}", text);
    }
    // print a big word "REZWAN AHMED SAMI" by special chars


    // println!("Hello World{}", "!");
    // println!("My name is rezwan");
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    loop {}
}