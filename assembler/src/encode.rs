use crate::parser::{token::Mnemonic::*, Instruction, Operand};

fn encode_instruction(inst: Instruction) -> u16 {
    let mut encoded = 0;

    match inst.mnemonic {
        Cls => encoded = 0x00E0,
        Jmp => {
            /// Encode the opcode.
            let opcode = 0x1;
            encoded &= 0x0FFF | (opcode << 12);

            /// Encode the 12-bit immediate hex number.
            let Operand::Immediate(nnn) = inst.operands[0] else {
                panic!("");
            };

            encoded &= 0xF000 | nnn;
        }
        _ => todo!(),
    }

    encoded
}

pub fn encode_instructions(instructions: Vec<Instruction>) -> Vec<u8> {
    instructions
        .into_iter()
        .flat_map(|inst| encode_instruction(inst).to_be_bytes())
        .collect()
}
