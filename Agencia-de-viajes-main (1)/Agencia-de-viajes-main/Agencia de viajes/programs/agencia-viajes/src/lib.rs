use anchor_lang::prelude::*;

declare_id!("BbPZJbdUeGAKmSYMzf622SVomnoFd3aSPJTi9XgXrf9A");
#[program]
pub mod agencia_viajes {
    use super::*;

    // CREATE
    pub fn crear_viaje(
        ctx: Context<CrearViaje>,
        destino: String,
        precio: u64,
        duracion: u8,
    ) -> Result<()> {

        require!(destino.len() <= 50, ErrorCode::DestinoMuyLargo);

        let viaje = &mut ctx.accounts.viaje;

        viaje.autor = ctx.accounts.autor.key();
        viaje.destino = destino;
        viaje.precio = precio;
        viaje.duracion = duracion;

        Ok(())
    }

    // UPDATE
    pub fn actualizar_viaje(
        ctx: Context<ActualizarViaje>,
        nuevo_precio: u64,
    ) -> Result<()> {

        let viaje = &mut ctx.accounts.viaje;
        viaje.precio = nuevo_precio;

        Ok(())
    }

    // DELETE
    pub fn borrar_viaje(_ctx: Context<BorrarViaje>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(destino: String)]
pub struct CrearViaje<'info> {

    #[account(
        init,
        payer = autor,
        space = 8 + 32 + (4 + 50) + 8 + 1
    )]
    pub viaje: Account<'info, Viaje>,

    #[account(mut)]
    pub autor: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActualizarViaje<'info> {

    #[account(mut, has_one = autor)]
    pub viaje: Account<'info, Viaje>,

    pub autor: Signer<'info>,
}

#[derive(Accounts)]
pub struct BorrarViaje<'info> {

    #[account(mut, has_one = autor, close = autor)]
    pub viaje: Account<'info, Viaje>,

    pub autor: Signer<'info>,
}

#[account]
pub struct Viaje {
    pub autor: Pubkey,
    pub destino: String,
    pub precio: u64,
    pub duracion: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("El destino es demasiado largo (max 50 caracteres)")]
    DestinoMuyLargo,
}
