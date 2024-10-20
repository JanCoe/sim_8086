/*!
The typical layout of the instruction code for Intel 8086 is as follows:

First byte:
 1   2   3   4   5   6   7   8  <- bit digit
-opco1- ---opco2--- ---opco3--- <- octal digit
--------OPCODE--------- -D- -W- <- Intel instruction manual

Second byte:
 1   2   3   4   5   6   7   8  <- bit number
---x--- -----r----- -----m----- <- octal digit
--MOD-- ----REG---- ----R/M---- <- Intel instruction manual (aligns to octal)
*/

#[derive(Debug)]
enum OpCode {
    Mov,
    Other,
}

#[derive(Debug)]
enum Mode {
    MemMode(u8),
    RegMode,
}

#[derive(Debug)]
#[rustfmt::skip]
enum Register {
    Al, Bl, Cl, Dl, Ah, Bh, Ch, Dh,
    Ax, Bx, Cx, Dx, Sp, Bp, Si, Di,
    Es, Cs, Ss, Ds,
}

#[derive(Debug)]
enum Displacement {
    D8,
    D16,
}

#[derive(Debug)]
struct Oct(u8, u8, u8);

/// Convert byte to octal.
fn byte_to_octal(byte: u8) -> Oct {
    Oct(
        (byte & 0b0_11000000) >> 6,
        (byte & 0b0_00111000) >> 3,
        (byte & 0b0_00000111) >> 0,
    )
}

/// Mapping from 3 octal digits to OpCode.
fn get_opcode(op1: &u8, op2: &u8, op3: &u8) -> OpCode {
    let opcode = op1 * 100 + op2 * 10 + op3;
    match opcode {
        210..=216 => OpCode::Mov,
        _ => OpCode::Other,
    }
}

/// Get the mode of the instruction based on the bits in the MOD position.
fn get_mode(mod_: &u8) -> Mode {
    match mod_ {
        0..=2 => Mode::MemMode(*mod_),
        3 => Mode::RegMode,
        _ => panic!("Invalid value."),
    }
}

/// Get the register based on the REG part of the instruction.
fn get_register(word: &u8, reg: &u8) -> Register {
    match (word, reg) {
        (0, 0) => Register::Al,
        (0, 1) => Register::Cl,
        (0, 2) => Register::Dl,
        (0, 3) => Register::Bl,
        (0, 4) => Register::Ah,
        (0, 5) => Register::Ch,
        (0, 6) => Register::Dh,
        (0, 7) => Register::Bh,
        (1, 0) => Register::Ax,
        (1, 1) => Register::Cx,
        (1, 2) => Register::Dx,
        (1, 3) => Register::Bx,
        (1, 4) => Register::Sp,
        (1, 5) => Register::Bp,
        (1, 6) => Register::Si,
        (1, 7) => Register::Di,
        _ => panic!("Undefined value"),
    }
}

/// Get the segment register.
fn get_segment(seg: &u8) -> Register {
    match seg {
        0 => Register::Es,
        1 => Register::Cs,
        2 => Register::Ss,
        3 => Register::Ds,
        _ => panic!("Undefined value"),
    }
}

/// Get register or segment.
fn get_register_or_segment(op3: &u8, word: &u8, reg: &u8) -> Register {
    match op3 {
        0..=3 => get_register(word, reg),
        _ => get_segment(reg),
    }
}

/// Get register or first part of memory field.
fn get_register_or_mem1(mod_: &Mode, word: &u8, mem: &u8) -> Register {
    match mod_ {
        Mode::MemMode(_) => get_memory1(&mem),
        Mode::RegMode => get_register(&word, &mem),
    }
}

/// Get first part of memory field.
fn get_memory1(mem: &u8) -> Register {
    match mem {
        0 | 1 => Register::Bx,
        2 | 3 => Register::Bp,
        4 => Register::Si,
        5 => Register::Di,
        6 => Register::Bp,
        7 => Register::Bx,
        _ => panic!("Invalid"),
    }
}

/// Get second part of memory field.
fn get_memory2(mod_: &u8, mem: &u8) -> Option<Register> {
    match mod_ {
        0..=2 => match mem {
            0 | 2 => Some(Register::Si),
            1 | 3 => Some(Register::Di),
            _ => None,
        },
        _ => None,
    }
}

/// Get displacement.
fn get_displacement(mod_: &Mode) -> Option<Displacement> {
    match mod_ {
        Mode::MemMode(1) => Some(Displacement::D8),
        Mode::MemMode(2) => Some(Displacement::D16),
        _ => None,
    }
}

fn main() {
    // Read binary file.
    let bytes = std::fs::read("./listing_0038_many_register_mov").unwrap();

    // Instruction to assemble for 16 bit machine.
    println!("bits 16");
    let mut bytes_iter = bytes.iter();

    // Loop through all the instructions.
    loop {
        let first = match bytes_iter.next() {
            Some(&byte) => byte_to_octal(byte),
            None => break,
        };

        let second = match bytes_iter.next() {
            Some(&byte) => byte_to_octal(byte),
            None => break,
        };

        let opcode: OpCode;
        let dest: u8;
        let word: u8;
        let mod_: Mode;
        let reg: Register;
        let mem1: Register;
        let mem2: Option<Register>;
        let disp: Option<Displacement>;

        match (first, second) {
            (Oct(op1, op2, op3), Oct(x, r, m)) => {
                opcode = get_opcode(&op1, &op2, &op3);

                dest = (op3 & 0b0_010) >> 1;
                word = (op3 & 0b0_001) >> 0;
                mod_ = get_mode(&x);
                reg = get_register_or_segment(&op3, &word, &r);

                mem1 = get_register_or_mem1(&mod_, &word, &m);
                mem2 = get_memory2(&x, &m);
                disp = get_displacement(&mod_);
            }
        };

        match (dest, mem2, disp) {
            (0, None, None) => println!("{opcode:?} {mem1:?}, {reg:?}"),
            (1, None, None) => println!("{opcode:?} {reg:?}, {mem1:?}"),
            (0, Some(mem2), None) => println!("{opcode:?} {mem1:?} + {mem2:?}, {reg:?}"),
            (1, Some(mem2), None) => println!("{opcode:?} {reg:?}, {mem1:?} + {mem2:?}"),
            (0, None, Some(disp)) => println!("{opcode:?} {mem1:?} + {disp:?}, {reg:?}"),
            (1, None, Some(disp)) => println!("{opcode:?} {reg:?}, {mem1:?} + {disp:?}"),
            (0, Some(mem2), Some(disp)) => println!("{opcode:?} {mem1:?} + {mem2:?} + {disp:?}, {reg:?}"),
            (1, Some(mem2), Some(disp)) => println!("{opcode:?} {reg:?}, {mem1:?} + {mem2:?} + {disp:?}"),
            _ => panic!("Not implemented."),
        }
    }
}
