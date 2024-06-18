use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, TokenAccount, Token};
use solana_program::entrypoint::ProgramResult;

use crate::accounts::*;

impl FinancialState {
	pub fn distribute_payouts(&mut self, contract_state: &mut ContractState) -> ProgramResult {
		if contract_state.phase != ContractPhase::Result {
			return Err(ProgramError::Custom(4)); // Not in result phase
		}
		let total_pool = .total_red + self.total_blue;
		let winner_pool = match match_state.match_result {
			Some(Team::Red) => betting_state.total_red,
			Some(Team::Blue) => betting_state.total_blue,
			None => {
				return Err(ProgramError::Custom(5)); // No winner set
			}
		};
		for bet in match_state.bets {
			if bet.team == match_state.match_result.clone().unwrap() {
				let _user_payout = (bet.amount * total_pool) / winner_pool;
				distribute_payouts(bet, _user_payout);
			}
		}
		// replace with reinitialize function
		betting_state.bets.clear();
		betting_state.total_red = 0;
		betting_state.total_blue = 0;
		contract_state.phase = ContractPhase::Betting; // Access through `contract_state`
		Ok(())
	}

	fn refund_bet(
		&self,
		bet: &Bet,
		token_program: &Program<Token>,
		contract_token_account: &Account<TokenAccount>,
		bettor_token_account: &Account<TokenAccount>,
		authority: &Signer
	) {
		let cpi_accounts = Transfer {
			from: contract_token_account.to_account_info(),
			to: bettor_token_account.to_account_info(),
			authority: authority.to_account_info(),
		};
		let cpi_context = CpiContext::new(token_program.to_account_info(), cpi_accounts);
		token::transfer(cpi_context, bet.amount)?;
	}
}

pub fn process_payout(ctx: Context<Payout>, amount: u64) {
	let cpi_accounts = Transfer {
		from: ctx.accounts.from.to_account_info(),
		to: ctx.accounts.to.to_account_info(),
		authority: ctx.accounts.authority.to_account_info(),
	};
	let cpi_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
	token::transfer(cpi_context, amount)?;
}