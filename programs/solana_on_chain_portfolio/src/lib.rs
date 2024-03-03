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

        return Ok(());
    }

    pub fn initialize_profile(ctx: Context<InitializeProfile>) -> Result<()> {
        let InitializeProfile {
            authority,
            new_portfolio,
            new_profile,
            program_authority,
            ..
        } = ctx.accounts;

        new_portfolio.user_id = authority.key();
        new_portfolio.protject_nonce = 0;
        new_portfolio.vouch_nonce = 0;
        new_portfolio.notifications = Vec::new();

        new_profile.authority = authority.key();
        new_profile.index = program_authority.nonce;

        program_authority.nonce += 1;

        return Ok(());
    }

    pub fn update_profile(ctx: Context<UpdateProfile>, params: Vec<ProfileParams>) -> Result<()> {
        let UpdateProfile { profile, .. } = ctx.accounts;

        for param in params {
            match param {
                // ProfileParams::Tags { data } => {},
                ProfileParams::Picture { data } => {
                    profile.picture_url = data;
                }

                ProfileParams::Content { data } => {
                    for item in data {
                        match item {
                            Method::Append { content } => {
                                profile.content.push(content);
                            }

                            Method::Update { index, content } => {
                                if profile.content[index].field == content.field {
                                    profile.content[index] = content;
                                }
                            }

                            Method::Delete { index } => {
                                profile.content.swap_remove(index);
                            }
                        }
                    }
                }

                ProfileParams::Social { data } => {
                    for item in data {
                        match item {
                            Method::Append { content } => {
                                profile.social_media.push(content);
                            }

                            Method::Update { index, content } => {
                                if profile.social_media[index].field == content.field {
                                    profile.social_media[index] = content;
                                }
                            }

                            Method::Delete { index } => {
                                profile.social_media.swap_remove(index);
                            }
                        }
                    }
                }
            }
        }

        return Ok(());
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

#[derive(Accounts)]
pub struct InitializeProfile<'info> {
    // create profile
    // create portfolio
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"authority"],
        bump = program_authority.bump
    )]
    pub program_authority: Account<'info, ProgramHeader>,

    #[account(
        init,
        payer = authority,
        space = 8,
        seeds = [
            program_authority.key().as_ref(),
            b"profile",
            authority.key().as_ref()
        ],
        bump
    )]
    pub new_profile: Account<'info, Profile>,

    #[account(
        init,
        payer = authority,
        space = 8,
        seeds = [
            program_authority.key().as_ref(),
            b"portolio",
            program_authority.nonce.to_ne_bytes().as_ref()
        ],
        bump
    )]
    pub new_portfolio: Account<'info, Portfolio>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority
    )]
    pub profile: Account<'info, Profile>,
}

#[account]
pub struct ProgramHeader {
    //      SEEDS:
    //          "authority"
    //      DATA:
    //          portolio_nonce:  u64 -> index to a portfolio, the current nonce is the current index
    pub is_initialized: bool,
    pub is_authority: bool,
    pub bump: u8,
    pub nonce: u64,
}

#[account]
pub struct Profile {
    //  Profile
    //      SEEDS:
    //          program_authority
    //          "profile"
    //          user_id
    //      DATA:
    pub authority: Pubkey,
    pub index: u64,
    pub picture_url: String,
    pub content: Vec<Content>,
    pub social_media: Vec<Content>,
    // pub tags: Vec<String>,
    //      LOGIC:
    //          create  ->  user can make new profile
    //          read    ->  data is queriable
    //          update  ->  [user authority]
    //          delete  ->  [user authority]
}

#[account]
pub struct Portfolio {
    //  Portfolio
    //      - indexing system of user portfolios, points to user_id
    //      SEEDS:
    //          program_authority
    //          "portfolio"
    //          nonce
    //      DATA:
    pub user_id: Pubkey,
    pub protject_nonce: u64,
    pub vouch_nonce: u64,
    pub notifications: Vec<MessageNotification>,
    //      LOGIC:
    //          create  ->  created when new profile is created
    //          read    ->  portfolio is queriable
    //          update  ->  [PROGRAM] updates nonce when new portfolio is created
    //          delete  ->  delete reference?
}

#[account]
pub struct Content {
    pub field: String,
    pub data: String,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub enum ProfileParams {
    Content { data: Vec<Method> },
    Social { data: Vec<Method> },

    // to update = content.data
    // Tags { data: Vec<Method> },
    Picture { data: String },
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub enum Method {
    Append { content: Content },
    Update { index: usize, content: Content },
    Delete { index: usize },
}

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

#[account]
pub struct MessageNotification {
    //  MessageNotification
    //      DATA:
    pub sender: Pubkey,
    pub nonce: u64,
    pub size: u64,
    //      LOGIC:
    //          create  -> [PROGRAM] appends to portfolio_header.notifications
    //          read    -> data is queriable
    //          update  -> [PROGRAM] updates notification at specified index, verifies sender == signer
    //          delete  -> [AUTHORITY] notification is deleted after read, authority == user_id
}

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
