//GLOBAL DESCRIPTOR TABLE
//Tells the CPU about memory segment
//The default here is a flat memory model,
//meaning there are only two big segments relative to the whole 4GB memory,
//one for data, one for code, the first entry is always zero, so three entries in total.

use core::arch::asm;
use core::mem::size_of;

const GDT_ENTRIES: usize = 3;

pub static GDT: GlobalDescriptorTable = {
    //segment length (0xffff means all 32bit memory)
    let limit = {
        let limit_low = 0xffff << 0;
        let limit_high = 0xf << 48;

        limit_low | limit_high
    };

    //base address
    let base = {
        let base_low = 0x0000 << 16;
        let base_high = 0x00 << 56;

        base_low | base_high
    };

    //access byte
    let access = {
        let p = 0b1 << 47; //present bit (1 for any segment)
        let dpl = 0b00 << 46; //descriptor privilege level (ring, 0 for highest privilege, 3 for lowest)
        let s = 0b1 << 44; //descriptor type bit
        let e = 0b0 << 43; //executable bit
        let dc = 0b0 << 42; //direction bit/conforming bit
        let rw = 0b1 << 41; //readable bit/writable bit
        let a = 0b0 << 40; //accessed bit

        p | dpl | s | e | dc | rw | a
    };

    //flags
    let flags = {
        let g = 0b1 << 55; //granularity flag
        let db = 0b1 << 54; //size flag
        let l = 0b0 << 53; //long mode flag
        let r = 0b0 << 52; //reserved

        g | db | l | r
    };

    let executable = 0b1 << 43; //set only executable flag again, instead of setting all values again

    //first entry is always zero
    //second entry is code segment (default + executable)
    //third entry is data segment (default)
    let zero = GdtEntry { entry: 0 };
    let code = GdtEntry {
        entry: limit | base | access | flags | executable,
    };
    let data = GdtEntry {
        entry: limit | base | access | flags,
    };

    GlobalDescriptorTable {
        entries: [zero, code, data],
    }
};

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct GdtEntry {
    entry: u64,
}

#[repr(C, packed)]
pub struct GlobalDescriptorTable {
    entries: [GdtEntry; GDT_ENTRIES],
}

#[repr(C, packed)]
pub struct GdtDescriptor {
    size: u16,                            //gdt size
    offset: *const GlobalDescriptorTable, //pointer to gdt
}

//global descriptor table for flat memory model
impl GlobalDescriptorTable {
    //load gdt using lgdt instruction
    pub fn load(&self) {
        let descriptor = GdtDescriptor {
            size: (GDT_ENTRIES * size_of::<GdtEntry>() - 1) as u16, //calculate size of gdt
            offset: self,                                           //pointer to gdt
        };

        unsafe {
            asm!("lgdt [{0:e}]", in(reg) &descriptor);
        }
    }
}
