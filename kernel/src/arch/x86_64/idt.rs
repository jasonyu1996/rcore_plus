use lazy_static::lazy_static;
use x86_64::structures::idt::*;
use crate::syscall::custom::user_interrupt_handlers;
use crate::process::process;
use spin::RwLock;

pub unsafe fn init() {
    IDT.load();
}


static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub unsafe fn idt_fill() {
    use crate::arch::interrupt::consts::*;
    use crate::arch::gdt::DOUBLE_FAULT_IST_INDEX;
    use x86_64::PrivilegeLevel;
    use core::mem::transmute;


        // 这里主要利用了x86_64库提供的IDT结构
        // 它进行了完善的封装，有强类型约束
        // 然而这里我们需要绕过一些限制，例如：
        // * 依赖于 "x86-interrupt" 函数ABI，而我们的是裸函数
        // * 某些保留中断号不允许设置，会触发panic
        // 于是下面用了一些trick绕过了它们

    let ring3 = [Syscall32];

    let entries = unsafe{ &mut *(&mut IDT as *mut _ as *mut [Entry<HandlerFunc>; 256]) };
    for i in 0..256 {
        let opt = entries[i].set_handler_fn(unsafe { transmute(__vectors[i]) });
        if ring3.contains(&(i as u8)) {
            opt.set_privilege_level(PrivilegeLevel::Ring3);
            // opt.disable_interrupts(false);
        }
        opt.disable_interrupts(true);
        if i == DoubleFault as usize {
            unsafe{ opt.set_stack_index(DOUBLE_FAULT_IST_INDEX as u16); }
        }
    }
}

pub fn enable_user_interrupts() {
    use core::mem::transmute;
    use x86_64::PrivilegeLevel;
    let proc = process();
    if proc.pid.get() == 0 {
        return;
    }
    unsafe {
        for (int_id, user_interrupt_entry) in user_interrupt_handlers.iter().enumerate() {
            if user_interrupt_entry.0 == proc.pid.get() {
                let opt = IDT[int_id].set_handler_fn(unsafe { transmute(user_interrupt_entry.1) });
                opt.set_privilege_level(PrivilegeLevel::Ring3);
                opt.disable_interrupts(false);
                // TODO: check out whether the stack addr needs to be reset
            }
        }
        IDT.load();
    }
}

pub fn disable_user_interrupts() {
    use crate::arch::interrupt::consts::*;
    use x86_64::PrivilegeLevel;
    use core::mem::transmute;

    let proc = process();
    if proc.pid.get() == 0 {
        return;
    }
    unsafe {
        for (int_id, user_interrupt_entry) in user_interrupt_handlers.iter().enumerate() {
            if user_interrupt_entry.0 == proc.pid.get() {
                // delegate to the kernel
                let opt = IDT[int_id].set_handler_fn(unsafe { transmute(__vectors[int_id]) });
                opt.set_privilege_level(
                    if(int_id as u8 == Syscall32){
                        PrivilegeLevel::Ring3
                    } else{
                        PrivilegeLevel::Ring0
                    }
                );
                opt.disable_interrupts(true);
            }
        }
        IDT.load();
    }
}

extern "C" {
    /// 中断向量表
    /// 符号定义在 [trap.asm](boot/trap.asm)
    //noinspection RsStaticConstNaming
    static __vectors: [extern "C" fn(); 256];
}
