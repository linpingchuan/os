use crate::println;
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;
#[cfg(test)]
use crate::{serial_print,serial_println};

// 创建一个 idt 用于整个 cpu 在整个生命周期中，进行任意中断
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}
pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    println!("EXCEPITON: BREAKPOINT\n {:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception(){
    serial_print!("test_breakpoint_exception..");
    x86_64::instructions::interrupts::int3();
    serial_println!("[ok]");
}