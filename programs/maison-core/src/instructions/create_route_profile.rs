use anchor_lang::prelude::*;
use crate::constants::ROUTE_PROFILE_SEED;
use crate::events::RouteProfileCreated;
use crate::state::route_profile::{RouteProfile, SplitAllocation};

#[derive(Accounts)]
#[instruction(profile_id: [u8; 32])]
pub struct CreateRouteProfile<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = RouteProfile::BASE_LEN,
        seeds = [ROUTE_PROFILE_SEED, owner.key().as_ref(), profile_id.as_ref()],
        bump
    )]
    pub route_profile: Account<'info, RouteProfile>,

    pub system_program: Program<'info, System>,
}

pub fn handle_create_route_profile(
    ctx: Context<CreateRouteProfile>,
    profile_id: [u8; 32],
    splits: Vec<SplitAllocation>,
    daily_limit: u64,
    compliance_tier: u8,
) -> Result<()> {
    RouteProfile::validate_splits(&splits)?;

    let clock = Clock::get()?;
    let profile = &mut ctx.accounts.route_profile;

    profile.owner = ctx.accounts.owner.key();
    profile.profile_id = profile_id;
    profile.bump = ctx.bumps.route_profile;
    profile.is_active = true;
    profile.compliance_tier = compliance_tier;
    profile.daily_limit = daily_limit;
    profile.daily_spent = 0;
    profile.last_reset_timestamp = clock.unix_timestamp;
    profile.splits = splits.clone();
    profile._reserved = [0u8; 32];

    emit!(RouteProfileCreated {
        profile: profile.key(),
        owner: profile.owner,
        profile_id,
        split_count: splits.len() as u8,
        daily_limit,
        compliance_tier,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}
