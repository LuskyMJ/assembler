use std::fs;

fn main() {
    let lines: String = fs::read_to_string("program.asm")
        .expect("Should be able to read file");

    let mut instructions: Vec<u32> = Vec::new();

    for line in lines.split("\n") {
        let mut instruction: u32 = 0;
        let cleaned_line: String = line.replace(",", "");
        let arguments: Vec<&str> = cleaned_line.split(" ").collect();
        
        let opcode = match arguments[0] {
            "add" | "sub" | "xor" | "or" | "and" | "sll" | "srl" | "sra" | "slt" | "sltu" => 0b0110011,
            "addi" | "xori" | "ori" | "andi" | "slli" | "srli" | "srai" | "slti" | "sltiu" => 0b0010011,
            "lb" | "lh" | "lw" | "lbu" | "lhu" => 0b0000010,
            "sb" | "sh" | "sw" => 0b0100011,
            "beq" | "bne" | "blt" | "bge" | "bltu" | "bgeu" => 0b1100011,
            "jal" => 0b1101111,
            "jalr" => 0b1100111,
            "lui" => 0b0110111,
            "auipc" => 0010111,
            "ecall" | "ebreak" => 1110011,
            _ => 1 // TODO: This means the instruction is invalid and should be handled
        };

        let funct3 = match arguments[0] {
            "and" | "andi" | "bgeu" => 7,
            "or" | "ori" | "bltu" => 6,
            "srl" | "srai" | "lhu" | "bge" => 5,
            "xor" | "xori" | "lbu" | "blt" => 4,
            "sltu" | "sltiu" => 3,
            "slt" | "slti" | "lw" | "sw" => 2,
            "sll" | "slli" | "lh" | "sh" | "bne" => 1,
            _ => 0
        };

        // Add instruction
        instruction |= opcode;
        instruction |= funct3 << 12;
        instructions.push(instruction);
    }
}