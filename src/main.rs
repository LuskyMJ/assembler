use std::fs;

enum InstructionType { R, I, S, B, U, J }

fn extract_bits(number: u32, end: u32, start: u32) -> u32 {
    let bit_count = end - start + 1;
    let base: u32 = 2;
    let mask = base.pow(bit_count) - 1;
    return (number >> start) & mask;
}

fn main() {
    let lines: String = fs::read_to_string("program.asm")
        .expect("Should be able to read file");
    let mut instructions: Vec<u32> = Vec::new();
    
    // Loop through each line
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
        
        let instruction_type = match opcode {
            0b0110011 => InstructionType::R,
            0b0100011 => InstructionType::S,
            0b1100011 => InstructionType::B,
            0b1101111 => InstructionType::J,
            0b0110111 | 0b0010111 => InstructionType::U,
            _ => InstructionType::I
        };

        // Extract immediate TODO: Not all operation give immediate here
        let imm: u32 = arguments[3].parse()
            .expect("Could parse immediate to integer");

        let imm_mask = match opcode {
            0b0010011 | 0b0000011 => imm << 20, // I-type
            0b0100011 => extract_bits(imm, 11, 5) << 25 | extract_bits(imm, 4, 0) << 7, // S-type
            _ => 0 // TODO: Add the other instruction types
        };

        // Extract rs1
        let rs1 = match instruction_type {
            InstructionType::R | InstructionType::I |InstructionType::S | InstructionType::B => {
                let test = arguments[3];
                let s = "Hello, World!";
                let mut s = String::from("hello world"); // TODO: Figure out wtf the difference between String, str and &str is. Ownership/Borrowing related?
                2
            },
            _ => 0
        };

        // Add instruction
        instruction |= opcode;
        instruction |= funct3 << 12;
        instruction |= imm_mask;
        instruction |= rs1 << 15;
        instructions.push(instruction);
        println!("Instruction: {}", instruction);
    }
}