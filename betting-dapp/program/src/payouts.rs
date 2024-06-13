use anchor_lang::prelude::*;

use crate::accounts::*;
use crate::program::*;

impl FinancialState {
	fn distribute_payouts(&mut self) -> ProgramResult {
		if self.current_phase != ContractPhase::Result {
			return Err(ProgramError::Custom(4)); // Not in result phase
		}
		let total_pool = self.total_red + self.total_blue;
		let winner_pool = match self.match_result {
			Some(Team::Red) => self.total_red,
			Some(Team::Blue) => self.total_blue,
			None => {
				return Err(ProgramError::Custom(5)); // No winner set
			}
		};
		for bet in &self.bets {
			if bet.team == self.match_result.clone().unwrap() {
				let _user_payout = (bet.amount * total_pool) / winner_pool;
				// Payout logic here
			}
		}
		self.bets.clear();
		self.total_red = 0;
		self.total_blue = 0;
		self.current_phase = ContractPhase::Betting;
		Ok(())
	}
}