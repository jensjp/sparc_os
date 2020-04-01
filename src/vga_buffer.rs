

const BUFFER_WIDTH : usize = 80;
const BUFFER_HEIGHT : usize = 25;

// The colours of the VGA buffer only the first 4 bits required 
#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15
}

// represents the color of one char includes both fg and bg
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {

   pub fn new(fg : Color, bg : Color) -> ColorCode {
       ColorCode( (bg as u8) << 4 | (fg as u8) )
   }

}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(C)]
struct ScreenChar{
    ascii_char: u8,
    color_code: ColorCode
}

#[repr(transparent)]
struct Buffer{
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer
}

impl Writer {

    pub fn write_byte(&mut self, byte : u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1;
                let column = self.column_position;
                self.buffer.chars[row][column] = 
                ScreenChar { ascii_char: byte,
                      color_code: self.color_code 
                    }; 
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, string :&str){
        for byte in string.bytes(){
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn new_line(&mut self){
        /* TODO */
    }

}

pub fn print_something(){
    let mut writer = Writer{
        column_position: 0,
        color_code: ColorCode::new(Color::LightGreen, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }
    };

    writer.write_byte(b'J');
    writer.write_byte(b'\n');
    writer.write_string("Jens J Parappallil")
}