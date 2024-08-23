//PRINTER
//Manages text output by directly writing to VGA video memory

use core::arch::asm;

//Warning! Mutable static here
//TODO: Implement a mutex to get safe access to this
pub static mut PRINTER: Printer = Printer {
    x: 0,
    y: 0,
    foreground: 0x7,
    background: 0,
};

const WIDTH: u16 = 80;
const HEIGHT: u16 = 25;

const VGA_START: u32 = 0x000b8000;

pub const COLOR_BLACK: u8 = 0x0;
pub const COLOR_BLUE: u8 = 0x1;
pub const COLOR_GREEN: u8 = 0x2;
pub const COLOR_CYAN: u8 = 0x3;
pub const COLOR_RED: u8 = 0x4;
pub const COLOR_MAGENTA: u8 = 0x5;
pub const COLOR_YELLOW: u8 = 0x6;
pub const COLOR_WHITE: u8 = 0x7;
pub const COLOR_LIGHT_BLACK: u8 = 0x8;
pub const COLOR_LIGHT_BLUE: u8 = 0x9;
pub const COLOR_LIGHT_GREEN: u8 = 0xA;
pub const COLOR_LIGHT_CYAN: u8 = 0xB;
pub const COLOR_LIGHT_RED: u8 = 0xC;
pub const COLOR_LIGHT_MAGENTA: u8 = 0xD;
pub const COLOR_LIGHT_YELLOW: u8 = 0xE;
pub const COLOR_LIGHT_WHITE: u8 = 0xF;

pub struct Printer {
    x: u16,
    y: u16,
    foreground: u8,
    background: u8,
}

impl Printer {
    //copy given char to memory pointed to vga_pointer
    pub fn printc(&mut self, c: char) {
        //e9 port hack
        unsafe {
            asm!("out dx, al", in("dx") 0xe9 as u16, in("al") c as u8);
        }

        if c == '\n' {
            self.new_line();
            return;
        }

        //calculate target from coords
        let target = (VGA_START + ((self.y * WIDTH + self.x) * 2) as u32) as *mut u8;

        unsafe {
            if self.y >= HEIGHT - 1 && self.x >= WIDTH - 1 {
                *target = c as u8;
                *target.byte_add(1) = self.background << 4 | self.foreground;

                self.scroll();
                self.x = 0;
            } else {
                *target = c as u8;
                *target.byte_add(1) = self.background << 4 | self.foreground;

                self.x += 1;
                if self.x >= WIDTH {
                    self.x = 0;
                    self.y += 1;
                }
            }
        }

        self.set_cursor_position();
    }

    //print a string by printing one char at the time
    pub fn prints(&mut self, s: &str) {
        //set coords to current cursor position
        let cursor = self.get_cursor_position();
        self.x = cursor.0;
        self.y = cursor.1;

        for c in s.chars() {
            self.printc(c);
        }

        //set cursors position to new coords
        self.set_cursor_position();
    }

    pub fn delete(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        } else if self.y > 0 {
            self.y -= 1;
            self.x = WIDTH - 1;
        } else {
            return;
        }

        let target: *mut u8 = (VGA_START + ((self.y * WIDTH + self.x) * 2) as u32) as *mut u8;

        unsafe {
            *target = b' ' as u8;
            *target.byte_add(1) = self.background << 4 | self.foreground;
        }

        self.set_cursor_position();
    }

    //get cursor position directly talking to vga hardware
    pub fn get_cursor_position(&self) -> (u16, u16) {
        let mut index: u16 = 0;

        unsafe {
            asm!("out dx, al", in("dx") 0x3d4 as u16, in("al") 0x0f as u8);
            let mut a: u8;
            asm!("in al, dx", out("al") a, in("dx") 0x3d5);

            index |= a as u16;

            asm!("out dx, al", in("dx") 0x3d4 as u16, in("al") 0x0e as u8);
            let b: u8;
            asm!("in al, dx", out("al") b, in("dx") 0x3d5);

            index |= (b as u16) << 8;
        }

        let x: u16 = index % WIDTH;
        let y: u16 = index / WIDTH;

        (x, y)
    }

    //set cursor position directly talking to vga hardware
    pub fn set_cursor_position(&self) {
        let index: u16 = self.y * WIDTH + self.x;

        unsafe {
            asm!("out dx, al", in("dx") 0x3d4 as u16, in("al") 0x0f as u8);
            asm!("out dx, al", in("dx") 0x3d5 as u16, in("al") (index & 0xff) as u8);
            asm!("out dx, al", in("dx") 0x3d4 as u16, in("al") 0x0e as u8);
            asm!("out dx, al", in("dx") 0x3d5 as u16, in("al") ((index >> 8) & 0xff) as u8);
        }
    }

    //copy content of each row to upper row
    pub fn scroll(&mut self) {
        for a in 0..HEIGHT {
            for i in (WIDTH * a)..((WIDTH * a) + WIDTH) {
                let new = (VGA_START + (i * 2) as u32) as *mut u8;
                let old = (VGA_START + ((i + WIDTH) * 2) as u32) as *const u8;

                unsafe {
                    *new = *old;
                    *new.byte_add(1) = *old.byte_add(1);
                }
            }
        }
    }

    pub fn set_colors(&mut self, foreground: u8, background: u8) {
        self.foreground = foreground;
        self.background = background;
    }

    pub fn reset_colors(&mut self) {
        self.set_colors(COLOR_WHITE, COLOR_BLACK)
    }

    pub fn new_line(&mut self) {
        if self.y == HEIGHT - 1 {
            self.scroll();
        } else {
            self.y += 1;
        }

        self.x = 0;

        self.set_cursor_position();
    }

    pub fn clear(&mut self) {
        self.x = 0;
        self.y = 0;

        for i in 0..(WIDTH * HEIGHT) {
            let target: *mut u8 = (VGA_START + (i * 2) as u32) as *mut u8;
            unsafe {
                *target = b' ' as u8;
                *target.byte_add(1) = self.background << 4 | self.foreground;
            }
        }

        self.set_cursor_position();
    }
}