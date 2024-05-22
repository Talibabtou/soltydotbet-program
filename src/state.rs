// Import necessary modules and types from the current crate and other crates
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::{
	msg,
	program_error::ProgramError,
	program_pack::{Pack, Sealed},
};

// Define a struct to represent an option type
#[derive(Clone, Copy, Debug, BorshSerialize, BorshDeserialize, BorshSchema, PartialEq, Default)]
pub struct OptionType {
	name : [u8; 20], // The name of the option
	vote_count : u16, // The number of votes for this option
}

// Define a struct to represent a wager account
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, BorshSchema, PartialEq)]
pub struct WagerAccount {
	pub balance : u32, // The balance of the wager account
	pub options : [OptionType; 8], // The options for the wager
	pub params : (u32, u32, u16, u16), // The parameters for the wager (min bet, max bet, min players, max players)
	pub player_counter : u16, // The number of players
	pub bump_seed : u8, // The bump seed for the account
	pub state : WagerState, // The state of the wager
}

// Define an enum to represent the state of a wager
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, BorshSchema, PartialEq)]
pub enum WagerState {
	Uninitialized, // The wager is uninitialized
	Ongoing, // The wager is ongoing
	Settled, // The wager is settled
}

// Implement the Sealed trait for WagerAccount
impl Sealed for WagerAccount {}

// Implement the Pack trait for WagerAccount
impl Pack for WagerAccount {
	const LEN: usize = 196; // The length of a packed WagerAccount

	// Define a method to pack a WagerAccount into a slice
	fn pack_into_slice(&self, dst: &mut [u8]) {
		let data = self.try_to_vec().unwrap();
		dst[..data.len()].copy_from_slice(&data);
	}

	// Define a method to unpack a WagerAccount from a slice
	fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
		let mut mut_src = src;
		Self::deserialize(&mut mut_src).map_err(|err| {
			msg!(
				"Error: failed to deserialize wager account: {}",
				err
			);
			ProgramError::InvalidAccountData
		})
	}
}

// Define a struct to represent a player account
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, BorshSchema, PartialEq)]
pub struct PlayerAccount {
	pub option_name : [u8; 20], // The name of the option the player has chosen
	pub bet_amount : u32, // The amount the player has bet
	pub voted : u8, // Whether the player has voted
	pub bump_seed :u8 // The bump seed for the account
}

// Implement the Sealed trait for PlayerAccount
impl Sealed for PlayerAccount {}

// Implement the Pack trait for PlayerAccount
impl Pack for PlayerAccount {
	const LEN: usize = 26; // The length of a packed PlayerAccount

	// Define a method to pack a PlayerAccount into a slice
	fn pack_into_slice(&self, dst: &mut [u8]) {
		let data = self.try_to_vec().unwrap();
		dst[..data.len()].copy_from_slice(&data);
	}

	// Define a method to unpack a PlayerAccount from a slice
	fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
		let mut mut_src = src;
		Self::deserialize(&mut mut_src).map_err(|err| {
			msg!(
				"Error: failed to deserialize player account: {}",
				err
			);
			ProgramError::InvalidAccountData
		})
	}
}

// Define a test module
#[cfg(test)]
mod tests {
	use crate::state::PlayerAccount;
	use super::{WagerAccount};

	// Define a test function
	#[test]
	fn it_works() {
		// Print the packed length of a WagerAccount
		print!("{}", solana_program::borsh::get_packed_len::<WagerAccount>());
		// Print the packed length of a PlayerAccount
		print!("{}", solana_program::borsh::get_packed_len::<PlayerAccount>());
	}
}