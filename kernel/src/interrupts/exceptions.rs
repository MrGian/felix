use core::arch::asm;

//CPU EXCEPTIONS HANDLERS

//handle excpetion based on interrupt number
#[no_mangle]
pub extern "C" fn exception_handler(int: u32, eip: u32, cs: u32, eflags: u32) {
    match int {
        0x00 => {
            libfelix::println!("DIVISION ERROR!");
        }
        0x01 => {
            libfelix::println!("DEBUG EXCEPTION!");
        }
        0x02 => {
            libfelix::println!("NMI INTERRUPT!");
        }
        0x03 => {
            libfelix::println!("BREAKPOINT!");
        }
        0x04 => {
            libfelix::println!("OVERFLOW!");
        }
        0x05 => {
            libfelix::println!("BOUND RANGE EXCEEDED!");
        }
        0x06 => {
            libfelix::println!("INVALID OPCODE (UNDEFINED OPCODE)!");
        }
        0x07 => {
            libfelix::println!("DEVICE NOT AVAILABLE (NO MATH COPROCESSOR)!");
        }
        0x08 => {
            libfelix::println!("DOUBLE FAULT!");
        }
        0x09 => {
            libfelix::println!("COPROCESSOR SEGMENT OVERRUN!");
        }
        0x0A => {
            libfelix::println!("INVALID TSS!");
        }
        0x0B => {
            libfelix::println!("SEGMENT NOT PRESENT!");
        }
        0x0C => {
            libfelix::println!("STACK-SEGMENT FAULT!");
        }
        0x0D => {
            libfelix::println!("GENERAL PROTECTION!");
        }
        0x0E => {
            libfelix::println!("PAGE FAULT!");
        }
        0x10 => {
            libfelix::println!("x87 FPU FLOATING-POINT ERROR (MATH ERROR)!");
        }
        0x11 => {
            libfelix::println!("ALIGNMENT CHECK!");
        }
        0x12 => {
            libfelix::println!("MACHINE CHECK!");
        }
        0x13 => {
            libfelix::println!("SIMD FLOATING-POINT EXCEPTION!");
        }
        0x14 => {
            libfelix::println!("VIRTUALIZATION EXCEPTION!");
        }
        0x15 => {
            libfelix::println!("CONTROL PROTECTION EXCEPTION!");
        }
        _ => {
            libfelix::println!("EXCEPTION!");
        }
    }
    libfelix::println!("EIP: {:X}, CS: {:X}, EFLAGS: {:b}", eip, cs, eflags);

    loop {}
}

#[naked]
pub extern "C" fn division_error() {
    unsafe {
        asm!(
            "push 0x00",
            "call exception_handler",
            "add esp, 4",
            "iretd",
            options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn debug_exception() {
    unsafe {
        asm!(
            "push 0x01",
            "call exception_handler",
            "add esp, 4",
            "iretd",
            options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn mni_interrupt() {
    unsafe {
        asm!(
            "push 0x02",
            "call exception_handler",
            "add esp, 4",
            "iretd",
            options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn breakpoint() {
    unsafe {
        asm!(
            "push 0x03",
            "call exception_handler",
            "add esp, 4",
            "iretd",
            options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn overflow() {
    unsafe {
        asm!(
            "push 0x04",
            "call exception_handler",
            "add esp, 4",
            "iretd",
            options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn bound_range_exceeded() {
    unsafe {
        asm!(
            "push 0x05",
            "call exception_handler",
            "add esp, 4",
            "iretd",
            options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn invalid_opcode() {
    unsafe {
        asm!(
            "push 0x06",
            "call exception_handler",
            "add esp, 4",
            "iretd",
            options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn device_not_available() {
    unsafe {
        asm!(
            "push 0x07",
            "call exception_handler",
            "add esp, 4",
            "iretd",
            options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn double_fault() {
    unsafe {
        asm!(
            "push 0x08",
            "call exception_handler",
            "add esp, 4",
            "iretd",
            options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn coprocessor_segment_overrun() {
    unsafe {
            asm!(
            "push 0x09",
            "call exception_handler",
            "add esp, 4",
            "iretd",
            options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn invalid_tss() {
    unsafe {
        asm!(
            "push 0x0A",
            "call exception_handler",
            "add esp, 4",
            "iretd",
            options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn segment_not_present() {
    unsafe {
        asm!(
            "push 0x0B",
            "call exception_handler",
            "add esp, 4",
            "iretd",
            options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn stack_segment_fault() {
    unsafe {
        asm!(
            "push 0x0C",
            "call exception_handler",
            "add esp, 4",
            "iretd",
            options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn general_protection() {
    unsafe {
        asm!(
            "push 0x0D",
            "call exception_handler",
            "add esp, 4",
            "iretd",
            options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn page_fault() {
    unsafe {
        asm!(
            "push 0x0E",
            "call exception_handler",
            "add esp, 4",
            "iretd",
            options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn x87_fpu_floating_point_error() {
    unsafe {
        asm!(
            "push 0x10",
            "call exception_handler",
            "add esp, 4",
            "iretd",
            options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn alignment_check() {
    unsafe {
        asm!(
            "push 0x11",
            "call exception_handler",
            "add esp, 4",
            "iretd",
            options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn machine_check() {
    unsafe {
        asm!(
            "push 0x12",
            "call exception_handler",
            "add esp, 4",
            "iretd",
            options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn simd_floating_point_exception() {
    unsafe {
        asm!(
            "push 0x13",
            "call exception_handler",
            "add esp, 4",
            "iretd",
            options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn virtualization_exception() {
    unsafe {
        asm!(
            "push 0x14",
            "call exception_handler",
            "add esp, 4",
            "iretd",
            options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn control_protection_exception() {
    unsafe {
        asm!(
            "push 0x15",
            "call exception_handler",
            "add esp, 4",
            "iretd",
            options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn generic_handler() {
    unsafe {
        asm!(
            "push 0xFF",
            "call exception_handler",
            "add esp, 4",
            "iretd",
            options(noreturn),
        );
    }
}