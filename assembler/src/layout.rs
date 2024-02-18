use crate::parser::Instruction;
use std::fmt;

#[derive(Debug)]
pub struct AddressedInstruction {
    /// The instruction's assigned address.
    addr: u16,

    /// The instruction.
    inst: Instruction,
}

impl fmt::Display for AddressedInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{addr:04X}: {inst}", addr = self.addr, inst = self.inst)
    }
}

/// Layout the program.
pub fn layout_program(instructions: Vec<Instruction>) -> Vec<AddressedInstruction> {
    let mut addressed_instructions = Vec::with_capacity(instructions.len());
    let mut cur_addr: u16 = 0;

    for inst in instructions {
        let addressed = AddressedInstruction {
            addr: cur_addr,
            inst,
        };
        addressed_instructions.push(addressed);
        cur_addr += 2;
    }

    addressed_instructions
}
