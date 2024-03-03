use anchor_lang::prelude::*;
use anchor_lang::solana_program;

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
            new_profile,
            program_authority,
            new_profile_header,
            ..
        } = ctx.accounts;

        new_profile.authority = authority.key();
        new_profile.index = program_authority.nonce;

        new_profile_header.authority = authority.key();
        new_profile_header.vouch_nonce = 0;
        new_profile_header.notifications = Vec::new();

        program_authority.nonce += 1;

        return Ok(());
    }

    pub fn initialize_portfolio(ctx: Context<InitializePortfolio>) -> Result<()> {
        let InitializePortfolio {
            authority,
            new_portfolio,
            new_project_header,
            ..
        } = ctx.accounts;

        new_portfolio.user_id = authority.key();
        new_portfolio.protject_nonce = 0;

        new_project_header.authority = authority.key();
        new_project_header.nonce = 0;

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

    pub fn add_project(ctx: Context<AddProject>, url: String, description: String) -> Result<()> {
        let AddProject {
            authority,
            project_header,
            new_project,
            ..
        } = ctx.accounts;

        new_project.authority = authority.key();
        new_project.project_url = url;
        new_project.description = description;

        project_header.nonce += 1;

        return Ok(());
    }

    pub fn update_project(
        ctx: Context<UpdateProject>,
        url: String,
        description: String,
    ) -> Result<()> {
        let UpdateProject { project, .. } = ctx.accounts;

        project.project_url = url;
        project.description = description;

        return Ok(());
    }

    pub fn initialize_message_header(ctx: Context<InitializeMessageHeader>) -> Result<()> {
        let InitializeMessageHeader {
            sender,
            recipient,
            new_message_header,
            ..
        } = ctx.accounts;

        if recipient.key() < sender.key() {
            new_message_header.members = [recipient.key(), sender.key()];
        } else {
            new_message_header.members = [sender.key(), recipient.key()];
        }

        new_message_header.nonce = 0;
        new_message_header.bump = ctx.bumps.new_message_header;

        return Ok(());
    }

    pub fn post_message(
        ctx: Context<PostMessage>,
        params: MessageParams,
        data: String,
    ) -> Result<()> {
        let PostMessage {
            sender,
            recipient,
            profile_header,
            message,
            message_header,
            ..
        } = ctx.accounts;

        match params {
            // potential attack by spamming the notofication vec
            // added a few checks to reduce that. but best way would use a BTreeMap
            MessageParams::Append => {
                let index = profile_header.last_index as usize;
                if profile_header.notifications[index].sender != sender.key() {
                    return err!(MessageError::SendAsUpdate);
                }

                let index = profile_header.notifications.len();
                if profile_header.notifications[index].sender != sender.key() {
                    return err!(MessageError::SendAsUpdate);
                }
                profile_header.last_index = profile_header.notifications.len() as u64;
                profile_header.notifications.push(MessageNotification {
                    sender: sender.key(),
                    nonce: 0,
                    size: 1,
                })
            }

            MessageParams::Update { index } => {
                profile_header.last_index = index;
                profile_header.notifications[index as usize].size += 1;
            }
        }

        message.sender = sender.key();
        message.recipient = recipient.key();
        message.content = data;

        message_header.nonce += 1;
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
        space = 8 +  1 + 1 + 1 + 8,
        seeds = [b"authority"],
        bump
    )]
    pub new_program_header: Account<'info, ProgramHeader>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeProfile<'info> {
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
        space = 8 + 32 + 8 + 2048,
        seeds = [
            b"profile",
            program_authority.key().as_ref(),
            authority.key().as_ref()
        ],
        bump
    )]
    pub new_profile: Account<'info, Profile>,

    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 8 + (32 + 8 + 8) * 20,
        seeds = [
            b"profile-header",
            new_profile.key().as_ref()
        ],
        bump
    )]
    pub new_profile_header: Account<'info, ProfileHeader>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializePortfolio<'info> {
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
        space = 8 + 32 + 8,
        seeds = [
            b"project-header",
            program_authority.key().as_ref(),
            authority.key().as_ref()
        ],
        bump
    )]
    pub new_project_header: Account<'info, ProjectHeader>,

    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 8 + (8 + 8 + 32) + 20,
        seeds = [
            b"portolio",
            program_authority.key().as_ref(),
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

#[derive(Accounts)]
pub struct AddProject<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"authority"],
        bump = program_authority.bump
    )]
    pub program_authority: Account<'info, ProgramHeader>,

    #[account(
        mut,
        has_one = authority
    )]
    pub project_header: Account<'info, ProjectHeader>,

    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 2048,
        seeds = [
            b"project",
            program_authority.key().as_ref(),
            authority.key().as_ref(),
            project_header.nonce.to_ne_bytes().as_ref()
        ],
        bump
    )]
    pub new_project: Account<'info, Project>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProject<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        has_one = authority,
    )]
    pub project: Account<'info, Project>,
}

#[derive(Accounts)]
pub struct InitializeMessageHeader<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(
        seeds = [b"authority"],
        bump
    )]
    pub program_authority: Account<'info, ProgramHeader>,

    pub recipient: Account<'info, Profile>,

    #[account(
        init,
        payer = sender,
        space = 8 + 32 + 32 + 8 + 1,
        seeds = [
            b"message-header",
            MessageHeader::hash([
                sender.key(),
                recipient.key(),
            ]).as_ref()
        ],
        bump
    )]
    pub new_message_header: Account<'info, MessageHeader>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PostMessage<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    pub recipient: Account<'info, Profile>,

    #[account(
        mut,
        constraint = profile_header.authority.key() ==  recipient.key(),
    )]
    pub profile_header: Account<'info, ProfileHeader>,

    #[account(
        seeds = [
            b"message-header",
            MessageHeader::hash([
                sender.key(),
                recipient.key(),
            ]).as_ref()
        ],
        bump = message_header.bump
    )]
    pub message_header: Account<'info, MessageHeader>,

    #[account(
        init,
        payer = sender,
        space = 8 + 2048,
        seeds = [
            b"message",
            message_header.key().as_ref(),
            message_header.nonce.to_ne_bytes().as_ref()
        ],
        bump
    )]
    pub message: Account<'info, Message>,

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
    pub bump: u8,
    pub nonce: u64,
}

#[account]
pub struct ProfileHeader {
    // updated when new vouch is created
    pub authority: Pubkey,
    pub vouch_nonce: u64,
    pub last_index: u64,
    pub notifications: Vec<MessageNotification>,
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
    //      LOGIC:
    //          create  ->  created when new profile is created
    //          read    ->  portfolio is queriable
    //          update  ->  [PROGRAM] updates nonce when new portfolio is created
    //          delete  ->  delete reference?
}

#[account]
pub struct ProjectHeader {
    // ProjectHeader
    //      SEEDS:
    //          program_authority
    //          "project-header"
    //          user_id
    //      DATA:
    pub authority: Pubkey,
    pub nonce: u64,
    //      LOGIC:
    //          create  ->  [program] created when new profile is created
    //          read    ->  data is queriable
    //          update  ->  [program]
    //          delete  ->  [program]
}

#[account]
pub struct Project {
    // Project
    //      SEEDS:
    //          program_authority
    //          "project"
    //          user_id
    //          nonce
    //      DATA:
    pub authority: Pubkey,
    pub project_url: String,
    pub description: String,
    //       pub tags:           Vec<Tag>,
    //      LOGIC:
    //          create  ->  [authority]
    //          read    ->  data is queriable
    //          update  ->  [authority]
    //          delete  ->  [authority]
}

#[account]
pub struct Content {
    pub field: String,
    pub data: String,
}

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

#[account]
pub struct MessageHeader {
    //  MessageHeader
    //      SEEDS:
    //          program_authority
    //          "message-header"
    //          portfolio.user_id
    //          employeer.user_id
    //      DATA:
    pub members: [Pubkey; 2],
    pub nonce: u64,
    pub bump: u8,
}

impl MessageHeader {
    pub fn hash(members: [Pubkey; 2]) -> solana_program::hash::Hash {
        let hash = &mut solana_program::hash::Hasher::default();
        if members[0] < members[1] {
            hash.hash(members[0].as_ref());
            hash.hash(members[1].as_ref());
        } else {
            hash.hash(members[1].as_ref());
            hash.hash(members[0].as_ref());
        }

        return hash.clone().result();
    }
}

#[account]
pub struct Message {
    // Message
    //      SEEDS:
    //          "chat"
    //          message_header_id
    //          nonce
    //      DATA:
    pub recipient: Pubkey,
    pub sender: Pubkey,
    pub content: String,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub enum MessageParams {
    Append,
    Update { index: u64 },
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

#[error_code]
pub enum MessageError {
    #[msg("Send as Update")]
    SendAsUpdate,
}
