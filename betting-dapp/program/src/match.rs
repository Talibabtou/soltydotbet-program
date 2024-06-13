use anchor_lang::prelude::*;

use crate::accounts::*;
use crate::program::*;

impl MatchState {
	pub fn start_match(&mut self) -> ProgramResult {
		// Update the contract phase to match
		self.phase = ContractPhase::Match;
		// Check if the contract is in the correct phase
		self.check_match_phase()?;
		// Check if either side has no bets and refund if necessary
		self.check_and_refund_empty_sides()?;
		// Calculate the rate of red to blue bets
		self.calculate_red_to_blue_rate();
		// Calculate the weight of each bet
		self.calculate_bet_weights();
		Ok(())
	}

	fn check_match_phase(&self) -> ProgramResult {
		// Ensure the contract is in the match phase
		if self.phase != ContractPhase::Match {
			return Err(ProgramError::Custom(2)); // Not in match phase
		}
		Ok(())
	}

	fn check_and_refund_empty_sides(&mut self) -> ProgramResult {
		// Check if either side has no bets
		if self.betting_state.total_red == 0 || self.betting_state.total_blue == 0 {
			// Refund all bets if one side is empty
			for bet in &self.betting_state.bets {
				// Refund the bet
				self.refund_bet(bet)?;
			}
			// Clear the bets and reset totals
			self.betting_state.bets.clear();
			self.betting_state.total_red = 0;
			self.betting_state.total_blue = 0;
			return Err(ProgramError::Custom(3)); // One side is empty
		}
		Ok(())
	}

	fn refund_bet(&self, bet: &Bet) -> ProgramResult {
		// Refund a single bet
		// This is a placeholder; actual implementation will depend on your system
		Ok(())
	}

	fn calculate_red_to_blue_rate(&mut self) {
		// Calculate the rate of red to blue bets
		self.match_state.red_to_blue_rate = self.betting_state.total_red as f64 / self.betting_state.total_blue as f64;
	}

	fn calculate_bet_weights(&mut self) {
		// Calculate the weight of each bet
		for bet in &self.betting_state.bets {
			let weight = match bet.team {
				Team::Red => bet.amount as f64 / self.betting_state.total_red as f64,
				Team::Blue => bet.amount as f64 / self.betting_state.total_blue as f64,
			};
			self.betting_state.bet_weights.push(BetWeight {
				user: bet.user,
				team: bet.team.clone(), // Clone the team to avoid move
				weight,
			});
		}
	}
}

