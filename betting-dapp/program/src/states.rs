use anchor_lang::prelude::*;
use std::collections::HashMap;

impl ContractState {
	fn launch(oracle: Pubkey) -> Self {
		Self {
			current_phase: ContractPhase::Betting,
			match_id: 0,
			match_result: None,
			bets: Vec::new(),
			bet_weights: Vec::new(),
			red_to_blue_rate: 1.0,
			total_red: 0,
			total_blue: 0,
			house_fee: 0,
			referral_fees: HashMap::new(),
			oracle,
		}
	}
}
