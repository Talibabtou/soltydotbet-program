use anchor_lang::prelude::*;

#[error_code]
pub enum ProgramError {
	#[msg("Only the deployer can call this function.")]
	Unauthorized = 1,
	#[msg("The contract has already been initialized.")]
	AlreadyInitialized = 2,
}
