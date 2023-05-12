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
        //Decode
        //Execute
    }
    fn fetch(&mut self) -> u16 {
        //CHIP_8 is big endian
        let high_byte = self.ram[self.pc as usize] as u16;
        let low_byte = self.ram[(self.pc + 1) as usize] as u16;
        let op_code = (high_byte << 8) | low_byte;
        self.pc += 2;
        op_code
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
