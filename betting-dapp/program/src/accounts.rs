use anchor_lang::prelude::*;

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