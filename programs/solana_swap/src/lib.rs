use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Token, Transfer, TokenAccount};

declare_id!("GorTDpuWrsRf3THg5EpS2i3PLuncQZEhYRBi8ZBKSDo5");

#[program]
pub mod solana_swap {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed: [u8; 8]) -> Result<()> {
        let swap_pool = &mut ctx.accounts.swap_pool;
        swap_pool.token_a_mint = ctx.accounts.token_a_mint.key();
        swap_pool.token_b_mint = ctx.accounts.token_b_mint.key();
        swap_pool.token_a_account = ctx.accounts.token_a_account.key();
        swap_pool.token_b_account = ctx.accounts.token_b_account.key();
        swap_pool.authority = ctx.accounts.authority.key();
        swap_pool.seed = seed.to_vec();

        msg!("交换池初始化成功！");
        Ok(())
    }

    pub fn swap_a_to_b(ctx: Context<Swap>, amount: u64) -> Result<()> {
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

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                transfer_a_to_pool,
            ),
            amount,
        )?;

        // 从池子的代币B账户转移到用户的代币B账户
        let bump = ctx.bumps.swap_pool;
        let seed = &ctx.accounts.swap_pool.seed;
        let seeds = &[b"swap_authority".as_ref(), seed.as_slice(), &[bump]];
        let signer = &[&seeds[..]];
        
        let transfer_b_to_user = Transfer {
            from: ctx.accounts.pool_token_b.to_account_info(),
            to: ctx.accounts.user_token_b.to_account_info(),
            authority: ctx.accounts.swap_pool.to_account_info(),
        };

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                transfer_b_to_user,
                signer,
            ),
            amount, // 1:1比例
        )?;

        msg!("代币A兑换成代币B成功！金额: {}", amount);
        Ok(())
    }

    pub fn swap_b_to_a(ctx: Context<Swap>, amount: u64) -> Result<()> {
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

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                transfer_b_to_pool,
            ),
            amount,
        )?;

        // 从池子的代币A账户转移到用户的代币A账户
        let bump = ctx.bumps.swap_pool;
        let seed = &ctx.accounts.swap_pool.seed;
        let seeds = &[b"swap_authority".as_ref(), seed.as_slice(), &[bump]];
        let signer = &[&seeds[..]];
        
        let transfer_a_to_user = Transfer {
            from: ctx.accounts.pool_token_a.to_account_info(),
            to: ctx.accounts.user_token_a.to_account_info(),
            authority: ctx.accounts.swap_pool.to_account_info(),
        };

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                transfer_a_to_user,
                signer,
            ),
            amount, // 1:1比例
        )?;

        msg!("代币B兑换成代币A成功！金额: {}", amount);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(seed: [u8; 8])]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = authority, 
        space = SwapPool::SIZE,
        seeds = [b"swap_authority".as_ref(), seed.as_ref()],
        bump
    )]
    pub swap_pool: Account<'info, SwapPool>,
    
    pub token_a_mint: Account<'info, token::Mint>,
    
    pub token_b_mint: Account<'info, token::Mint>,
    
    #[account(
        constraint = token_a_account.mint == token_a_mint.key() @ ErrorCode::InvalidMint,
        constraint = token_a_account.owner == swap_pool.key() @ ErrorCode::InvalidOwner
    )]
    pub token_a_account: Account<'info, TokenAccount>,
    
    #[account(
        constraint = token_b_account.mint == token_b_mint.key() @ ErrorCode::InvalidMint,
        constraint = token_b_account.owner == swap_pool.key() @ ErrorCode::InvalidOwner
    )]
    pub token_b_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(
        mut,
        seeds = [b"swap_authority".as_ref(), swap_pool.seed.as_slice()],
        bump,
    )]
    pub swap_pool: Account<'info, SwapPool>,
    
    #[account(
        mut,
        constraint = pool_token_a.mint == swap_pool.token_a_mint @ ErrorCode::InvalidMint,
        constraint = pool_token_a.key() == swap_pool.token_a_account @ ErrorCode::InvalidAccount,
        constraint = pool_token_a.owner == swap_pool.key() @ ErrorCode::InvalidOwner
    )]
    pub pool_token_a: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = pool_token_b.mint == swap_pool.token_b_mint @ ErrorCode::InvalidMint,
        constraint = pool_token_b.key() == swap_pool.token_b_account @ ErrorCode::InvalidAccount,
        constraint = pool_token_b.owner == swap_pool.key() @ ErrorCode::InvalidOwner
    )]
    pub pool_token_b: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = user_token_a.mint == swap_pool.token_a_mint @ ErrorCode::InvalidMint,
        constraint = user_token_a.owner == user.key() @ ErrorCode::InvalidOwner
    )]
    pub user_token_a: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = user_token_b.mint == swap_pool.token_b_mint @ ErrorCode::InvalidMint,
        constraint = user_token_b.owner == user.key() @ ErrorCode::InvalidOwner
    )]
    pub user_token_b: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct SwapPool {
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub token_a_account: Pubkey,
    pub token_b_account: Pubkey,
    pub authority: Pubkey,
    pub seed: Vec<u8>,
}

impl SwapPool {
    pub const SIZE: usize = 8 + // 歧视器
        32 + // token_a_mint
        32 + // token_b_mint
        32 + // token_a_account
        32 + // token_b_account
        32 + // authority
        4 + 32; // seed (4字节长度 + 最多32字节数据)
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
