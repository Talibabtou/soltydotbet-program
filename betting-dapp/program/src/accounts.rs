use anchor_lang::solana_program::program::account_info::AccountInfo;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_lang::solana_program::program::account_info::Signer;
use anchor_lang::solana_program::sysvar::Sysvar;
use anchor_lang::prelude::{Account, Accounts, Pubkey};

#[account]
pub struct ContractState {
	pub betting_state: BettingState,
	pub match_state: MatchState,
	pub financial_state: FinancialState,
	pub phase: ContractPhase,
	pub oracle: Pubkey,
}

#[derive(Accounts)]
pub enum ContractPhase {
	Betting,
	Match,
	Result,
}

#[derive(Accounts)]
pub struct InitializeOracle<'info> {
	#[account(mut)]
	pub contract_state: Account<'info, ContractState>,
	pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
	#[account(init, payer = user, seeds = [b"red"], bump, space = 1024)]
	pub red_bets_account: Account<'info, BetPool>,
	
	#[account(init, payer = user, seeds = [b"blue"], bump, space = 1024)]
	pub blue_bets_account: Account<'info, BetPool>,

	#[account(mut)]
	pub user: Signer<'info>,
	pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ChangePhase<'info> {
	#[account(mut)]
	pub contract_state: Account<'info, ContractPhase>,
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
	pub match_id: u64,
	pub match_result: Option<Team>,
	pub red_to_blue_rate: f64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Team {
	Red,
	Blue,
}

#[account]
pub struct FinancialState {
	pub house_fee: u64,
	pub referral_fees: HashMap<ReferralFee>,
}

#[derive(Accounts)]
pub struct ReferralFee<'info> {
	#[account(mut)]
	pub contract_state: Account<'info, ContractState>,
	#[account(mut)]
	pub user: Signer<'info>,
}
