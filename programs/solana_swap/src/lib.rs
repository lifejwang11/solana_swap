use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

declare_id!("GorTDpuWrsRf3THg5EpS2i3PLuncQZEhYRBi8ZBKSDo5");

#[program]
pub mod solana_swap {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("交换池初始化成功！");
        Ok(())
    }

    pub fn swap_a_to_b(ctx: Context<SwapAToB>, amount: u64) -> Result<()> {
        // 验证数量大于0
        require!(amount > 0, ErrorCode::InvalidAmount);
        
        // 验证用户账户有足够的代币A
        require!(
            ctx.accounts.user_token_a.amount >= amount,
            ErrorCode::InsufficientFunds
        );
        
        // 验证池子有足够的代币B
        require!(
            ctx.accounts.pool_token_b.amount >= amount,
            ErrorCode::InsufficientFunds
        );

        // 从用户的代币A账户转移到池子的代币A账户
        let transfer_a_to_pool = Transfer {
            from: ctx.accounts.user_token_a.to_account_info(),
            to: ctx.accounts.pool_token_a.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_a_to_pool,
        );

        anchor_spl::token::transfer(cpi_ctx, amount)?;

        // 从池子的代币B账户转移到用户的代币B账户
        let bump = ctx.bumps.pool_authority;
        let seeds = &[b"pool_authority".as_ref(), &[bump]];
        let signer = &[&seeds[..]];
        
        let transfer_b_to_user = Transfer {
            from: ctx.accounts.pool_token_b.to_account_info(),
            to: ctx.accounts.user_token_b.to_account_info(),
            authority: ctx.accounts.pool_authority.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_b_to_user,
            signer,
        );

        anchor_spl::token::transfer(cpi_ctx, amount)?;

        msg!("代币A兑换成代币B成功！金额: {}", amount);
        Ok(())
    }

    pub fn swap_b_to_a(ctx: Context<SwapBToA>, amount: u64) -> Result<()> {
        // 验证数量大于0
        require!(amount > 0, ErrorCode::InvalidAmount);
        
        // 验证用户账户有足够的代币B
        require!(
            ctx.accounts.user_token_b.amount >= amount,
            ErrorCode::InsufficientFunds
        );
        
        // 验证池子有足够的代币A
        require!(
            ctx.accounts.pool_token_a.amount >= amount,
            ErrorCode::InsufficientFunds
        );

        // 从用户的代币B账户转移到池子的代币B账户
        let transfer_b_to_pool = Transfer {
            from: ctx.accounts.user_token_b.to_account_info(),
            to: ctx.accounts.pool_token_b.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_b_to_pool,
        );

        anchor_spl::token::transfer(cpi_ctx, amount)?;

        // 从池子的代币A账户转移到用户的代币A账户
        let bump = ctx.bumps.pool_authority;
        let seeds = &[b"pool_authority".as_ref(), &[bump]];
        let signer = &[&seeds[..]];
        
        let transfer_a_to_user = Transfer {
            from: ctx.accounts.pool_token_a.to_account_info(),
            to: ctx.accounts.user_token_a.to_account_info(),
            authority: ctx.accounts.pool_authority.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_a_to_user,
            signer,
        );

        anchor_spl::token::transfer(cpi_ctx, amount)?;

        msg!("代币B兑换成代币A成功！金额: {}", amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // 创建PDA作为池子代币账户的管理者
    /// CHECK: 这是一个派生的PDA，用作池子权限
    #[account(
        init_if_needed,
        payer = admin,
        seeds = [b"pool_authority"],
        bump,
        space = 8
    )]
    pub pool_authority: AccountInfo<'info>,

    // 代币A的池子账户
    #[account(
        init_if_needed,
        payer = admin,
        seeds = [b"token_pool_a", token_a_mint.key().as_ref()],
        token::mint = token_a_mint,
        token::authority = pool_authority,
        bump
    )]
    pub pool_token_a: Account<'info, TokenAccount>,

    // 代币B的池子账户
    #[account(
        init_if_needed,
        payer = admin,
        seeds = [b"token_pool_b", token_b_mint.key().as_ref()],
        token::mint = token_b_mint,
        token::authority = pool_authority,
        bump
    )]
    pub pool_token_b: Account<'info, TokenAccount>,

    pub token_a_mint: Account<'info, Mint>,
    pub token_b_mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SwapAToB<'info> {
    /// CHECK: 这是一个派生的PDA，用作池子权限
    #[account(
        mut,
        seeds = [b"pool_authority"],
        bump
    )]
    pub pool_authority: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [b"token_pool_a", token_a_mint.key().as_ref()],
        bump,
        token::mint = token_a_mint,
        token::authority = pool_authority,
    )]
    pub pool_token_a: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"token_pool_b", token_b_mint.key().as_ref()],
        bump,
        token::mint = token_b_mint,
        token::authority = pool_authority,
    )]
    pub pool_token_b: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_a: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_b: Account<'info, TokenAccount>,

    pub token_a_mint: Account<'info, Mint>,
    pub token_b_mint: Account<'info, Mint>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SwapBToA<'info> {
    /// CHECK: 这是一个派生的PDA，用作池子权限
    #[account(
        mut,
        seeds = [b"pool_authority"],
        bump
    )]
    pub pool_authority: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [b"token_pool_a", token_a_mint.key().as_ref()],
        bump,
        token::mint = token_a_mint,
        token::authority = pool_authority,
    )]
    pub pool_token_a: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"token_pool_b", token_b_mint.key().as_ref()],
        bump,
        token::mint = token_b_mint,
        token::authority = pool_authority,
    )]
    pub pool_token_b: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_a: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_b: Account<'info, TokenAccount>,

    pub token_a_mint: Account<'info, Mint>,
    pub token_b_mint: Account<'info, Mint>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("铸币厂不匹配")]
    InvalidMint,
    #[msg("账户所有者不匹配")]
    InvalidOwner,
    #[msg("账户不匹配")]
    InvalidAccount,
    #[msg("余额不足")]
    InsufficientFunds,
    #[msg("无效的交换金额")]
    InvalidAmount,
}
