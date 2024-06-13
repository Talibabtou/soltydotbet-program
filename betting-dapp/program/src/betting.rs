use anchor_lang::prelude::*;

use crate::accounts::*;
use crate::program::*;

impl BettingState {
	fn place_bet(&mut self, bet: Bet) -> ProgramResult {
		self.validate_bet(&bet)?; // Propagates the error code if validate_bet fails
		let (house_fee, referral_fee) = self.calculate_fees(&bet);
		let net_amount = bet.amount - house_fee;
		match bet.team {
			Team::Red => self.total_red += net_amount,
			Team::Blue => self.total_blue += net_amount,
		}
		betting_state.bets.push(bet);
		Ok(())
	}

	fn validate_bet(&self, bet: &Bet) -> ProgramResult {
		if self.current_phase != ContractPhase::Betting {
			return Err(ProgramError::Custom(0)); // Not in betting phase
		}
		Ok(())
	}

	fn calculate_fees(&self, bet: &Bet) -> (u64, u64) {
		let fee;
		let referral_fee;
		if let Some(referral) = bet.referral {
			fee = (bet.amount * 35) / 1000; // 3.5% house fee
			referral_fee = (bet.amount * 5) / 1000; // 0.5% referral fee
			// Add the referral fee to the referral's total fee
			let total_fee = self.referral_fees.entry(referral).or_insert(0);
			*total_fee += referral_fee;
		} else {
			fee = (bet.amount * 4) / 100; // 4% house fee
		}
		self.financial_state.house_fee += fee;
		(house_fee, referral_fee)
	}
}