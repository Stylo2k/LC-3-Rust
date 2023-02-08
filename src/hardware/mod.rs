//pub mod instruction;
//pub mod memory;
pub mod instruction;
pub mod register;
pub mod vm;

use std::hash::{Hash, Hasher};
use vm::VM;

pub const MEMORY_SIZE: usize = std::u16::MAX as usize;

pub fn execute_program(vm: &mut VM) {

    // set for hashes
    let mut seen: std::collections::HashSet<u64> = std::collections::HashSet::new();

    //initialize Registers
    while vm.registers.pc < MEMORY_SIZE as u16 {
        //read instruction
        let instruction = vm.read_memory(vm.registers.pc);

        // hash the vm state
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        vm.hash(&mut hasher);
        let hash = hasher.finish();

        // check if we have seen this state before
        if seen.contains(&hash) {
            eprintln!("Loop detected at PC: {}", vm.registers.pc);
            eprintln!("Instruction: {}", instruction);
            eprintln!("Registers: {:?}", vm.registers);
            eprintln!("=== Note: This is an experimental feature. It might not work as expected. ===");
        } else {
            seen.insert(hash);
        }

        //increment program counter
        vm.registers.pc += 1;

        //extract op_code and execute operation...
        instruction::execute_instruction(instruction, vm)
    }
}
