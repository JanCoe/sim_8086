#[derive(Debug)]
enum OpCode {
    Mov,
    Other,
}

#[derive(Debug)]
struct D(bool);

#[derive(Debug)]
struct W(bool);

#[derive(Debug)]
enum Mode {
    MemMode(u8),
    RegMode,
}

#[derive(Debug)]
#[rustfmt::skip]
enum Register { AL, BL, CL, DL, AH, BH, CH, DH, AX, BX, CX, DX, SP, BP, SI, DI, }

fn register_encode(bit_reg: &u8, w: &W) -> Register {
    match bit_reg {
        0b0_000 => {
            if !w.0 {
                Register::AL
            } else {
                Register::AX
            }
        }
        0b0_001 => {
            if !w.0 {
                Register::CL
            } else {
                Register::CX
            }
        }
        0b0_010 => {
            if !w.0 {
                Register::DL
            } else {
                Register::DX
            }
        }
        0b0_011 => {
            if !w.0 {
                Register::BL
            } else {
                Register::BX
            }
        }
        0b0_100 => {
            if !w.0 {
                Register::AH
            } else {
                Register::SP
            }
        }
        0b0_101 => {
            if !w.0 {
                Register::CH
            } else {
                Register::BP
            }
        }
        0b0_110 => {
            if !w.0 {
                Register::DH
            } else {
                Register::SI
            }
        }
        0b0_111 => {
            if !w.0 {
                Register::BH
            } else {
                Register::DI
            }
        }
        _ => panic!("Undefined value"),
    }
}

fn get_code(byte: &u8, first: u8, last: u8) -> u8 {
    (byte & (first..=last).into_iter().map(|x| 2_u8.pow((8-x) as u32)).sum::<u8>()) >> (8-last)
}

fn main() {
    let bytes = std::fs::read("./listing_0038_many_register_mov").unwrap();

    println!("bits 16");

    for byte_pair in bytes.chunks_exact(2) {
        let bit_opcode = get_code(&byte_pair[0], 1, 6);
        let bit_d = get_code(&byte_pair[0], 7, 7);
        let bit_w = get_code(&byte_pair[0], 8, 8);

        let bit_mod = get_code(&byte_pair[1], 1, 2);
        let bit_reg = get_code(&byte_pair[1], 3, 5);
        let bit_rm = get_code(&byte_pair[1], 6, 8);

        let opcode = match bit_opcode {
            0b0_100010 => OpCode::Mov,
            _ => OpCode::Other,
        };

        let d = if bit_d == 0 { D(false) } else { D(true) };
        let w = if bit_w == 0 { W(false) } else { W(true) };

        let mode = match bit_mod {
            0b0_00 => Mode::MemMode(0),
            0b0_01 => Mode::MemMode(8),
            0b0_10 => Mode::MemMode(16),
            0b0_11 => Mode::RegMode,
            _ => panic!("Invalid value."),
        };

        let reg = register_encode(&bit_reg, &w);

        let rm = match mode {
            Mode::MemMode(0) => unimplemented!(),
            Mode::MemMode(8) => unimplemented!(),
            Mode::MemMode(16) => unimplemented!(),
            Mode::RegMode => register_encode(&bit_rm, &w),
            _ => panic!("Invalid value"),
        };

        match d.0 {
            true => println!("{opcode:?} {reg:?}, {rm:?}"),
            false => println!("{opcode:?} {rm:?}, {reg:?}"),
        }
    }
}
