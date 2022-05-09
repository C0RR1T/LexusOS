#![no_std]
#![no_main]




mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello World!").unwrap();
    write!(vga_buffer::WRITER.lock(), "Hello again! This is number {}", 5).unwrap();
    loop {}
}


#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}