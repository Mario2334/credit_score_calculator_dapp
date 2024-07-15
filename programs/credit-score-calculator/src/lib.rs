use anchor_lang::prelude::*;

declare_id!("6S5riJKdh8eLBbwE7qWpReRoN9LU14sCXz3WobkVebCc");

#[program]
pub mod credit_score_calculator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,
                      volume_history: u64,
                      wallet_balance: u64,
                      freq_transaction: u64,
                      transaction_mix: u64,
                      new_transactions: u64,) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.volume_history = volume_history;
        my_account.wallet_balance = wallet_balance;
        my_account.freq_transaction = freq_transaction;
        my_account.transaction_mix = transaction_mix;
        my_account.new_transactions = new_transactions;
        my_account.credit_score = calculate_credit_score(
            volume_history,
            wallet_balance,
            freq_transaction,
            transaction_mix,
            new_transactions,
        );
        Ok(())
    }

    pub fn update_volume_history(ctx: Context<Update>, volume_history:u64 ) -> Result<()>{
        let my_account = &mut ctx.accounts.my_account;
        my_account.volume_history = volume_history;
        my_account.credit_score = calculate_credit_score(
            my_account.volume_history,
            my_account.wallet_balance,
            my_account.freq_transaction,
            my_account.transaction_mix,
            my_account.new_transactions,
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer=user, space = 8 + MyAccount::LEN)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}

#[account]
pub struct MyAccount{
    pub volume_history: u64,
    pub wallet_balance: u64,
    pub freq_transaction: u64,
    pub transaction_mix: u64,
    pub new_transactions: u64,
    pub credit_score: u64
}

impl MyAccount {
    const LEN: usize = 8 * 6; // 8 for u64, 4 for string length, 32 for string content
}

fn calculate_credit_score(
    volume_history: u64,
    wallet_balance: u64,
    freq_transaction: u64,
    transaction_mix: u64,
    new_transactions: u64,
) -> u64 {
    let score = (volume_history * 35 / 100)
        + (wallet_balance * 30 / 100)
        + (freq_transaction * 15 / 100)
        + (transaction_mix * 10 / 100)
        + (new_transactions * 10 / 100);
    score.min(850).max(300)
}