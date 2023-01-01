#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;

use alloc::string::String;

extern crate nx;
use nx::svc;
use nx::result::*;
use nx::util;
use nx::diag::abort;
use nx::diag::log::lm::LmLogger;
use nx::gpu;

use core::panic;

extern crate ui2d;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[no_mangle]
pub fn initialize_heap(hbl_heap: util::PointerAndSize) -> util::PointerAndSize {
    if hbl_heap.is_valid() {
        hbl_heap
    }
    else {
        let heap_size: usize = 0x10000000;
        let heap_address = svc::set_heap_size(heap_size).unwrap();
        util::PointerAndSize::new(heap_address, heap_size)
    }
}

#[no_mangle]
pub fn main() -> Result<()> {
    let mut gpu_ctx = gpu::Context::new(gpu::NvDrvServiceKind::Applet, gpu::ViServiceKind::System, 0x800000)?;

    let color_fmt = gpu::ColorFormat::A8B8G8R8;
    

    let c_white = ui2d::RGBA8::new_rgb(0xFF, 0xFF, 0xFF);
    let c_black = ui2d::RGBA8::new_rgb(0, 0, 0);
    
    let mut surface = ui2d::SurfaceEx::from(gpu_ctx.create_stray_layer_surface("Default", 2, color_fmt, gpu::PixelFormat::RGBA_8888, gpu::Layout::BlockLinear)?);

    let result = unsafe{
        multiply(2, 5)
    };

    loop {

        surface.start()?;

        let msg = String::from(&format!(" Hello world from aarch64-switch-rs!\n\n Multiply 2*5 : {}", result));
        
        surface.clear(c_white);
        surface.draw_bitmap_text(msg, c_black, 2, 10, 250, true);

        surface.end()?;
    }
}

#[panic_handler]
fn panic_handler(info: &panic::PanicInfo) -> ! {
    util::simple_panic_handler::<LmLogger>(info, abort::AbortLevel::FatalThrow())
}
