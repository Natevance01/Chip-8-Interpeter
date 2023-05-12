const RAM_SIZE: usize = 4096;
const SCREEN_WIDTH: usize = 32;
const SCREEN_HEIGHT: usize = 64;
const NUM_REGISTARS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;
///Processor
pub struct Processor {
    pc: u16,
    ///program counter
    ram: [u8; RAM_SIZE],
    ///ram
    display: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_registars: [u8; NUM_REGISTARS],
    index_registar: u16,
    stack_pointer: u16,
    stack: [u16; STACK_SIZE],
    keys: [bool; NUM_KEYS],
    delay_timer: u8,
    sound_timer: u8,
}

const START_ADDRESS: u16 = 0x200;

impl Processor {
    pub fn new() -> Self {
        Self {
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
        }
    }
}
