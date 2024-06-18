use anchor_lang::prelude::*;
use anchor_lang::prelude::{PublicKey, Signature, Program, Token, TokenAccount};

use crate::ContractState;
use crate::ContractPhase;
use crate::CustomError;
use std::collections::HashMap;


impl ContractState {
	pub fn launch_pda_oracle(oracle: Pubkey) {
		let contract_state = &mut ctx.accounts.contract_state;
		let (pda, _bump) = Pubkey::find_program_address(&[b"oracle"], ctx.program_id);
		contract_state.oracle = pda;
	}

	pub fn update_match_result(&mut self, new_result: Team, oracle_sig: Vec<u8>, oracle_data: Vec<u8>) -> Result<(), CustomError> {
		// Verify the oracle signature
		if !self.verify_oracle_signature(oracle_sig, oracle_data) {
			return Err(CustomError::WrongOracle);
		}
		// Update the match result
		self.matches[current_match_index].match_state.match_result = Some(new_result);
		Ok(())
	}

	fn verify_oracle_signature(&self, signature: Vec<u8>, data: Vec<u8>) -> bool {
		let oracle_pubkey = PublicKey::from_bytes(self.oracle.as_ref()).unwrap();
		let sig = Signature::from_bytes(&signature).unwrap();
		oracle_pubkey.verify(&data, &sig).is_ok()
	}

	pub fn change_phase(&mut self, new_phase: ContractPhase, token_program: &Program<Token>, contract_token_account: &Account<TokenAccount>, bettor_token_accounts: &HashMap<Pubkey, Account<TokenAccount>>) {
		match (self.phase, new_phase) {
			(ContractPhase::Betting, ContractPhase::Match) => {
				// Custom logic for phase change from Betting to Match
			},
			(ContractPhase::Match, ContractPhase::Result) => {
				// Custom logic for phase change from Match to Result
			},
			(ContractPhase::Result, ContractPhase::Betting) => {
				let oracle = self.oracle; // Preserve the oracle
				*self = new(oracle);
			},
			_ => {
				// If the phase change is invalid, cancel everything and refund bets
				self.cancel_and_refund(token_program, contract_token_account, bettor_token_accounts)?;
				Err(ProgramError::Custom(4)); // Invalid phase change
			}
		}
		self.phase = new_phase;
	}
}
