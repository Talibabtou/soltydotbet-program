// This line tells the Rust compiler to ignore this file if the "no-entrypoint" feature is enabled.
// This is useful for testing and other situations where you might want to disable the entry point.
#![cfg(not(feature = "no-entrypoint"))]

// Import necessary modules and types from the solana_program crate
use solana_program::{
	pubkey::Pubkey,
	entrypoint, entrypoint::ProgramResult,
	account_info::AccountInfo,
};

// Define the entry point for the program. The `entrypoint!` macro from the Solana SDK sets up
// the boilerplate needed to create an entry point for a Solana program.
entrypoint!(process_instruction);

// Define the function that will be called when the program is invoked. This function is the
// "real" entry point for the program.
// `program_id` is the public key of the program, `accounts` is a slice of account information
// that provides context and data for the instruction, and `instruction_data` is a byte slice
// containing the instruction data.
pub fn process_instruction (
	program_id : &Pubkey,
	accounts : &[AccountInfo],
	instruction_data : &[u8]
) -> ProgramResult {
	// Call the `process_instruction` function from the processor module. This function will
	// handle the actual logic of the program.
	crate::processor::process_instruction(program_id, accounts, instruction_data)
}