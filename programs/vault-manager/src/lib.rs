use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

declare_id!("2eH4VtkkB5X5592hmuQqFQvQ9QKaTEmRZyvQgf9EWyxp");

#[program]
pub mod vault_manager {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn deposit(
        ctx: Context<Deposit>,
        amount: u64,
    ) -> Result<()> {
        msg!("Amount Deposting: {}", amount); //logs the amount deposited
        //creates reference to the senders's token amount from ctx.accounts
        let sender_token_account: &Account<TokenAccount> = &ctx.accounts.sender_token_account;

        //checks the senders has enough tokens to send
        if sender_token_account.amount < amount{
            return Err(VaultError::InsufficientFunds.into());
        }

        // display balance in senders account
        msg!("Senders account balance: {}", sender_token_account.amount);

        // Creats a transfer instruction for SPL token program
        let tx_instruct: anchor_spl::token::Transfer = Transfer {
            from: ctx.accounts.sender_token_account.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };  

        //creates a CPI cross platform invocation Context
        //it combines the token program account with the transfer instruction
        // READ MORE: https://solana.com/docs/core/cpi
        let cpi_ctx: CpiContext<anchor_spl::token::Transfer> = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            tx_instruct
        );

        //executes transfer instruction usign CPI Context
        // ? shows any errors the may occur during transfer
        anchor_spl::token::transfer(cpi_ctx, amount)?;
        Ok(()) // if no erros are thrown is returns Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
		init_if_needed,
		payer = signer,
		seeds = [b"SSF_ACCOUNT_VAULT"],
		bump,
		space = 8
	)]
	/// CHECK: Struct field "token_account_owner_pda" is unsafe, but is not documented.
	token_account_owner_pda: AccountInfo<'info>,

	#[account(mut)]
	signer: Signer<'info>,

	system_program: Program<'info, System>,
	token_program:  Program<'info, Token>,
	rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Deposit<'info>{
	#[account(mut,
		seeds=[b"SSF_ACCOUNT_VAULT"],
		bump
	)]
	token_account_owner_pda: AccountInfo<'info>,
	#[account(
		init_if_needed,
		seeds = [
			b"SSF_PDA_VAULT".as_ref(),
			mint_account.key().as_ref()
		],
		token::mint      = mint_account,
		token::authority = token_account_owner_pda,
		payer            = signer,
		bump
	)]
	pub vault: Account<'info, TokenAccount>,

	#[account(mut)]
	pub signer: Signer<'info>,

	pub mint_account: Account<'info, Mint>,

	#[account(mut)]
	pub sender_token_account: Account<'info, TokenAccount>,

	pub token_program:  Program<'info, Token>,
	pub system_program: Program<'info, System>,
}


#[error_code]
pub enum VaultError {
	#[msg("Insufficient Funds in Wallet!")]
	InsufficientFunds,
}
