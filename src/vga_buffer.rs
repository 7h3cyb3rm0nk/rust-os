use volatile::Volatile;
use core::fmt;



#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color { //colors for vga buffer as u8
    Black =       0,
    Blue =        1,
    Green =       2,
    Cyan =        3,
    Red =         4,
    Magenta =     5,
    Brown =       6,
    LightGray =   7,
    DarkGray =    8,
    LightBlue =   9,
    LightGreen = 10,
    LightCyan =  11,
    LightRed =   12,
    Pink =       13,
    Yellow =     14,
    White =      15,

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
//repr(transparent) for using the memory
//layout of the inner field which is u8
struct ColorCode(u8); // for generating color code for each character

impl ColorCode{ // calculates the color code : lower 4 bits foreground higher 4 bits background
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
// repr(C) will make the struct exactly
// 2 bytes to store in the Buffer Array
struct ScreenChar {  // struct for character in the buffer with 
                     // two fields ascii character and it's color code
    ascii_character: u8,
    color_code: ColorCode,
}

//declaring constants for the width and height of vga buffer
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;


// declaring a struct to represent the vga buffer as a 2d array 
// of BUFFER_HEIGHT rows and BUFFER_WIDTH columns 
// repr(transparent) to use memory layout of inner fields
// to replicate c like 2d array because
//ScreenChar also use c like memory layout
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}                                                                 // using Volatile<ScreenChar> for 
                                                                  // volatile writes and avoid
                                                                  // compiler optimization



// for writing to the vga_buffer

pub struct Writer {
    column_position: usize, // current position of the writer
    color_code: ColorCode, // color code
    buffer: &'static mut Buffer, // reference to vga_buffer

}



impl Writer {
    // function to write a single byte to vga buffer
    pub fn write_byte(&mut self, byte: u8) {

        match byte {

            b'\n' => self.new_line(),

            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                // calculate the row and column in the buffer

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                // writes to Buffer position using write() from Volatile
                // this is done rather than using an 
                // assignment to Buffer so that compiler
                // doesn't do optimizations and 
                // skip the write operation
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,

                });

                self.column_position += 1;
            }

            }
        }

    // moves the characters up a new line
    
    pub fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read(); // reads the characters from
                                                                    // buffer[row][col]
                self.buffer.chars[row-1][col].write(character); // writes the characters from the
                                                                 // lower row to the upper row
            }                                                    //using
                                                                 //volatile::Volatile::write() and
                                                                 //read() here
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    pub fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
            
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank); // clears all the characters to blank
        }

    }
     // write a string to buffer
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                //printable ascii bytes or new_line
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // for non printable ascii
                _ => self.write_byte(0xfe)
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}



pub fn print_something() {
    use core::fmt::Write;
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe {
            &mut *(0xb8000 as *mut Buffer)
             },                            // this may seem complicated
                                           // 0xb8000 is cast as a mutable pointer to a Buffer
                                           // struct and this pointer is dereferenced and converted
                                           // to a mutable reference 
                                           // using unsafe {} for the rust compiler 
                                           // to not generate error
                                           // Note: so when we dereference we get the memory address of
                                           // vga buffer inside the Buffer Struct
        
    };

    // writer.write_string("Rust-OS An Operating System built on Rust Programming Language");
    write!(&mut writer, "Rust-OS An Operating System built on Rust Programming Language ~ Version 0.1.0").unwrap();//using write! macro
    for _i in 0..10 {
    writer.new_line();
    }
}


// creating a global interface for usage from
// other modules
//
pub static WRITER: Writer = Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::Yellow, Color::Black),
    buffer: unsafe{ &mut *(0xb800 as *mut Buffer)},

};







