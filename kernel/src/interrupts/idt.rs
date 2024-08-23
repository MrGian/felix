//INTERRUPT DESCRIPTOR TABLE

use crate::interrupts::exceptions;
use core::arch::asm;
use core::mem::size_of;

//Warning! Mutable static here
//TODO: Implement a mutex to get safe access to this
pub static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable {
    entries: [IDT_ENTRY; IDT_ENTRIES],
};

const IDT_ENTRIES: usize = 256;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct IdtEntry {
    offset_low: u16,       //lower 16 bits of handler func address
    segment_selector: u16, //segment selector of gdt entry
    reserved: u8,          //always zero
    flags: u8,             //entry flags
    offset_high: u16,      //higher 16 bits of handler func address
}

#[repr(C, packed)]
pub struct InterruptDescriptorTable {
    entries: [IdtEntry; IDT_ENTRIES],
}

#[repr(C, packed)]
pub struct IdtDescriptor {
    size: u16,                               //idt size
    offset: *const InterruptDescriptorTable, //pointer to idt
}

impl InterruptDescriptorTable {
    pub fn init(&mut self) {
        for i in 0..IDT_ENTRIES {
            self.add(i, exceptions::generic_handler as u32);
        }
    }

    pub fn add(&mut self, int: usize, handler: u32) {
        self.entries[int].set(handler);
    }

    //load idt using lidt instruction
    pub fn load(&self) {
        let descriptor = IdtDescriptor {
            size: (IDT_ENTRIES * size_of::<IdtEntry>() - 1) as u16, //calculate size of idt
            offset: self,                                           //pointer to idt
        };

        unsafe {
            asm!("lidt [{0:e}]", in(reg) &descriptor);
        }
    }

    //add exception handlers for various cpu exceptions
    pub fn add_exceptions(&mut self) {
        self.add(0x00, exceptions::division_error as u32);
        self.add(0x01, exceptions::debug_exception as u32);
        self.add(0x02, exceptions::mni_interrupt as u32);
        self.add(0x03, exceptions::breakpoint as u32);
        self.add(0x04, exceptions::overflow as u32);
        self.add(0x05, exceptions::bound_range_exceeded as u32);
        self.add(0x06, exceptions::invalid_opcode as u32);
        self.add(0x07, exceptions::device_not_available as u32);
        self.add(0x08, exceptions::double_fault as u32);
        self.add(0x09, exceptions::coprocessor_segment_overrun as u32);
        self.add(0x0A, exceptions::invalid_tss as u32);
        self.add(0x0B, exceptions::segment_not_present as u32);
        self.add(0x0C, exceptions::stack_segment_fault as u32);
        self.add(0x0D, exceptions::general_protection as u32);
        self.add(0x0E, exceptions::page_fault as u32);
        self.add(0x10, exceptions::x87_fpu_floating_point_error as u32);
        self.add(0x11, exceptions::alignment_check as u32);
        self.add(0x12, exceptions::machine_check as u32);
        self.add(0x13, exceptions::simd_floating_point_exception as u32);
        self.add(0x14, exceptions::virtualization_exception as u32);
        self.add(0x15, exceptions::control_protection_exception as u32);
    }
}

pub static IDT_ENTRY: IdtEntry = {
    let segment_selector: u16 = {
        //segment selector of gdt entry
        let rpl = 0b00 << 0; //ring privilege level (0 for ring 0)
        let ti = 0b0 << 2; //0 to use gdt, 1 to use ldt
        let index = 0b1 << 3; //bits 3-15 of gdt code entry, in my case 0x8 (0b1000)

        rpl | ti | index
    };

    let reserved: u8 = 0; //always zero

    let flags: u8 = {
        //entry flags
        let gate_type = 0xe << 0; //gate type, 0xe for 32bit interrupt gate, 0xf for 32bit trap gate
        let zero = 0 << 3; //always zero
        let dpl = 0 << 5; //ring allowed to use this interrupt
        let p = 1 << 7; //presence bit, 1 to enable

        gate_type | zero | dpl | p
    };

    IdtEntry {
        offset_low: 0,
        segment_selector: segment_selector,
        reserved: reserved,
        flags: flags,
        offset_high: 0,
    }
};

impl IdtEntry {
    pub fn set(&mut self, offset: u32) {
        self.offset_low = ((offset << 16) >> 16) as u16; //calculate lower 16 bits of offset
        self.offset_high = (offset >> 16) as u16; //calculate higher 16 bits of offset
    }
}
