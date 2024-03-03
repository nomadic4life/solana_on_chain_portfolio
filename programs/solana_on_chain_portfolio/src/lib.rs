use anchor_lang::prelude::*;

declare_id!("CSg17Be8aubUBAcjroEifZLf5WdFgMWiYBTVmmfxgHPn");

#[program]
pub mod solana_on_chain_portfolio {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let Initialize {
            new_program_header, ..
        } = ctx.accounts;

        new_program_header.is_initialized = true;
        new_program_header.is_authority = true;
        new_program_header.nonce = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8,
        seeds = [b"authority"],
        bump
    )]
    pub new_program_header: Account<'info, ProgramHeader>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct ProgramHeader {
    //      SEEDS:
    //          "authority"
    //      DATA:
    //          portolio_nonce:  u64 -> index to a portfolio, the current nonce is the current index
    pub is_initialized: bool,
    pub is_authority: bool,
    pub nonce: u64,
}

//  Portfolio
//      - indexing system of user portfolios, points to user_id
//      SEEDS:
//          program_authority
//          "portfolio"
//          nonce
//      DATA:
//          user_id:            Pubkey
//          protject_nonce:     u64
//          vouch_nonce:        u64
//          notifications:      Vec<MessageNotification>
//      LOGIC:
//          create  ->  created when new profile is created

//  Profile
//      SEEDS:
//          program_authority
//          "profile"
//          user_id
//      DATA:
//          authority:      Pubkey
//          picture_url:    String
//          content:        Vec<String>  -> {\"FIELD\":\"TEXT\"} -> FIELD : [bio, description, etc..]
//          social_media:   Vec<String>
//          tags:           Vec<tag>
//      LOGIC:
//          create  ->  user can make new profile
//          read    ->  data is queriable
//          update  ->  [user authority]
//          delete  ->  [user authority]

// ProjectHeader
//      SEEDS:
//          program_authority
//          "project-header"
//          user_id
//      DATA:
//          authority:      Pubkey ? not need if program is authority
//          nonce:          u64 -> number of projects
//      LOGIC:
//          create  ->  [program] created when new profile is created
//          read    ->  data is queriable
//          update  ->  [program]
//          delete  ->  [program]

// Project
//      SEEDS:
//          program_authority
//          "project"
//          user_id
//          nonce
//      DATA:
//          authority:      Pubkey
//          project_url:    String
//          tags:           Vec<Tag>
//      LOGIC:
//          create  ->  [authority]
//          read    ->  data is queriable
//          update  ->  [authority]
//          delete  ->  [authority]

// Tag
//      DATA:
//          name:   String

//  MessageNotification
//      DATA:
//          sender:     Pubkey
//          nonce:      u64
//          size:       u64
//      LOGIC:
//          create  -> [PROGRAM] appends to portfolio_header.notifications
//          read    -> data is queriable
//          update  -> [PROGRAM] updates notification at specified index, verifies sender == signer
//          delete  -> [AUTHORITY] notification is deleted after read, authority == user_id

//  ChatHeader
//      SEEDS:
//          program_authority
//          "chat"
//          portfolio.user_id
//          employeer.user_id
//      DATA:
//          portfolio_user_id:  Pubkey
//          employeer_user_id:  Pubkey
//          nonce:              u64

// Message
//      SEEDS:
//          chat_header_id
//          "chat"
//          nonce
//      DATA:
//          recipient:  Pubkey
//          sender:     Pubkey
//          content:    String

// Voucher
//      SEEDS:
//          program_autority
//          portfolio_user_id
//          nonce
//      DATA:
//          content:    String
//          rating:     Rating

// Rating -> ENUM [0,1,2,3,4,5]
