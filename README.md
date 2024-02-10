# c8
A CHIP-8 assembler and emulator written in Rust.

## Assembly Cheatsheet

| Mnemonic | Description | Example |
| -------- | ----------- | ------- |
| `cls`    | Clear the screen | `cls` |
| `jmp NNN`| Jump to specified memory address | `jmp #500` |
| `csrt NNN` | Call sub routine | `csrt #500` |
| `rsrt` | Return from subrutine | `rsrt` |
| `seq VX, NN or VY` | Skip next instruction if operands are equal | `seq V0, #100`, `seq V0, V1` |
| `sneq VX, NN or VY`| Skip next instruction if operands are not equal | `sneq V0, #100`, `sneq V0, V1` |
| `set VX, NN or VY or delay` | Set the value of `VX` | `set V0, #100`, `set V0, V2`, `set V2, delay` |
| `set delay, VX` | Set delay timer to value of `VX` | `set delay, V0` |
| `set sound, VX` | Set sound timer to value of `VX` | `set sound, V0` |
| `set index, NNN` | Set index register to immediate 12-bit memory address | `set index, #340` |
| `set index, VX` | Set index register to address of hexadecimal character in `VX` | `set index, V1` |
| `add VX, NN` | Add immediate 8-bit value to the value of `VX` | `add V0, #5`, `add V1, V3` |
| `add VX, VY` | Add value of `VY` to the value of `VX` (affects `VF`) | `add V1, V3` |
| `add index, VX` | Add value of `VX` to index register | `add index, V0` | 
| `bwor VX, VY` | Bitwise OR the value of `VX` with the value of `VY` | `bwor V0, V2` |
| `bwand VX, VY` | Bitwise AND the value of `VX` with the value of `VY` | `bwand V0, V2` |
| `bwxor VX, VY` | Bitwise XOR the value of `VX` with the value of `VY` | `bwxor V0, V2` |
| `sub VX, VY` | Subtract value of `VY` from value of `VX` (affects `VF`) | `sub V0, V1` |
| `subb VX, VY` | Subtract value of `VX` from value of `VY` (affects `VF`) | `subb V0, V1` |
| `sftr VX` | Shifts the value in `VX` right by one (affects `VF`) | `sftr V1` |
| `sftl VX` | Shifts the value in `VX` left by one (affects `VF`) | `sftl V1` |
| `jmpwo NNN`| Jump with offset | `jmpwo #200` |
| `rand VX, NN` | Generate a random number, storing the result of bitwise ANDing it with immediate 8-bit value in `VX` | `rand V0, #10` |
| `draw VX, VY, N` | Draw a `N` pixels tall sprite at coordinates (value of `VX`, value of `VY`) | `draw V0, V2, #15` | 
| `skp VX` | Skip next instruction is key corresponding to `VX`'s value is pressed | `skp V1` |
| `sknp VX` | Skip next instruction is key corresponding to `VX`'s value is not pressed | `sknp V1` |
| `gk VX` | Block until a key is pressed, storing the pressed key's hex value in `VX` | `gk V1` |
| `bcd VX` | Perform BCD conversion with the number in `VX` | `bcd V1` |
| `stm VX` | Store value of all variable registers up to `VX`, inclusive, into successive memory addresses starting at address in the index register | `stm V7` |
| `ldm VX` | Load values in successive memory addresses, starting at address in the index register, into all variable registers up to `VX` inclusive | `ldm V7` |
