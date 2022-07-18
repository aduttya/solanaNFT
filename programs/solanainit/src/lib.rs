use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token;
use anchor_spl::token::{Token,InitializeMint,MintTo,Transfer};
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v2};

declare_id!("9RPKdTnvGJb53Dn87mASUw2fRD6xM9XGr5faL66HR4jg");

#[program]
pub mod solanainit{
    use super::*;


    pub fn mint_to(
        ctx:Context<MintToken>,
        creator_key : Pubkey,
        url : String,
        title : String
    ) -> Result<()>{

        let cpi_accounts = MintTo{
            mint : ctx.accounts.mint.to_account_info(),
            to : ctx.accounts.token_account.to_account_info(),
            authority : ctx.accounts.payer.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();

        let cpi_ctx = CpiContext::new(cpi_program,cpi_accounts);

        token::mint_to(cpi_ctx,1)?;

        msg!("Token minted !!");
        let account_info = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];

        msg!("Account info Assigned");

        let creator = vec![
            mpl_token_metadata::state::Creator{
                address:creator_key,
                verified : false,
                share : 100
            },
            mpl_token_metadata::state::Creator{
                address : creator_key,
                verified : false,
                share : 0
            },
        ];

        msg!("creator assigned");

        let symbol = std::string::ToString::to_string("testing");
        invoke(&create_metadata_accounts_v2(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.mint_authority.key(),
            ctx.accounts.payer.key(),
            ctx.accounts.payer.key(),
            title,
            symbol,
            url,
            Some(creator),
            1,
            true,
            false,
            None,
            None
        ),
            account_info.as_slice(),
    )?;

    msg!("metadata account created");
    let master_edition_infos = vec![
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        msg!("Master Edition Account Infos Assigned");

        invoke(
            &create_master_edition_v3(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.master_edition.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.payer.key(),
                Some(0),
            ),
            master_edition_infos.as_slice(),
        )?;
        msg!("Master Edition Nft Minted !!!");

        Ok(())

    }

    pub fn transfer(ctx:Context<TransferToken>, _amount: u64) -> Result<()> {

        let transfer_instruction = Transfer{
            from : ctx.accounts.from.to_account_info(),
            to : ctx.accounts.to.to_account_info(),
            authority : ctx.accounts.signer.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();

        let cpi_ctx = CpiContext::new(cpi_program,transfer_instruction);

        anchor_spl::token::transfer(cpi_ctx, _amount)?;
        Ok(())
    }


}


#[derive(Accounts)]
pub struct MintToken<'info>{

    #[account(mut)]
    pub mint_authority : Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint : UncheckedAccount<'info>,
    pub token_program : Program<'info,Token>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account : UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub payer : AccountInfo<'info>,
    pub system_program: Program<'info, System>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata : UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_metadata_program : UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub rent : UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub master_edition : UncheckedAccount<'info>

    
}

#[derive(Accounts)]
pub struct TransferToken<'info>{

    pub token_program : Program<'info,Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub from : UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub to : AccountInfo<'info>,

    #[account(mut)]
    pub signer : Signer<'info>,
}

#[account]
pub struct Count {
    pub count: u64,
    pub authority: Pubkey
}

// #[derive(Accounts)]
// pub struct Init <'info>{
//     #[account(init,
//         seeds = [b"count"],
//         bump,
//         payer = user,
//         space = 8 + 8,)]
//      pub data : Account<'info,Count>,
//      #[account(mut)]
//      pub user : Signer<'info>,
//      pub system_program : Program<'info,System>
// }

// #[derive(Accounts)]
// pub struct Inc<'info>{
//     #[account(mut)]
//     pub data : Account<'info,Count>

// }


// #[account]
// pub struct Count{
//     pub data : u64
// }