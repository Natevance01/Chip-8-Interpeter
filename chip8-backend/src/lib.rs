use rand::random;

const RAM_SIZE: usize = 4096;
const SCREEN_WIDTH: usize = 32;
const SCREEN_HEIGHT: usize = 64;
const NUM_REGISTARS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;
const FONTSET_SIZE: usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
///Processor
#[allow(dead_code)]
pub struct Interpreter {
    pc: u16,
    ///program counter
    ram: [u8; RAM_SIZE],
    ///ram
    display: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_registars: [u8; NUM_REGISTARS],
    //points to location in RAM
    index_registar: u16,
    stack_pointer: u16,
    /// points to top of stack
    stack: [u16; STACK_SIZE],
    /// stack
    keys: [bool; NUM_KEYS],
    delay_timer: u8,
    sound_timer: u8,
}

const START_ADDRESS: u16 = 0x200;

impl Interpreter {
    pub fn new() -> Self {
        let mut new_interpreter = Self {
            pc: START_ADDRESS,
            ram: [0; RAM_SIZE],
            display: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_registars: [0; NUM_REGISTARS],
            index_registar: 0,
            stack_pointer: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            delay_timer: 0,
            sound_timer: 0,
        };
        new_interpreter.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        new_interpreter
    }

    ///Push to top of the Interpreters Stack
    fn push(&mut self, value: u16) {
        self.stack[self.stack_pointer as usize] = value;
        self.stack_pointer += 1;
    }
    //TODO --- could cause underflow panic, implement safeguard
    fn pop(&mut self) -> u16 {
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer as usize]
    }

    pub fn reset(&mut self) {
        self.pc = START_ADDRESS;
        self.ram = [0; RAM_SIZE];
        self.display = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.v_registars = [0; NUM_REGISTARS];
        self.index_registar = 0;
        self.stack_pointer = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }
    pub fn tick(&mut self) {
        //Fetch
        let op_code = self.fetch();
        self.execute(op_code);
    }
    fn fetch(&mut self) -> u16 {
        //CHIP_8 is big endian
        let high_byte = self.ram[self.pc as usize] as u16;
        let low_byte = self.ram[(self.pc + 1) as usize] as u16;
        let op_code = (high_byte << 8) | low_byte;
        self.pc += 2;
        op_code
    }

    fn execute(&mut self, op: u16) {
        let nibble1 = (op & 0xF000) >> 12;
        let nibble2 = (op & 0x0F00) >> 8;
        let nibble3 = (op & 0x00F0) >> 4;
        let nibble4 = op & 0x000F;

        match (nibble1, nibble2, nibble3, nibble4) {
            //no op
            (0, 0, 0, 0) => return,
            //clear screen
            (0, 0, 0xE, 0) => {
                self.display = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            }
            //00EE return from subroutine, sets program counter to return address popped from stack
            (0, 0, 0xE, 0xE) => {
                let return_address = self.pop();
                self.pc = return_address;
            }
            //1NNN jump to address
            (1, _, _, _) => {
                let addr = op & 0xFFF;
                self.pc = addr;
            }
            //2NNN call subroutne
            (2, _, _, _) => {
                let addr = op & 0xFFF;
                self.push(self.pc); //pushes current addr in the program counter ontop of the stack
                self.pc = addr // jumps to subroutne
            }
            //3XNN skip next if VF == NN
            (3, _, _, _) => {
                let x = nibble2 as usize;
                let nn = (op & 0xFF) as u8;
                if self.v_registars[x] == nn {
                    self.pc += 2;
                }
            }
            //4XNN skip next if VF != NN
            (4, _, _, _) => {
                let x = nibble2 as usize;
                let nn = (op & 0xFF) as u8;
                if self.v_registars[x] != nn {
                    self.pc += 2;
                }
            }
            //5XY0 skip next instruction if Vx == Vy
            (5, _, _, 0) => {
                let x = nibble2 as usize;
                let y = nibble3 as usize;
                if self.v_registars[x] == self.v_registars[y] {
                    self.pc += 2;
                }
            }
            //6XNN, set Vx to NN
            (6, _, _, _) => {
                let x = nibble2 as usize;
                let nn = (op & 0xFF) as u8;
                self.v_registars[x] = nn;
            }
            //Set Vx to Vx + NN
            (7, _, _, _) => {
                let x = nibble2 as usize;
                let nn = (op & 0xFF) as u8;
                self.v_registars[x] = self.v_registars[x].wrapping_add(nn)
            }
            //8XY0, set Vx to Vy
            (8, _, _, 0) => {
                let x = nibble2 as usize;
                let y = nibble3 as usize;
                self.v_registars[x] = self.v_registars[y];
            }
            //8XY1, Vx |= Vy
            (8, _, _, 1) => {
                let x = nibble2 as usize;
                let y = nibble3 as usize;
                self.v_registars[x] |= self.v_registars[y];
            }
            //8XY2, Vx &= Vy
            (8, _, _, 2) => {
                let x = nibble2 as usize;
                let y = nibble3 as usize;
                self.v_registars[x] &= self.v_registars[y];
            }
            //8XY3, Vx ^= Vy
            (8, _, _, 3) => {
                let x = nibble2 as usize;
                let y = nibble3 as usize;
                self.v_registars[x] ^= self.v_registars[y];
            }
            //8XY4, Vx += Vy, overflow can occur in this function
            (8, _, _, 4) => {
                let x = nibble2 as usize;
                let y = nibble3 as usize;

                let (new_vx, carry) = self.v_registars[x].overflowing_add(self.v_registars[y]);

                let new_vf = if carry { 1 } else { 0 };

                self.v_registars[x] = new_vx;
                self.v_registars[0xF] = new_vf;
            }
            //8XY5, Vx - Vy, underflow can occur here
            (8, _, _, 5) => {
                let x = nibble2 as usize;
                let y = nibble3 as usize;

                let (new_vx, borrow) = self.v_registars[x].overflowing_sub(self.v_registars[y]);

                let new_vf = if borrow { 0 } else { 1 };

                self.v_registars[x] = new_vx;
                self.v_registars[0xF] = new_vf;
            }
            //8XY6, Vx >>= 1,
            (8, _, _, 6) => {
                let x = nibble2 as usize;

                let lsb = self.v_registars[x] & 1;

                self.v_registars[x] >>= 1;
                self.v_registars[0xF] = lsb;
            }
            //8XY7, Vy - Vx, underflow can also occur here
            (8, _, _, 7) => {
                let x = nibble2 as usize;
                let y = nibble3 as usize;

                let (new_vx, borrow) = self.v_registars[y].overflowing_sub(self.v_registars[x]);

                let new_vf = if borrow { 0 } else { 1 };
                self.v_registars[x] = new_vx;
                self.v_registars[0xF] = new_vf;
            }
            //8XYE, Vx <<= 1
            (8, _, _, 0xE) => {
                let x = nibble2 as usize;
                let msb = (self.v_registars[x] >> 7) & 1;
                self.v_registars[x] <<= 1;
                self.v_registars[0xF] = msb;
            }
            //9XY0, skip next instruction if Vx != Vy
            (9, _, _, 0) => {
                let x = nibble2 as usize;
                let y = nibble3 as usize;

                if self.v_registars[x] != self.v_registars[y] {
                    self.pc += 2;
                }
            }

            //ANNN, sets index registar to NNN
            (0xA, _, _, _) => {
                let nnn = op & 0xFFF;
                self.index_registar = nnn;
            }

            //BNNN, jumps to V0 + NNN
            (0xB, _, _, _) => {
                let nnn = op & 0xFFF;
                self.pc = (self.v_registars[0] as u16) + nnn;
            }
            //CXNN, Chip8 random number generator
            //Vx = rng & nn
            (0xC, _, _, _) => {
                let x = nibble2 as usize;
                let nn = (op & 0xFF) as u8;
                let rng: u8 = random();
                self.v_registars[x] = rng & nn;
            }
            //draw sprite to screen
            //DXYN
            (0xD, _, _, _) => {
                // get (x, y) coordinates
                let x_coord = self.v_registars[nibble2 as usize] as u16;
                let y_coord = self.v_registars[nibble3 as usize] as u16;

                //N gives us the height of the sprite
                let num_rows = nibble4;

                //flag for tracking if pixels are flipped
                let mut flipped = false;

                //loop through each row in bitmap
                for row in 0..num_rows {
                    //get address of row data
                    let address = self.index_registar + row as u16;
                    let pixels = self.ram[address as usize];
                    //loop through each column in bitmap
                    for col in 0..8 {
                        //mask to fetch current pixel's bit, only flip if bit is 1
                        if (pixels & (0b1000_0000 >> col)) != 0 {
                            let x = (x_coord + col) as usize % SCREEN_WIDTH;
                            let y = (y_coord + row) as usize % SCREEN_HEIGHT;

                            let idx = x + SCREEN_WIDTH * y;

                            flipped |= self.display[idx];
                            self.display[idx] ^= true;
                        }
                    }
                }

                //populate 0xF registar

                if flipped {
                    self.v_registars[0xF] = 1;
                } else {
                    self.v_registars[0xF] = 0;
                }
            }
            //EX9E, skip next instruction if keypress
            (0xE, _, 9, 0xE) => {
                let x = nibble2 as usize;
                let vx = self.v_registars[x];
                let key = self.keys[vx as usize];
                if key {
                    self.pc += 2;
                }
            }
            //EXA1, skip next instruciton if key is not pressed
            (0xE, _, 0xA, 1) => {
                let x = nibble2 as usize;
                let vx = self.v_registars[x];
                let key = self.keys[vx as usize];
                if !key {
                    self.pc += 2;
                }
            }
            //FX07 Vx = Delay timer
            (0xF, _, 0, 7) => {
                let x = nibble2 as usize;
                self.v_registars[x] = self.delay_timer;
            }

            //FX0A, wait for keypress
            (0xF, _, 0, 0xA) => {
                let x = nibble2 as usize;
                let mut pressed = false;

                for i in 0..self.keys.len() {
                    if self.keys[i] {
                        self.v_registars[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }
                if !pressed {
                    //redo opcode
                    self.pc -= 2;
                }
            }

            //FX15, delay timer = Vx
            (0xF, _, 1, 5) => {
                let x = nibble2 as usize;
                self.delay_timer = self.v_registars[x];
            }

            //FX18, sound timer = vx
            (0xF, _, 1, 8) => {
                let x = nibble2 as usize;
                self.sound_timer = self.v_registars[x];
            }
            //FX1E, Index registar += Vx
            (0xF, _, 1, 0xE) => {
                let x = nibble2 as usize;
            }

            //unimplemented opcode
            (_, _, _, _) => unimplemented!("Unimplemented opcode: {}", op),
        }
    }

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                //Beep
            }
            self.sound_timer -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Interpreter, FONTSET, FONTSET_SIZE, NUM_KEYS, NUM_REGISTARS, SCREEN_HEIGHT, SCREEN_WIDTH,
        STACK_SIZE, START_ADDRESS,
    };

    #[test]

    fn initialization() {
        let cpu = Interpreter::new();
        assert_eq!(cpu.pc, START_ADDRESS);
        assert_eq!(cpu.ram[..FONTSET_SIZE], FONTSET);
        assert_eq!(cpu.display, [false; SCREEN_HEIGHT * SCREEN_WIDTH]);
        assert_eq!(cpu.v_registars, [0; NUM_REGISTARS]);
        assert_eq!(cpu.index_registar, 0);
        assert_eq!(cpu.stack_pointer, 0);
        assert_eq!(cpu.stack, [0; STACK_SIZE]);
        assert_eq!(cpu.keys, [false; NUM_KEYS]);
        assert_eq!(cpu.delay_timer, 0);
        assert_eq!(cpu.sound_timer, 0);
    }
}
