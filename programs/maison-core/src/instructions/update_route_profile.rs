use anchor_lang::prelude::*;
use crate::constants::ROUTE_PROFILE_SEED;
use crate::errors::MaisonError;
use crate::events::RouteProfileUpdated;
use crate::state::route_profile::{RouteProfile, SplitAllocation};

#[derive(Accounts)]
pub struct UpdateRouteProfile<'info> {
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [ROUTE_PROFILE_SEED, owner.key().as_ref(), route_profile.profile_id.as_ref()],
        bump = route_profile.bump,
        has_one = owner @ MaisonError::Unauthorized
    )]
    pub route_profile: Account<'info, RouteProfile>,
}

pub fn handle_update_route_profile(
    ctx: Context<UpdateRouteProfile>,
    new_splits: Option<Vec<SplitAllocation>>,
    new_daily_limit: Option<u64>,
    new_is_active: Option<bool>,
) -> Result<()> {
    let clock = Clock::get()?;
    let profile = &mut ctx.accounts.route_profile;

    if let Some(splits) = new_splits {
        RouteProfile::validate_splits(&splits)?;
        profile.splits = splits;
    }

    if let Some(limit) = new_daily_limit {
        profile.daily_limit = limit;
    }

    if let Some(active) = new_is_active {
        profile.is_active = active;
    }

    emit!(RouteProfileUpdated {
        profile: profile.key(),
        owner: profile.owner,
        split_count: profile.splits.len() as u8,
        daily_limit: profile.daily_limit,
        is_active: profile.is_active,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}
