// This attribute forbids the use of unsafe code in this module
#[forbid(unsafe_code)]

// Import other modules from the same crate
mod entrypoint;
pub mod instruction;
pub mod state;
pub mod processor;

// Re-export the solana_program crate so that other modules can use it
pub use solana_program;
// Import specific items from the solana_program crate
use solana_program::{pubkey::{Pubkey}, program_error::ProgramError, msg};

// Declare the program's ID. This is a unique identifier for the program on the Solana blockchain
solana_program::declare_id!("EEjpJXCfHEqcRyAxW6tr3MNZqpP2MjAErkezFyp4HEah");

// Function to get the address of a pot given a name. The address is derived from the program's ID and the name
pub(crate) fn get_pot_address (name:[u8; 20]) -> (Pubkey, u8) {
	Pubkey::find_program_address(&[&name], &id())
}

// Function to check if a given pot address matches the derived address for a given name
pub(crate) fn get_pot_address_checked (name : [u8; 20], pot_address : &Pubkey) -> Result<(Pubkey, u8), ProgramError> {
	let (derived_address, derived_bump_seed) = get_pot_address(name);
	msg!("{}", derived_address);
	msg!("{}", pot_address);
		if derived_address != *pot_address {
			msg!("Error: pot address derivation mismatch");
			return Err(ProgramError::InvalidArgument);
		} else {
			return Ok((derived_address, derived_bump_seed));
		}
}

// Function to get the address of a player given a bet identifier and funder info. The address is derived from the program's ID, the bet identifier, and the funder info
pub(crate) fn get_player_address (bet_identifier:[u8; 20],funder_info : [u8;32]) -> (Pubkey, u8) {
	Pubkey::find_program_address(&[&bet_identifier,&funder_info], &id())
}

// Function to check if a given player address matches the derived address for a given bet identifier and funder info
pub(crate) fn get_player_address_checked (bet_identifier:[u8; 20],funder_info : [u8;32], player_address : &Pubkey) -> Result<(Pubkey, u8), ProgramError> {
	let (derived_address, derived_bump_seed) = get_player_address(bet_identifier, funder_info);
	msg!("{}", derived_address);
	msg!("{}", player_address);
		if derived_address != *player_address {
			msg!("Error: player address derivation mismatch");
			return Err(ProgramError::InvalidArgument);
		} else {
			return Ok((derived_address, derived_bump_seed));
		}
}

// Unit tests for this module
#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}
}
