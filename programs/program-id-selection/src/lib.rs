use anchor_lang::prelude::*;

#[cfg(not(feature = "localnet"))]
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[cfg(feature = "localnet")]
include!(concat!(env!("OUT_DIR"), "/program_id_local.rs"));

#[program]
pub mod program_id_selection {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
