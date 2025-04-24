use anchor_lang::prelude::*;


declare_id!("");

#[program]
pub mod bump_seed {
    use super:;*;

    //@audit-issue :: AS there is no canonical bump used there will be a high chances of generating different addresses for the same seeds which breaks PDA usage of uniqueness
    // Attacker could pass a non-canonical bump and the program will allow duplicate accounts
    pub fn set_value(ctx: Context<BumpSeed>, key: u64, new_value: u64, bump: u8) -> Result<()> {
        let address = Pubkey::create_program_address(&[key.to_le_bytes().as_ref(), &[bump]], ctx.program_id)?;
        if address != ctx.accounts.data.key() {
            return Err(ProgramError::InvalidArgument);
        }

        ctx.accounts.data.value = new_value;
        Ok(())
    }


}

#[derive(Accounts)]
pub struct BumpSeed<'info> {
    data:Account<'info, Data>,
}

#[account]
pub struct Data {
    value: u64,
}

//*// 1. One way to fix is adding seeds and bump directly in the Accounts
#[derive(Accounts)]
#[instruction(key: u64)]
pub struct BumpSeed<'info> {
    #[account(seeds = [key.to_le_bytes.as_ref()], bump)]
    data: Account<'info,Data>
}

//*// 2. Another way of doing it is using find_program_address and validates the provided account matches with PDA

pub fn set_value(ctx: Context<BumpSeed>, key: u64, new_value:u64, bump:u8,) -> Result<()> {
    let (address, expected_bump) = Pubkey::find_program_address(&[key.to_le_bytes().as_ref()], ctx.program_id);

    if address != ctx.accounts.data.key() {
        return Err(ProgramError::InvalidArgument);
    }
    if expected_bump != bump {
        return Err(ProgramError::InvalidArgument);
    }

    ctx.accounts.data.value = new_value;
    Ok(())
}