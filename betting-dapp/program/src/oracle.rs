use anchor_lang::prelude::*;

impl ContractState {
	fn update_match_result(&mut self, new_result: Team, oracle_sig: Vec<u8>, oracle_data: Vec<u8>) -> ProgramResult {
		// Verify the oracle signature
		require!(self.verify_oracle_signature(oracle_sig, oracle_data), ProgramError::Custom(0));
		// Update the match result
		self.match_state.match_result = Some(new_result);
		Ok(())
	}

	fn verify_oracle_signature(&self, signature: Vec<u8>, data: Vec<u8>) -> bool {
		let oracle_pubkey = PublicKey::from_bytes(&self.oracle.to_bytes()).unwrap();
		let sig = Signature::from_bytes(&signature).unwrap();
		oracle_pubkey.verify(&data, &sig).is_ok()
	}

	fn initialize_oracle(&mut self, oracle: Pubkey, program_id: &Pubkey) {
		// Set up a PDA to establish a protected connection with the oracle
		let (pda, _nonce) = Pubkey::find_program_address(&[b"oracle"], program_id);
		self.oracle = pda;
	}

	fn change_phase(&mut self, new_phase: ContractPhase) {
		if self.current_phase == ContractPhase::Result && new_phase == ContractPhase::Betting {
			let oracle = self.oracle; // Preserve the oracle
			*self = Self::new(oracle);
		}
		self.current_phase = new_phase;
	}
}
