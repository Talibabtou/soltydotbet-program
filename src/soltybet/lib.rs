/*
- Define the data structures: You'll need to define the data structures that will hold the state of your smart contract.
This could include the current phase, the bets placed by users, and the result of the match.

- Implement phase transitions: You'll need to implement functions that transition the contract from one phase to the next.
This could include a function to start the betting phase, a function to start the match phase, and a function to start the result phase.

- Implement betting: You'll need to implement a function that allows users to place bets.
This function should only be callable during the betting phase, and it should record the user's bet and the amount of SOL they've bet.

- Implement match locking: You'll need to implement a function that locks the match, preventing further bets from being placed.
This function should only be callable during the match phase.

- Implement result announcement: You'll need to implement a function that announces the result of the match.
This function should only be callable during the result phase, and it should record the result of the match.

- Implement payout calculation: You'll need to implement a function that calculates the payout for each user
based on the result of the match and the bets that were placed. This function should only be callable during the result phase.

- Implement payout distribution: You'll need to implement a function that distributes the calculated payouts to the users.
This function should only be callable during the result phase.

- Write tests: Finally, you'll need to write tests for each of these functions to ensure they work as expected.
*/

use anchor_lang::prelude::*;

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

	#[state]
	pub struct ContractState {
		current_phase: ContractPhase,
		match_id: u64,
		match_result: Team,
		bets: Vec<Bet>,
	}

	pub struct Bet {
		user: Pubkey,
		team: Team,
		amount: u64,
		referal: Option<RefPubKey>,
	}

	impl ContractState {
		pub fn new() -> Self {
			Self {
				current_phase: ContractPhase::Betting,
				match_id: 0,
				match_result: None,
				bets: Vec::new(),
			}
		}

		pub fn calculate_totals(&self) -> (u64, u64) {
			let mut total_red = 0;
			let mut total_blue = 0;
	
			for Bet in &self.bets {
				if Bet.team == 0 {
					total_red += Bet.amount;
				} else if bet.team == 1 {
					total_blue += Bet.amount;
				}
			}
	
			(total_red, total_blue)
		}

		pub fn handle_phase(&mut self) {
			match self.current_phase {
				ContractPhase::Betting => {
					// Handle betting phase
				}
				ContractPhase::Match => {
					// Handle match phase
				}
				ContractPhase::Result => {
					// Handle result phase
					// Increment match_id for the next match
					self.match_id += 1;
				}
			}
		}
	}
}

	#[access_control(ctx.accounts.user.key() == ctx.accounts.bet.user)] {
	pub fn place_bet (
		ctx: Context<PlaceBet>,
		team: Team,
		amount: u64
	) -> ProgramResult {
		let bet = Bet {
			user: *ctx.accounts.user.key(),
			team,
			amount,
			referal: None,
		};
		ctx.accounts.bet.try_borrow_mut()?.push(bet);
		Ok(())
	}

	pub fn start_match(ctx: Context<StartMatch>) -> ProgramResult {
		// TODO: Implement match start
		Ok(())
	}

	pub fn announce_result(ctx: Context<AnnounceResult>, result: u8) -> ProgramResult {
		// TODO: Implement result announcement
		Ok(())
	}

	pub fn distribute_payouts(ctx: Context<DistributePayouts>) -> ProgramResult {
		// TODO: Implement payout distribution
		Ok(())
	}
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
	#[account(mut)]
	pub user: Signer<'info>,
	#[account(mut)]
	pub bet: Account<'info, Bet>,
}

#[derive(Accounts)]
pub struct StartMatch<'info> {
	// TODO: Define accounts needed to start match
}

#[derive(Accounts)]
pub struct AnnounceResult<'info> {
	// TODO: Define accounts needed to announce result
}

#[derive(Accounts)]
pub struct DistributePayouts<'info> {
	// TODO: Define accounts needed to distribute payouts
}
