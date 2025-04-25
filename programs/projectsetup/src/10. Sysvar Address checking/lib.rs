use anchor_lang::prelude::*;


declare_id!("");

#[program]
pub mod missing_check {
    use super::*;

//^// @info :: Sysvar are read only Solana system accounts which provide chain information like Rent , Clock and Stake History. Mostly they have fixed addreses
//^// @audit-issue :: Here there is no address verification for the rent sysvar so attacker could pass fake rent account with malicious data and also there is no ownership check which confirms that this account is owned by SysvarProgram

    pub fn check_sysvar_address(ctx: Context<CheckSysVarAddress>) ->Result<()> {
        msg!("Rent Address => {}", ctx.accounts.rent.key().to_string());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CheckSysVarAddress<'info> {
    rent: AccountInfo<'info>,
}



//*// One Way to solve is using Sysvar and Rent account check which validates address and owner both
#[derive(Accounts)]
pub struct CheckSysVarAddress<'info> {
    rent: Sysvar<'info, Rent>,
}


//*// Another way is to add a manual check by matching rent account public key with Solana official Rent Sysvar address

pub fn check_sysvar_address(ctx: <CheckSysVarAddress>) -> Result<()> {
    require_eq!(ctx.accounts.rent.key(), sysvar::rent::ID);
    msg!("Rent Key -> {}", ctx.accounts.rent.key().to_string());
    Ok(())
}