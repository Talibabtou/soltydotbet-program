use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
	program::invoke,
	system_instruction,
};

#[program]
pub mod betting_contract {
	use super::*;

	pub enum ContractPhase {
		Betting,
		Match,
		Result,
	}

	pub enum Team {
		Red,
		Blue,
	}

	#[account]
	pub struct ContractState {
		pub current_phase: ContractPhase,
		pub match_id: u64,
		pub match_result: Option<Team>,
		pub bets: Vec<Bet>,
		pub bet_weights: Vec<BetWeight>,
		pub red_to_blue_rate: f64,
		pub total_red: u64,
		pub total_blue: u64,
		pub house_fee: u64,
	}

	#[account]
	pub struct Bet {
		pub user: Pubkey,
		pub team: Team,
		pub amount: u64,
		pub referral: Option<Pubkey>,
	}

	#[derive(Clone)]
	pub struct BetWeight {
		pub user: Pubkey,
		pub team: Team,
		pub weight: f64,
	}

	impl ContractState {
		pub fn new() -> Self {
			Self {
				match_id: 0,
				match_result: None,
				bets: Vec::new(),
				red_to_blue_rate: 1,
				total_red: 0,
				total_blue: 0,
				house_fee: 0,
			}
		}

		impl ContractState {
			pub fn change_phase(&mut self, new_phase: ContractPhase) {
				if self.current_phase == ContractPhase::Result && new_phase == ContractPhase::Betting {
					*self = Self::new();
				}
				self.current_phase = new_phase;
			}
		}

		pub fn place_bet(&mut self, bet: Bet) -> ProgramResult {
			if self.current_phase != ContractPhase::Betting {
				return Err(ProgramError::Custom(0))
			}
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
			self.house_fee += fee;
			let net_amount = bet.amount - fee;
			match bet.team {
				Team::Red => self.total_red += net_amount,
				Team::Blue => self.total_blue += net_amount,
			}
			self.bets.push(bet);
			Ok(())
		}

		pub fn start_match(&mut self) -> ProgramResult {
			if self.current_phase != ContractPhase::Betting {
				return Err(ProgramError::Custom(1)); // Not in betting phase
			}
			if self.total_red == 0 || self.total_blue == 0 {
				// Refund all bets if one side is empty
				for bet in &self.bets {
					let accounts = ctx.accounts.iter().find(|a| a.key() == bet.user);
					if let Some(account) = accounts {
						let ix = system_instruction::transfer(
							&ctx.accounts.contract_state.to_account_info().key,
							&bet.user,
							bet.amount,
						);
						invoke(
							&ix,
							&[
								ctx.accounts.contract_state.to_account_info(),
								account.clone(),
							],
						)?;
					}
				}
				self.bets.clear();
				self.total_red = 0;
				self.total_blue = 0;
				return Err(ProgramError::Custom(2)); // One side is empty
			}
			else {
				// Calculate the rate of total red to total blue
				self.red_to_blue_rate = self.total_red as f64 / self.total_blue as f64;
				// Calculate weights for each bet
				self.bet_weights.clear(); // Clear previous weights
				for bet in &self.bets {
					let weight = match bet.team {
						Team::Red => bet.amount as f64 / self.total_red as f64,
						Team::Blue => bet.amount as f64 / self.total_blue as f64,
					};
					self.bet_weights.push(BetWeight {
						user: bet.user,
						team: bet.team,
						weight,
					});
				}
				self.current_phase = ContractPhase::Match;
				Ok(())
			}
		}

		pub fn end_match(&mut self, winner: Team) -> ProgramResult {
			if self.current_phase != ContractPhase::Match {
				return Err(ProgramError::Custom(3)); // Not in match phase
			}
			self.match_result = Some(winner);
			self.current_phase = ContractPhase::Result;
			Ok(())
		}

		pub fn distribute_payouts(&mut self) -> ProgramResult {
			if self.current_phase != ContractPhase::Result {
				return Err(ProgramError::Custom(4)); // Not in result phase
			}
			let total_pool = self.total_red + self.total_blue;
			let winner_pool = match self.match_result {
				Some(Team::Red) => self.total_red,
				Some(Team::Blue) => self.total_blue,
				None => return Err(ProgramError::Custom(5)), // No winner set
			};
			for bet in &self.bets {
				if bet.team == self.match_result.unwrap() {
					let user_payout = (bet.amount * total_pool) / winner_pool;
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
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
	#[account(mut)]
	pub user: Signer<'info>,
	#[account(mut)]
	pub contract_state: Account<'info, ContractState>,
}

#[derive(Accounts)]
pub struct StartMatch<'info> {
	#[account(mut)]
	pub contract_state: Account<'info, ContractState>,
}

#[derive(Accounts)]
pub struct EndMatch<'info> {
	#[account(mut)]
	pub contract_state: Account<'info, ContractState>,
}

#[derive(Accounts)]
pub struct DistributePayouts<'info> {
	#[account(mut)]
	pub contract_state: Account<'info, ContractState>,
}

