use anchor_lang::prelude::*;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke;
use solana_program::system_instruction;
use anchor_lang::system_program::ID;

pub mod accounts;

#[program]
pub mod betting_contract {
	use super::*;

	pub fn initialize_oracle(ctx: Context<InitializeOracle>, oracle: Pubkey) -> ProgramResult {
		let contract_state = &mut ctx.accounts.contract_state;
		contract_state.initialize_oracle(oracle);
		Ok(())
	}

	pub fn change_phase(ctx: Context<ChangePhase>, new_phase: ContractPhase) -> ProgramResult {
		let contract_state = &mut ctx.accounts.contract_state;

		// Ensure the caller is the authorized oracle
		if ctx.accounts.oracle.key != &contract_state.oracle {
			return Err(ProgramError::Custom(6)); // Unauthorized
		}

		contract_state.change_phase(new_phase);
		Ok(())
	}

	pub fn place_bet(ctx: Context<PlaceBet>, bet: Bet) -> ProgramResult {
		let contract_state = &mut ctx.accounts.contract_state;
		contract_state.place_bet(bet)
	}

	pub fn start_match<'a, 'b, 'c, 'info>(
		ctx: Context<'a, 'b, 'c, 'info, StartMatch<'info>>,
	) -> ProgramResult {
		let contract_state = &mut ctx.accounts.contract_state;
		contract_state.start_match(ctx)
	}

	pub fn distribute_payouts(ctx: Context<DistributePayouts>) -> ProgramResult {
		let contract_state = &mut ctx.accounts.contract_state;
		contract_state.distribute_payouts()
	}
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ContractPhase {
	Betting,
	Match,
	Result,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
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
	pub referral_fees: std::collections::HashMap<Pubkey, u64>,
	pub oracle: Pubkey,
}

#[account]
pub struct Bet {
	pub user: Pubkey,
	pub team: Team,
	pub amount: u64,
	pub referral: Option<Pubkey>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BetWeight {
	pub user: Pubkey,
	pub team: Team,
	pub weight: f64,
}

impl ContractState {
	fn new(oracle: Pubkey) -> Self {
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
			referral_fees: std::collections::HashMap::new(),
			oracle, // Initialize oracle
		}
	}

	fn initialize_oracle(&mut self, oracle: Pubkey) {
		self.oracle = oracle;
	}

	fn change_phase(&mut self, new_phase: ContractPhase) {
		if self.current_phase == ContractPhase::Result && new_phase == ContractPhase::Betting {
			let oracle = self.oracle; // Preserve the oracle
			*self = Self::new(oracle);
		}
		self.current_phase = new_phase;
	}

	fn place_bet(&mut self, bet: Bet) -> ProgramResult {
		if self.current_phase != ContractPhase::Betting {
			return Err(ProgramError::Custom(0)); // Not in betting phase
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

	fn start_match<'a, 'b, 'c, 'info>(
		&mut self,
		ctx: Context<'a, 'b, 'c, 'info, StartMatch<'info>>,
	) -> ProgramResult {
		if self.current_phase != ContractPhase::Match {
			return Err(ProgramError::Custom(3)); // Not in match phase
		}
		if self.total_red == 0 || self.total_blue == 0 {
			// Refund all bets if one side is empty
			for bet in &self.bets {
				let accounts = ctx.remaining_accounts.iter().find(|a| a.key() == bet.user);
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
		} else {
			// Calculate the rate of total red to total blue
			self.red_to_blue_rate = self.total_red as f64 / self.total_blue as f64;
			// Calculate weights for each bet
			for bet in &self.bets {
				let weight = match bet.team {
					Team::Red => bet.amount as f64 / self.total_red as f64,
					Team::Blue => bet.amount as f64 / self.total_blue as f64,
				};
				self.bet_weights.push(BetWeight {
					user: bet.user,
					team: bet.team.clone(), // Clone the team to avoid move
					weight,
				});
			}
			self.current_phase = ContractPhase::Match;
			Ok(())
		}
	}

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

#[derive(Accounts)]
pub struct ReferralFee<'info> {
	#[account(mut)]
	pub contract_state: Account<'info, ContractState>,
	#[account(mut)]
	pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct InitializeOracle<'info> {
	#[account(mut)]
	pub contract_state: Account<'info, ContractState>,
	pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ChangePhase<'info> {
	#[account(mut)]
	pub contract_state: Account<'info, ContractState>,
	#[account(signer, address = contract_state.oracle)]
	pub oracle: AccountInfo<'info>,
}


