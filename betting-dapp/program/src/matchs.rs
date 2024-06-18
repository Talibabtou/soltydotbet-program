use anchor_lang::prelude::*;

use crate::accounts::*;

impl MatchState {
	pub fn start_match(&mut self, contract_state: &mut ContractState) -> ProgramResult {
		contract_state.phase = ContractPhase::Match;
		self.check_match_phase(contract_state)?;
		self.check_and_refund_empty_sides(&mut contract_state.matches[current_match_index].betting_state)?;
		self.calculate_red_to_blue_rate(&contract_state.matches[current_match_index].betting_state);
		self.calculate_bet_weights(&mut contract_state.matches[current_match_index].betting_state);
		Ok(())
	}

	fn check_match_phase(&self, contract_state: &ContractState) -> ProgramResult {
		if contract_state.phase != ContractPhase::Match {
			return Err(ProgramError::Custom(2)); // Not in match phase
		}
		Ok(())
	}

	fn check_and_refund_empty_sides(&mut self, betting_state: &mut BettingState) -> ProgramResult {
		if betting_state.total_red == 0 || betting_state.total_blue == 0 {
			for bet in &betting_state.bets {
				self.refund_bet(bet)?;
			}
			// replace with reinitialize function
			betting_state.bets.clear();
			betting_state.total_red = 0;
			betting_state.total_blue = 0;
			return Err(ProgramError::Custom(3)); // One side is empty
		}
		Ok(())
	}

	fn refund_bet(&self, bet: &Bet) -> ProgramResult {
		// Refund a single bet
		// This is a placeholder; actual implementation will depend on your system
		Ok(())
	}

	fn calculate_red_to_blue_rate(&mut self, betting_state: &BettingState) {
		self.red_to_blue_rate = betting_state.total_red as f64 / betting_state.total_blue as f64;
	}

	fn calculate_bet_weights(&self, betting_state: &mut BettingState) {
		for bet in &betting_state.bets {
			let weight = match bet.team {
				Team::Red => bet.amount as f64 / betting_state.total_red as f64,
				Team::Blue => bet.amount as f64 / betting_state.total_blue as f64,
			};
			betting_state.bet_weights.push(BetWeight {
				user: bet.user,
				team: bet.team.clone(),
				weight,
			});
		}
	}
}
