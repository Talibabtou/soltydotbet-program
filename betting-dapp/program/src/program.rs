use anchor_lang::prelude::*;

#[program]
pub mod betting_contract {
	use super::*;

	pub fn initialize(oracle: Pubkey) -> ProgramResult {
		let contract_state = &mut ctx.accounts.contract_state;
		require!(ctx.accounts.user.key == ctx.accounts.authority.key, CustomError::Unauthorized); // Ensure only deployer can call
		require!(!contract_state.initialized, CustomError::AlreadyInitialized); // Ensure it can only be called once
	
		launch(oracle, program_id)
		contract_state.initialize(oracle);
		contract_state.initialized = true;
		Ok(())
	}

	pub fn change_phase(ctx: Context<ChangePhase>, new_phase: ContractPhase) -> ProgramResult {
		let contract_state = &mut ctx.accounts.contract_state;
		let token_program = &ctx.accounts.token_program;
		let contract_token_account = &ctx.accounts.contract_token_account;
		let bettor_token_accounts = &ctx.accounts.bettor_token_accounts;

		contract_state.change_phase(new_phase, token_program, contract_token_account, bettor_token_accounts)?;
		Ok(())
	}

	pub fn place_bet(ctx: Context<PlaceBet>, bet: Bet) -> ProgramResult {
		let betting_state = &mut ctx.accounts.contract_state.betting_state;
		betting_state.place_bet(bet)
	}

	pub fn start_new_match(ctx: Context<StartMatch>, match_data: MatchData) -> ProgramResult {
		let contract_state = &mut ctx.accounts.contract_state;
		// Store the new match data at the current index
		contract_state.matches[contract_state.current_match_index] = Some(match_data);
		// Update the current match index using modulo operation
		contract_state.current_match_index = (contract_state.current_match_index + 1) % 3;
		// Set the phase to Match
		contract_state.phase = ContractPhase::Match;

		Ok(())
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
