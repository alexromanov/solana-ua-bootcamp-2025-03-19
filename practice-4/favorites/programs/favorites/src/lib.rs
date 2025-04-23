use anchor_lang::prelude::*;

declare_id!("At4g5RmWPSE5w91VwMbZWWdUNC3uQ3RaXYA4yMrVsuk8");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,

    #[max_len(50)]
    pub color: String,
}

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE * Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump,
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"favorites", user.key().as_ref()],
        bump,
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}

#[program]
pub mod favorites {
    use super::*;

    pub fn set_favorites(context: Context<SetFavorites>, number: u64, color: String) -> Result<()> {
        let user_public_key = context.accounts.user.key();
        msg!("Greetings from {}", context.program_id);
        msg!("User {}'s favorite number is {} and favorite color is: {}",
            user_public_key,
            number,
            color
        );

        context.accounts.favorites.set_inner(Favorites {number, color});
        Ok(())
    }

    pub fn update_favorites(
        context: Context<UpdateFavorites>,
        new_number: Option<u64>,
        new_color: Option<String>,
    ) -> Result<()> {
        let favorites = &mut context.accounts.favorites;
        
        // Update number if provided
        if let Some(number) = new_number {
            favorites.number = number;
        }
        
        // Update color if provided
        if let Some(color) = new_color {
            favorites.color = color;
        }
        
        // Log the updates
        msg!(
            "Updated favorites for user {}. New favorite number: {}, New favorite color: {}",
            context.accounts.user.key(),
            favorites.number,
            favorites.color
        );
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
