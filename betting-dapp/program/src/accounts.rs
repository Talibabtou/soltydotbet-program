use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::HashMap;

#[account]
pub struct ContractState {
	pub matches: [Option<MatchData>; 3], // Fixed-size array to store the last 3 matches
	pub current_match_index: usize,
	pub phase: ContractPhase,
	pub oracle: Pubkey,
	pub initialized: bool,
}

#[derive(BorshDeserialize, BorshSerialize, PartialEq, Clone)]
pub enum ContractPhase {
	Betting,
	Match,
	Result,
}

#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct MatchData {
	pub match_id: u64,
	pub betting_state: BettingState,
	pub match_state: MatchState,
	pub financial_state: FinancialState,
}

#[derive(Accounts)]
pub struct InitializeOracle<'info> {
	#[account(mut)]
	pub contract_state: Account<'info, ContractState>,
	pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
	#[account(init, payer = user, space = 8 + std::mem::size_of::<ContractState>())]
	pub contract_state: Account<'info, ContractState>,
	#[account(mut)]
	pub user: Signer<'info>,
	pub authority: Signer<'info>,
	pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ChangePhase<'info> {
	#[account(mut)]
	pub contract_state: Account<'info, ContractState>,
	pub token_program: Program<'info, Token>,
	#[account(mut)]
	pub contract_token_account: Account<'info, TokenAccount>,
	#[account(mut)]
	pub bettor_token_accounts: Account<'info, HashMap<Pubkey, Account<'info, TokenAccount>>>,
	#[account(signer, address = contract_state.oracle)]
	pub oracle: AccountInfo<'info>,
}

#[account]
pub struct BettingState {
	pub bets: Vec<Bet>,
	pub bet_weights: Vec<BetWeight>,
	pub total_red: u64,
	pub total_blue: u64,
}

#[account]
pub struct Bet {
	pub user: Pubkey,
	pub team: Team,
	pub amount: u64,
	pub referral: Option<Pubkey>,
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
	#[account(mut)]
	pub user: Signer<'info>,
	#[account(mut)]
	pub contract_state: Account<'info, ContractState>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BetWeight {
	pub user: Pubkey,
	pub team: Team,
	pub weight: f64,
}

#[account]
pub struct MatchState {
	pub match_result: Option<Team>,
	pub red_to_blue_rate: f64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Team {
	Red,
	Blue,
}

#[account(BorshDeserialize, BorshSerialize)]
pub struct FinancialState {
	pub house_fee: u64,
	pub referral_fees: HashMap<Pubkey, u64>,
	pub payouts: HashMap<Pubkey, u64>,
}

#[derive(Accounts)]
pub struct Payout<'info> {
	#[account(mut)]
	pub from: Account<'info, TokenAccount>,
	#[account(mut)]
	pub to: Account<'info, TokenAccount>,
	pub authority: Signer<'info>,
	pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ReferralFee<'info> {
	#[account(mut)]
	pub contract_state: Account<'info, ContractState>,
	#[account(mut)]
	pub user: Signer<'info>,
}
