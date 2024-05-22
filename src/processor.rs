// Import necessary modules and types from the current crate and other crates
use crate::{instruction::*, get_pot_address_checked, state::*, get_player_address_checked};
use borsh::BorshDeserialize;
use solana_program::{
	account_info::{next_account_info, AccountInfo},
	entrypoint::ProgramResult,
	program_pack::Pack,
	program::{invoke_signed},
	pubkey::Pubkey,
	rent::Rent,
	sysvar::Sysvar,
	system_instruction,
	msg, 
};

// Define the function that will be called when the program is invoked. This function is the
// "real" entry point for the program.
pub fn process_instruction (
	program_id : &Pubkey,
	accounts : &[AccountInfo],
	instruction_data : &[u8]
) -> ProgramResult {
	// Log a message to the console
	msg!("Running program");

	// Unpack the instruction data into a WagerInstruction
	let instruction = WagerInstruction::unpack_from_slice(instruction_data)?;

	// Create an iterator over the accounts
	let account_info_iter = &mut accounts.iter();

	// Match on the instruction to determine what to do
	match instruction {
		// If the instruction is a NewWager instruction
		WagerInstruction::NewWager { name, account_state } => {
			// Log a message to the console
			msg!("WagerInstruction::NewWager");

			// Get the account info for the funder, pot, system program, and rent sysvar
			let funder_info = next_account_info(account_info_iter)?;
			let pot_info = next_account_info(account_info_iter)?;
			let system_program_info = next_account_info(account_info_iter)?;
			let rent_sysvar_info = next_account_info(account_info_iter)?;

			// Get the rent from the rent sysvar account info
			let rent = &Rent::from_account_info(rent_sysvar_info)?;

			// Get the pot address and bump seed
			let (_,pot_bump_seed) = get_pot_address_checked(name, pot_info.key)?;

			// Define the seeds for the pot signer
			let pot_signer_seeds : &[&[_]] = &[
				&name,
				&[pot_bump_seed]
			];

			// Invoke the system program to create a new account for the pot
			invoke_signed(
				&system_instruction::create_account(
					funder_info.key,
					pot_info.key,
					1.max(rent.minimum_balance(WagerAccount::get_packed_len())),
					WagerAccount::get_packed_len() as u64,
					program_id
				),
				&[
					funder_info.clone(),
					pot_info.clone(),
					system_program_info.clone(),
				],
				&[pot_signer_seeds]
			)?;

			// Pack the account state into the pot account data
			account_state.pack_into_slice(&mut pot_info.data.borrow_mut())
		},

		// If the instruction is a MakeBet instruction
		WagerInstruction::MakeBet {bet_identifier,player_state} => {
			// Log a message to the console
			msg!("WagerInstruction::MakeBet");

			// Get the account info for the funder, pot, player, system program, and rent sysvar
			let funder_info = next_account_info(account_info_iter)?;
			let pot_info = next_account_info(account_info_iter)?;
			let player_info = next_account_info(account_info_iter)?;
			let system_program_info = next_account_info(account_info_iter)?;
			let rent_sysvar_info = next_account_info(account_info_iter)?;

			// Get the rent from the rent sysvar account info
			let rent = &Rent::from_account_info(rent_sysvar_info)?;

			// Get the player address and bump seed
			let (_,player_bump_seed) = get_player_address_checked(bet_identifier, funder_info.key.to_bytes(), player_info.key)?;

			// Define the seeds for the player signer
			let player_signer_seeds : &[&[_]] = &[
				&bet_identifier,
				&funder_info.key.to_bytes(),
				&[player_bump_seed]
			];

			// Get the pot account state
			let mut pot_account = WagerAccount::try_from_slice(&mut pot_info.data.borrow_mut())?;

			// Increment the player counter in the pot account state
			pot_account.player_counter += 1 ;

			// Define the seeds for the pot signer
			let pot_signer_seeds : &[&[_]] = &[
				&bet_identifier,
				&[pot_account.bump_seed]
			];

			// Invoke the system program to create a new account for the player
			invoke_signed(
				&system_instruction::create_account(
					funder_info.key,
					player_info.key,
					1.max(rent.minimum_balance(PlayerAccount::get_packed_len())),
					PlayerAccount::get_packed_len() as u64,
					program_id
				),
				&[
					funder_info.clone(),
					player_info.clone(),
					system_program_info.clone(),
				],
				&[player_signer_seeds,pot_signer_seeds]
			)?;

			// Invoke the system program to transfer funds from the funder to the pot
			invoke_signed(
				&system_instruction::transfer(
				funder_info.key, 
				pot_info.key, 
				player_state.bet_amount.into()
			),
			&[
				funder_info.clone(),
				pot_info.clone(),
			], 
			&[
				pot_signer_seeds
			])?;

			// Increase the balance in the pot account state
			pot_account.balance += player_state.bet_amount;

			// Pack the pot account state into the pot account data
			pot_account.pack_into_slice(&mut pot_info.data.borrow_mut());

			// Pack the player state into the player account data
			player_state.pack_into_slice(&mut player_info.data.borrow_mut());
		},

		// If the instruction is a VoteWinner instruction
		WagerInstruction::VoteWinner {outcome } => {
			// Log the outcome to the console
			msg!(&outcome.to_string());
		},

		// If the instruction is a View instruction
		WagerInstruction::View => {

		},
	}

	// Return Ok to indicate that the instruction was processed successfully
	Ok(())
}