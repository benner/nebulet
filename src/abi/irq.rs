use core::{ptr, mem};
use x86_64::structures::idt::ExceptionStackFrame;
use arch::idt;
use arch::devices::pic;

use wasm::instance::VmCtx;

type Handler = unsafe extern fn(*const VmCtx);

/// This has a "side effect" of enabling interrupts... ugh.
pub fn set_irq_handler(index: u32, handler: u32, vmctx: &VmCtx) {
    if index >= 256 {
        return;
    }

    let index = index as usize;
    let data = vmctx.data();
    let process = &data.process;
    let table = &process.instance().read().tables;

    let func_addr = table[0][handler as usize];

    unsafe {
        let handler = mem::transmute(func_addr);

        IDT_REDIRECTION_TABLE[index] = (handler, vmctx);
        let handler = IDT_HANDLER[index];
        let mut guard = idt::IdtGuard::new();
        guard[index].set_handler_fn(handler);

        let irq = index as u8 - pic::MASTER_OFFSET;
        if irq < 8 {
            pic::MASTER.mask_clear(irq);
        } else {
            pic::SLAVE.mask_clear(irq);
        }
    }
}


unsafe extern fn null_handler(_: *const VmCtx) {}

static mut IDT_REDIRECTION_TABLE: [(Handler, *const VmCtx); 256] = [(null_handler, ptr::null()); 256];

macro_rules! idt_handlers {
    ($($name:ident ( $value:expr) ),*) => {
        [$( {
                extern "x86-interrupt" fn $name(_: &mut ExceptionStackFrame) {
                    unsafe {
                        pic::MASTER.ack();
                        let entry = IDT_REDIRECTION_TABLE[$value];
                        entry.0(entry.1);
                    }
                }
                $name
            }
         ),*
        ]
    }
}

static IDT_HANDLER: [extern "x86-interrupt" fn(&mut ExceptionStackFrame); 256] = idt_handlers! {
    idt_handler_0x00 ( 0x00 ),
    idt_handler_0x01 ( 0x01 ),
    idt_handler_0x02 ( 0x02 ),
    idt_handler_0x03 ( 0x03 ),
    idt_handler_0x04 ( 0x04 ),
    idt_handler_0x05 ( 0x05 ),
    idt_handler_0x06 ( 0x06 ),
    idt_handler_0x07 ( 0x07 ),
    idt_handler_0x08 ( 0x08 ),
    idt_handler_0x09 ( 0x09 ),
    idt_handler_0x0a ( 0x0a ),
    idt_handler_0x0b ( 0x0b ),
    idt_handler_0x0c ( 0x0c ),
    idt_handler_0x0d ( 0x0d ),
    idt_handler_0x0e ( 0x0e ),
    idt_handler_0x0f ( 0x0f ),
    idt_handler_0x10 ( 0x10 ),
    idt_handler_0x11 ( 0x11 ),
    idt_handler_0x12 ( 0x12 ),
    idt_handler_0x13 ( 0x13 ),
    idt_handler_0x14 ( 0x14 ),
    idt_handler_0x15 ( 0x15 ),
    idt_handler_0x16 ( 0x16 ),
    idt_handler_0x17 ( 0x17 ),
    idt_handler_0x18 ( 0x18 ),
    idt_handler_0x19 ( 0x19 ),
    idt_handler_0x1a ( 0x1a ),
    idt_handler_0x1b ( 0x1b ),
    idt_handler_0x1c ( 0x1c ),
    idt_handler_0x1d ( 0x1d ),
    idt_handler_0x1e ( 0x1e ),
    idt_handler_0x1f ( 0x1f ),
    idt_handler_0x20 ( 0x20 ),
    idt_handler_0x21 ( 0x21 ),
    idt_handler_0x22 ( 0x22 ),
    idt_handler_0x23 ( 0x23 ),
    idt_handler_0x24 ( 0x24 ),
    idt_handler_0x25 ( 0x25 ),
    idt_handler_0x26 ( 0x26 ),
    idt_handler_0x27 ( 0x27 ),
    idt_handler_0x28 ( 0x28 ),
    idt_handler_0x29 ( 0x29 ),
    idt_handler_0x2a ( 0x2a ),
    idt_handler_0x2b ( 0x2b ),
    idt_handler_0x2c ( 0x2c ),
    idt_handler_0x2d ( 0x2d ),
    idt_handler_0x2e ( 0x2e ),
    idt_handler_0x2f ( 0x2f ),
    idt_handler_0x30 ( 0x30 ),
    idt_handler_0x31 ( 0x31 ),
    idt_handler_0x32 ( 0x32 ),
    idt_handler_0x33 ( 0x33 ),
    idt_handler_0x34 ( 0x34 ),
    idt_handler_0x35 ( 0x35 ),
    idt_handler_0x36 ( 0x36 ),
    idt_handler_0x37 ( 0x37 ),
    idt_handler_0x38 ( 0x38 ),
    idt_handler_0x39 ( 0x39 ),
    idt_handler_0x3a ( 0x3a ),
    idt_handler_0x3b ( 0x3b ),
    idt_handler_0x3c ( 0x3c ),
    idt_handler_0x3d ( 0x3d ),
    idt_handler_0x3e ( 0x3e ),
    idt_handler_0x3f ( 0x3f ),
    idt_handler_0x40 ( 0x40 ),
    idt_handler_0x41 ( 0x41 ),
    idt_handler_0x42 ( 0x42 ),
    idt_handler_0x43 ( 0x43 ),
    idt_handler_0x44 ( 0x44 ),
    idt_handler_0x45 ( 0x45 ),
    idt_handler_0x46 ( 0x46 ),
    idt_handler_0x47 ( 0x47 ),
    idt_handler_0x48 ( 0x48 ),
    idt_handler_0x49 ( 0x49 ),
    idt_handler_0x4a ( 0x4a ),
    idt_handler_0x4b ( 0x4b ),
    idt_handler_0x4c ( 0x4c ),
    idt_handler_0x4d ( 0x4d ),
    idt_handler_0x4e ( 0x4e ),
    idt_handler_0x4f ( 0x4f ),
    idt_handler_0x50 ( 0x50 ),
    idt_handler_0x51 ( 0x51 ),
    idt_handler_0x52 ( 0x52 ),
    idt_handler_0x53 ( 0x53 ),
    idt_handler_0x54 ( 0x54 ),
    idt_handler_0x55 ( 0x55 ),
    idt_handler_0x56 ( 0x56 ),
    idt_handler_0x57 ( 0x57 ),
    idt_handler_0x58 ( 0x58 ),
    idt_handler_0x59 ( 0x59 ),
    idt_handler_0x5a ( 0x5a ),
    idt_handler_0x5b ( 0x5b ),
    idt_handler_0x5c ( 0x5c ),
    idt_handler_0x5d ( 0x5d ),
    idt_handler_0x5e ( 0x5e ),
    idt_handler_0x5f ( 0x5f ),
    idt_handler_0x60 ( 0x60 ),
    idt_handler_0x61 ( 0x61 ),
    idt_handler_0x62 ( 0x62 ),
    idt_handler_0x63 ( 0x63 ),
    idt_handler_0x64 ( 0x64 ),
    idt_handler_0x65 ( 0x65 ),
    idt_handler_0x66 ( 0x66 ),
    idt_handler_0x67 ( 0x67 ),
    idt_handler_0x68 ( 0x68 ),
    idt_handler_0x69 ( 0x69 ),
    idt_handler_0x6a ( 0x6a ),
    idt_handler_0x6b ( 0x6b ),
    idt_handler_0x6c ( 0x6c ),
    idt_handler_0x6d ( 0x6d ),
    idt_handler_0x6e ( 0x6e ),
    idt_handler_0x6f ( 0x6f ),
    idt_handler_0x70 ( 0x70 ),
    idt_handler_0x71 ( 0x71 ),
    idt_handler_0x72 ( 0x72 ),
    idt_handler_0x73 ( 0x73 ),
    idt_handler_0x74 ( 0x74 ),
    idt_handler_0x75 ( 0x75 ),
    idt_handler_0x76 ( 0x76 ),
    idt_handler_0x77 ( 0x77 ),
    idt_handler_0x78 ( 0x78 ),
    idt_handler_0x79 ( 0x79 ),
    idt_handler_0x7a ( 0x7a ),
    idt_handler_0x7b ( 0x7b ),
    idt_handler_0x7c ( 0x7c ),
    idt_handler_0x7d ( 0x7d ),
    idt_handler_0x7e ( 0x7e ),
    idt_handler_0x7f ( 0x7f ),
    idt_handler_0x80 ( 0x80 ),
    idt_handler_0x81 ( 0x81 ),
    idt_handler_0x82 ( 0x82 ),
    idt_handler_0x83 ( 0x83 ),
    idt_handler_0x84 ( 0x84 ),
    idt_handler_0x85 ( 0x85 ),
    idt_handler_0x86 ( 0x86 ),
    idt_handler_0x87 ( 0x87 ),
    idt_handler_0x88 ( 0x88 ),
    idt_handler_0x89 ( 0x89 ),
    idt_handler_0x8a ( 0x8a ),
    idt_handler_0x8b ( 0x8b ),
    idt_handler_0x8c ( 0x8c ),
    idt_handler_0x8d ( 0x8d ),
    idt_handler_0x8e ( 0x8e ),
    idt_handler_0x8f ( 0x8f ),
    idt_handler_0x90 ( 0x90 ),
    idt_handler_0x91 ( 0x91 ),
    idt_handler_0x92 ( 0x92 ),
    idt_handler_0x93 ( 0x93 ),
    idt_handler_0x94 ( 0x94 ),
    idt_handler_0x95 ( 0x95 ),
    idt_handler_0x96 ( 0x96 ),
    idt_handler_0x97 ( 0x97 ),
    idt_handler_0x98 ( 0x98 ),
    idt_handler_0x99 ( 0x99 ),
    idt_handler_0x9a ( 0x9a ),
    idt_handler_0x9b ( 0x9b ),
    idt_handler_0x9c ( 0x9c ),
    idt_handler_0x9d ( 0x9d ),
    idt_handler_0x9e ( 0x9e ),
    idt_handler_0x9f ( 0x9f ),
    idt_handler_0xa0 ( 0xa0 ),
    idt_handler_0xa1 ( 0xa1 ),
    idt_handler_0xa2 ( 0xa2 ),
    idt_handler_0xa3 ( 0xa3 ),
    idt_handler_0xa4 ( 0xa4 ),
    idt_handler_0xa5 ( 0xa5 ),
    idt_handler_0xa6 ( 0xa6 ),
    idt_handler_0xa7 ( 0xa7 ),
    idt_handler_0xa8 ( 0xa8 ),
    idt_handler_0xa9 ( 0xa9 ),
    idt_handler_0xaa ( 0xaa ),
    idt_handler_0xab ( 0xab ),
    idt_handler_0xac ( 0xac ),
    idt_handler_0xad ( 0xad ),
    idt_handler_0xae ( 0xae ),
    idt_handler_0xaf ( 0xaf ),
    idt_handler_0xb0 ( 0xb0 ),
    idt_handler_0xb1 ( 0xb1 ),
    idt_handler_0xb2 ( 0xb2 ),
    idt_handler_0xb3 ( 0xb3 ),
    idt_handler_0xb4 ( 0xb4 ),
    idt_handler_0xb5 ( 0xb5 ),
    idt_handler_0xb6 ( 0xb6 ),
    idt_handler_0xb7 ( 0xb7 ),
    idt_handler_0xb8 ( 0xb8 ),
    idt_handler_0xb9 ( 0xb9 ),
    idt_handler_0xba ( 0xba ),
    idt_handler_0xbb ( 0xbb ),
    idt_handler_0xbc ( 0xbc ),
    idt_handler_0xbd ( 0xbd ),
    idt_handler_0xbe ( 0xbe ),
    idt_handler_0xbf ( 0xbf ),
    idt_handler_0xc0 ( 0xc0 ),
    idt_handler_0xc1 ( 0xc1 ),
    idt_handler_0xc2 ( 0xc2 ),
    idt_handler_0xc3 ( 0xc3 ),
    idt_handler_0xc4 ( 0xc4 ),
    idt_handler_0xc5 ( 0xc5 ),
    idt_handler_0xc6 ( 0xc6 ),
    idt_handler_0xc7 ( 0xc7 ),
    idt_handler_0xc8 ( 0xc8 ),
    idt_handler_0xc9 ( 0xc9 ),
    idt_handler_0xca ( 0xca ),
    idt_handler_0xcb ( 0xcb ),
    idt_handler_0xcc ( 0xcc ),
    idt_handler_0xcd ( 0xcd ),
    idt_handler_0xce ( 0xce ),
    idt_handler_0xcf ( 0xcf ),
    idt_handler_0xd0 ( 0xd0 ),
    idt_handler_0xd1 ( 0xd1 ),
    idt_handler_0xd2 ( 0xd2 ),
    idt_handler_0xd3 ( 0xd3 ),
    idt_handler_0xd4 ( 0xd4 ),
    idt_handler_0xd5 ( 0xd5 ),
    idt_handler_0xd6 ( 0xd6 ),
    idt_handler_0xd7 ( 0xd7 ),
    idt_handler_0xd8 ( 0xd8 ),
    idt_handler_0xd9 ( 0xd9 ),
    idt_handler_0xda ( 0xda ),
    idt_handler_0xdb ( 0xdb ),
    idt_handler_0xdc ( 0xdc ),
    idt_handler_0xdd ( 0xdd ),
    idt_handler_0xde ( 0xde ),
    idt_handler_0xdf ( 0xdf ),
    idt_handler_0xe0 ( 0xe0 ),
    idt_handler_0xe1 ( 0xe1 ),
    idt_handler_0xe2 ( 0xe2 ),
    idt_handler_0xe3 ( 0xe3 ),
    idt_handler_0xe4 ( 0xe4 ),
    idt_handler_0xe5 ( 0xe5 ),
    idt_handler_0xe6 ( 0xe6 ),
    idt_handler_0xe7 ( 0xe7 ),
    idt_handler_0xe8 ( 0xe8 ),
    idt_handler_0xe9 ( 0xe9 ),
    idt_handler_0xea ( 0xea ),
    idt_handler_0xeb ( 0xeb ),
    idt_handler_0xec ( 0xec ),
    idt_handler_0xed ( 0xed ),
    idt_handler_0xee ( 0xee ),
    idt_handler_0xef ( 0xef ),
    idt_handler_0xf0 ( 0xf0 ),
    idt_handler_0xf1 ( 0xf1 ),
    idt_handler_0xf2 ( 0xf2 ),
    idt_handler_0xf3 ( 0xf3 ),
    idt_handler_0xf4 ( 0xf4 ),
    idt_handler_0xf5 ( 0xf5 ),
    idt_handler_0xf6 ( 0xf6 ),
    idt_handler_0xf7 ( 0xf7 ),
    idt_handler_0xf8 ( 0xf8 ),
    idt_handler_0xf9 ( 0xf9 ),
    idt_handler_0xfa ( 0xfa ),
    idt_handler_0xfb ( 0xfb ),
    idt_handler_0xfc ( 0xfc ),
    idt_handler_0xfd ( 0xfd ),
    idt_handler_0xfe ( 0xfe ),
    idt_handler_0xff ( 0xff )
};