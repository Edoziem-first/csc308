#![no_std]
#![no_main]
mod writer;
use writer::FrameBufferWriter;
use core::fmt::Write;

use bootloader_api::config::Mapping;
use x86_64::instructions::hlt;
//Use the entry_point macro to register the entry point function:
// bootloader_api::entry_point!(kernel_main)
//optionally pass a custom config

//print macro
macro_rules! print { 
    ($writer:expr, $fmt:expr $(, $args:expr)* ) => {{
        use core::fmt::Write;
        write!($writer, $fmt $(, $args)*).unwrap();
    }};
}

pub static BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};
bootloader_api::entry_point!(my_entry_point, config =
&BOOTLOADER_CONFIG);

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        hlt();
    }
}


fn my_entry_point(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    let frame_buffer_info = boot_info.framebuffer.as_mut().unwrap().info();
    let buffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut(); let mut frame_buffer_writer =
    FrameBufferWriter::new(buffer, frame_buffer_info);
    frame_buffer_writer.set_cursor_position(100, 50); //function to allow user manually set cursor position
    //testing the escape sequence and color changing text
    print!(frame_buffer_writer, "Hello, world!\nThis is a test.\n\\cBlue Text\tIndented");


// this was to test the custom position logic
  // // Set the cursor position to (100, 50)
  // frame_buffer_writer.set_cursor_position(100, 50);

  // // Write text starting from the custom position
  // writeln!(
  //     frame_buffer_writer,
  //     "Hello from a custom position!"
  // )
  // .unwrap();






    //this was to test the handling of overflow and scroll logic
//     frame_buffer_writer.set_cursor(0, 100);
// writeln!(frame_buffer_writer, "Lorem Ipsum comes from a latin text written in 45BC by Roman statesman, lawyer, scholar, and philosopher, Marcus Tullius Cicero. The text is titled de Finibus Bonorum et Malorum which means The Extremes of Good and Evil. The most common form of Lorem ipsum is the following:

// Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.

// The text is a corrupted version of the original and therefore does not mean anything in particular. The book however where it originates discusses the philosophical views of Epicureanism, Stoicism, and the Platonism of Antiochus of Ascalon.

// Lorem ipsum is widely in use since the 14th century and up to today as the default dummy random text of the typesetting and web development industry. In fact not only it has survived the test of time but it thrived and can be found in many software products, from Microsoft Word to WordPress.").unwrap();



    loop {
        hlt();//stop x86_64 from being unnecessarily busy while looping
    }
}