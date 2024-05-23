// Import necessary modules and types from the borsh and solana_program crates
use borsh::{BorshDeserialize,BorshSerialize,BorshSchema};
use solana_program::{
	program_pack::{Pack,Sealed},
	program_error::ProgramError,
	msg,
};

// Import the state module from the current crate
use crate::state::*;

// Define an enum for the different instructions that this program can handle
#[derive(Clone,Debug,BorshSerialize,BorshDeserialize,BorshSchema,PartialEq)]
pub enum WagerInstruction {
	// The NewWager variant is used to create a new wager. It takes a name and a WagerAccount state
	NewWager {
		name : [u8; 20],
		account_state : WagerAccount,
	},
	// The MakeBet variant is used to make a bet. It takes a bet identifier and a PlayerAccount state
	MakeBet {
		bet_identifier : [u8;20],
		player_state : PlayerAccount
	},
	// The VoteWinner variant is used to vote for the winner of a wager. It takes an outcome
	VoteWinner {
		outcome : u8,
	},
	// The View variant is used to view the state of a wager
	View,
}

// Implement the Sealed trait for WagerInstruction. This is a marker trait used by the Solana SDK
impl Sealed for WagerInstruction {}

// Implement the Pack trait for WagerInstruction. This trait is used for serializing and deserializing the instruction data
impl Pack for WagerInstruction {
	// Define the length of the serialized data
	const LEN : usize = 137;

	// Define how to pack the instruction data into a byte slice
	fn pack_into_slice(&self, dst: &mut [u8]) {
		let data = self.pack_into_vec();
		dst[..data.len()].copy_from_slice(&data)
	}

	// Define how to unpack the instruction data from a byte slice
	fn unpack_from_slice(src: &[u8]) -> Result<Self, solana_program::program_error::ProgramError> {
		let mut mut_src = src;
		Self::deserialize(&mut mut_src).map_err(|err| {
			msg!(
				"Unable to deserialize wager instruction {}",
				err
			);
			ProgramError::InvalidInstructionData
		})
	}
}

// Implement additional methods for WagerInstruction
impl WagerInstruction {
	// Define a method to pack the instruction data into a vector
	fn pack_into_vec (&self) -> Vec<u8> {
		self.try_to_vec().expect("try_to_vec")
	}
}