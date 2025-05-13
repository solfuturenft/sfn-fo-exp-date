use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("id");

#[program]
pub mod nft_futures {
    use super::*;

    pub fn create_future(ctx: Context<CreateFuture>, strike_price: u64, expiry: i64) -> Result<()> {
        let future = &mut ctx.accounts.future_account;
        future.creator = ctx.accounts.creator.key();
        future.strike_price = strike_price;
        future.expiry = expiry;
        future.nft_mint = ctx.accounts.nft_mint.key();
        future.oracle = ctx.accounts.oracle.key();
        Ok(())
    }

    pub fn join_future(ctx: Context<JoinFuture>) -> Result<()> {
        let future = &mut ctx.accounts.future_account;
        future.taker = Some(ctx.accounts.taker.key());
        Ok(())
    }

    pub fn settle_future(ctx: Context<SettleFuture>, oracle_price: u64) -> Result<()> {
        let future = &ctx.accounts.future_account;
        require!(Clock::get()?.unix_timestamp >= future.expiry, FutureError::TooEarly);
        require_keys_eq!(ctx.accounts.oracle.key(), future.oracle, FutureError::Unauthorized);

        if oracle_price >= future.strike_price {
            token::transfer(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.escrow_nft_account.to_account_info(),
                        to: ctx.accounts.taker_nft_account.to_account_info(),
                        authority: ctx.accounts.escrow_signer.to_account_info(),
                    },
                ).with_signer(&[&[b"future", &[ctx.bumps.escrow_signer]]]),
                1,
            )?;
        } else {
            token::transfer(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.escrow_nft_account.to_account_info(),
                        to: ctx.accounts.creator_nft_account.to_account_info(),
                        authority: ctx.accounts.escrow_signer.to_account_info(),
                    },
                ).with_signer(&[&[b"future", &[ctx.bumps.escrow_signer]]]),
                1,
            )?;
        }
        Ok(())
    }

    pub fn create_option(ctx: Context<CreateOption>, strike_price: u64, expiry: i64, premium: u64) -> Result<()> {
        let option = &mut ctx.accounts.option_account;
        option.writer = ctx.accounts.writer.key();
        option.strike_price = strike_price;
        option.expiry = expiry;
        option.premium = premium;
        option.nft_mint = ctx.accounts.nft_mint.key();
        option.oracle = ctx.accounts.oracle.key();
        Ok(())
    }

    pub fn exercise_option(ctx: Context<ExerciseOption>, oracle_price: u64) -> Result<()> {
        let option = &ctx.accounts.option_account;
        require!(Clock::get()?.unix_timestamp < option.expiry, FutureError::TooLate);
        require_keys_eq!(ctx.accounts.oracle.key(), option.oracle, FutureError::Unauthorized);

        if oracle_price >= option.strike_price {
            token::transfer(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.option_escrow_nft_account.to_account_info(),
                        to: ctx.accounts.holder_nft_account.to_account_info(),
                        authority: ctx.accounts.escrow_signer.to_account_info(),
                    },
                ).with_signer(&[&[b"option", &[ctx.bumps.escrow_signer]]]),
                1,
            )?;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateFuture<'info> {
    #[account(init, payer = creator, space = 8 + 32 + 8 + 8 + 32 + 32)]
    pub future_account: Account<'info, FutureContract>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub nft_mint: Account<'info, Mint>,
    pub oracle: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct JoinFuture<'info> {
    #[account(mut)]
    pub future_account: Account<'info, FutureContract>,
    pub taker: Signer<'info>,
}

#[derive(Accounts)]
pub struct SettleFuture<'info> {
    #[account(mut)]
    pub future_account: Account<'info, FutureContract>,
    #[account(mut)]
    pub escrow_nft_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub taker_nft_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub creator_nft_account: Account<'info, TokenAccount>,
    #[account(seeds = [b"future"], bump)]
    pub escrow_signer: AccountInfo<'info>,
    pub oracle: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CreateOption<'info> {
    #[account(init, payer = writer, space = 8 + 32 + 8 + 8 + 8 + 32 + 32)]
    pub option_account: Account<'info, OptionContract>,
    #[account(mut)]
    pub writer: Signer<'info>,
    pub nft_mint: Account<'info, Mint>,
    pub oracle: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExerciseOption<'info> {
    #[account(mut)]
    pub option_account: Account<'info, OptionContract>,
    #[account(mut)]
    pub option_escrow_nft_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub holder_nft_account: Account<'info, TokenAccount>,
    #[account(seeds = [b"option"], bump)]
    pub escrow_signer: AccountInfo<'info>,
    pub oracle: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct FutureContract {
    pub creator: Pubkey,
    pub taker: Option<Pubkey>,
    pub strike_price: u64,
    pub expiry: i64,
    pub nft_mint: Pubkey,
    pub oracle: Pubkey,
}

#[account]
pub struct OptionContract {
    pub writer: Pubkey,
    pub strike_price: u64,
    pub expiry: i64,
    pub premium: u64,
    pub nft_mint: Pubkey,
    pub oracle: Pubkey,
}

#[error_code]
pub enum FutureError {
    #[msg("Settlement attempted too early.")]
    TooEarly,
    #[msg("Option expired.")]
    TooLate,
    #[msg("Unauthorized oracle.")]
    Unauthorized,
}
