use anchor_lang::prelude::*;

use crate::accounts::*;
use crate::errors::*;

#[program]
pub mod betting_contract {
	use super::*;

	pub fn initialize_oracle(ctx: Context<InitializeOracle>, oracle: Pubkey) -> ProgramResult {
		let contract_state = &mut ctx.accounts.contract_state;
		contract_state.initialize_oracle(oracle, ctx.program_id);
		Ok(())
	}

	pub fn change_phase(ctx: Context<ChangePhase>, new_phase: ContractPhase) -> ProgramResult {
		let contract_state = &mut ctx.accounts.contract_state;
		// Ensure the caller is the authorized oracle
		if ctx.accounts.oracle.key != &contract_state.oracle {
			return Err(ProgramError::Custom(Unauthorized));
		}
		contract_state.change_phase(new_phase);
		Ok(())
	}

	pub fn place_bet(ctx: Context<PlaceBet>, bet: Bet) -> ProgramResult {
		let betting_state = &mut ctx.accounts.contract_state.betting_state;
		betting_state.place_bet(bet)
	}

	pub fn start_match<'a, 'b, 'c, 'info>(
		ctx: Context<'a, 'b, 'c, 'info, StartMatch<'info>>,
	) -> ProgramResult {
		let match_state = &mut ctx.accounts.match_state;
		match_state.start_match(ctx)
	}

	pub fn update_match_result(ctx: Context<UpdateMatchResult>, new_result: Team, oracle_sig: Vec<u8>, oracle_data: Vec<u8>) -> ProgramResult {
		let contract_state = &mut ctx.accounts.contract_state;
		contract_state.update_match_result(new_result, oracle_sig, oracle_data)
	}

	pub fn distribute_payouts(ctx: Context<DistributePayouts>) -> ProgramResult {
		let financial_state = &mut ctx.accounts.match_state;
		match_state.distribute_payouts()
	}
}
