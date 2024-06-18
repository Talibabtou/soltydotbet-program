use anchor_lang::prelude::*;

impl ContractState {
	pub fn initialize(oracle: Pubkey) -> Self {
		let mut instance = Self {
			self.phase: ContractPhase::Betting,
			match_id: 0,
			match_result: None,
			bets: Vec::new(),
			bets.bet_weights: Vec::new(),
			red_to_blue_rate: 1.0,
			bets.total_red: 0,
			bets.total_blue: 0,
			bets.house_fee: 0,
			bets.referral_fees: HashMap::new(),
			oracle: oracle,
		};
		instance.launch_pda_oracle(oracle);
		instance
	}

	pub fn cancel_and_refund(
		&mut self,
		match_state: &mut MatchState,
		betting_state: &mut BettingState,
		financial_state: &mut FinancialState,
		token_program: &Program<Token>,
		contract_token_account: &Account<TokenAccount>,
		bettor_token_accounts: &HashMap<Pubkey, Account<TokenAccount>>,
		authority: &Signer,
	) {
		// Iterate through all bets and refund them
		for bet in &betting_state.bets {
			if let Some(bettor_token_account) = bettor_token_accounts.get(&bet.user) {
				self.refund_bet(bet, token_program, contract_token_account, bettor_token_account, authority);
			}
		}
		// Reset betting state information
		betting_state.bets.clear();
		betting_state.bet_weights.clear();
		betting_state.total_red = 0;
		betting_state.total_blue = 0;
		financial_state.house_fee = 0;
		financial_state.referral_fees.clear();
		match_state.match_result = None;
		self.phase = ContractPhase::Betting;
	}

	pub fn quick_reinit(&mut self, match_id: u64, match_result: Option<Team>) {
		self.match_id = match_id;
		self.match_result = match_result;
	}
}