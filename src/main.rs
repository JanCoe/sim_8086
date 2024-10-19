use std::slice::Iter;

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
enum Register { AL, BL, CL, DL, AH, BH, CH, DH, AX, BX, CX, DX, SP, BP, SI, DI, }

#[derive(Debug)]
struct Octal(u8, u8, u8);

fn byte_to_octal(byte: u8) -> Octal {
    Octal((byte & 0b0_11000000) >> 6, (byte & 0b0_00111000) >> 3, byte & 0b0_00000111)
}

fn bits(byte: u8, first: u8, number: u8) -> u8 {
    // Return the number of bits starting from first.
    (byte
        & (first..(first + number))
            .into_iter()
            .map(|x| 2_u8.pow((8 - x) as u32))
            .sum::<u8>())
        >> (8 - (first + number - 1))
}

fn get_register(reg: u8, w: u8) -> &'static str {
    // Get the register based on the Reg part of the OpCode.
    match (w, reg) {
        (0, 0o0_0) => "AL",
        (0, 0o0_1) => "CL",
        (0, 0o0_2) => "DL",
        (0, 0o0_3) => "BL",
        (0, 0o0_4) => "AH",
        (0, 0o0_5) => "CH",
        (0, 0o0_6) => "DH",
        (0, 0o0_7) => "BH",
        (1, 0o0_0) => "AX",
        (1, 0o0_1) => "CX",
        (1, 0o0_2) => "DX",
        (1, 0o0_3) => "BX",
        (1, 0o0_4) => "SP",
        (1, 0o0_5) => "BP",
        (1, 0o0_6) => "SI",
        (1, 0o0_7) => "DI",
        _ => panic!("Undefined value"),
    }
}

fn get_mode(mode: u8) -> Mode {
    // Get the Mode of the OpCode.
    match mode {
        0b0_00 => Mode::MemMode(0),
        0b0_01 => Mode::MemMode(8),
        0b0_10 => Mode::MemMode(16),
        0b0_11 => Mode::RegMode,
        _ => panic!("Invalid value."),
    }
}

fn mov_reg_to_rm(byte_1: u8, bytes: &mut Iter<u8>) {
    /* 1 0 0 0 1 0 d w  mod mod reg reg reg rm rm rm
    __ 1 2 3 4 5 6 7 8   1   2   3   4   5   6  7  8 */

    let d = bits(byte_1, 7, 1);
    let w = bits(byte_1, 8, 1);

    let byte_2 = *bytes.next().unwrap();

    let mode = get_mode(bits(byte_2, 1, 2));
    let reg = get_register(bits(byte_2, 3, 3), w);

    let rm = match mode {
        Mode::RegMode => {
            if w == 0 {
                match bits(byte_2, 6, 3) {
                    0o0_0 => &"AL",
                    0o0_1 => &"CL",
                    0o0_2 => &"DL",
                    0o0_3 => &"BL",
                    0o0_4 => &"AH",
                    0o0_5 => &"CH",
                    0o0_6 => &"DH",
                    0o0_7 => &"BH",
                    _ => panic!("Undefined value"),
                }
            } else {
                match bits(byte_2, 6, 3) {
                    0b0_000 => &"AX",
                    0b0_001 => &"CX",
                    0b0_010 => &"DX",
                    0b0_011 => &"BX",
                    0b0_100 => &"SP",
                    0b0_101 => &"BP",
                    0b0_110 => &"SI",
                    0b0_111 => &"DI",
                    _ => panic!("Undefined value"),
                }
            }
        }
        Mode::MemMode(_) => match bits(byte_2, 6, 3) {
            0o0_0 => &"bx + si",
            0o0_1 => &"bx + di",
            0o0_2 => &"bp + si",
            0o0_3 => &"bp + di",
            0o0_4 => &"si",
            0o0_5 => &"di",
            0o0_6 => &"bp",
            0o0_7 => &"bx",
            _ => panic!("invalid"),
        },
    };

    if d == 0 {
        println!("MOV {reg}, {rm}")
    } else {
        println!("MOV {rm}, {reg}")
    };
}

fn mov_imm_to_reg_mem(byte_1: u8, bytes: &mut Iter<u8>) {
    unimplemented!()
}

fn mov_imm_to_reg(byte_1: u8, bytes: &mut Iter<u8>) {
    let w = bits(byte_1, 5, 1);
    let reg = get_register(bits(byte_1, 7, 2), 0); //check

    let data = *bytes.next().unwrap();

    if w == 0 {
        println!("{:?} {:?}, {:?}", OpCode::Mov, reg, data)
    } else {
        println!(
            "{:?} {:?}, {:?}",
            OpCode::Mov,
            reg,
            data + *bytes.next().unwrap()
        )
    };
}

fn mov_mem_to_acc(byte_1: u8, bytes: &mut Iter<u8>) {
    unimplemented!();
}

fn mov_acc_to_mem(byte_1: u8, bytes: &mut Iter<u8>) {
    unimplemented!();
}

fn main() {
    // Read binary file.
    let bytes = std::fs::read("./listing_0038_many_register_mov").unwrap();

    // Instruction to assemble for 16 bit machine.
    println!("bits 16");
    let mut bytes_iter = bytes.iter();

    loop {
        let byte = match bytes_iter.next() {
            Some(&byte) => byte,
            None => break,
        };

        let octal1 = byte_to_octal(byte);
        // println!("{:?}", octal1);

        // let byte2 = match bytes_iter.next() {
        //     Some(&byte) => byte,
        //     None => break,
        // };
        // let octal2 = byte_to_octal(byte2);

        if bits(byte, 1, 4) == 0b0_1011 {
            mov_imm_to_reg(byte, &mut bytes_iter)
        } else if bits(byte, 1, 6) == 0b0_100010 {
            mov_reg_to_rm(byte, &mut bytes_iter)
        } else {
            match bits(byte, 1, 7) {
                0b0_1100011 => mov_imm_to_reg_mem(byte, &mut bytes_iter),
                0b0_1010000 => mov_mem_to_acc(byte, &mut bytes_iter),
                0b0_1010001 => mov_acc_to_mem(byte, &mut bytes_iter),
                _ => unimplemented!(""),
            }
        };
    }
}
