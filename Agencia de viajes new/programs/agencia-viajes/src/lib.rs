use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("BbPZJbdUeGAKmSYMzf622SVomnoFd3aSPJTi9XgXrf9A");

#[program]
pub mod agencia_viajes {
    use super::*;

    // 1. CREAR VIAJE (El autor define el destino y precio)
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
        viaje.reservado = false;
        viaje.cliente = None; // Inicialmente no hay comprador

        Ok(())
    }

    // 2. RESERVAR VIAJE (El comprador paga y se registra)
    pub fn reservar_viaje(ctx: Context<ReservarViaje>) -> Result<()> {
        let viaje = &mut ctx.accounts.viaje;

        // Validar que no esté ya reservado
        require!(!viaje.reservado, ErrorCode::ViajeYaReservado);

        // --- TRANSFERENCIA DE SOL ---
        // El comprador envía 'viaje.precio' al autor del viaje
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.comprador.to_account_info(),
                to: ctx.accounts.autor_viaje.to_account_info(),
            },
        );
        system_program::transfer(cpi_context, viaje.precio)?;

        // --- REGISTRO DEL COMPRADOR ---
        viaje.cliente = Some(ctx.accounts.comprador.key()); // Guardamos la dirección del comprador
        viaje.reservado = true; // Marcamos como vendido

        msg!("Viaje a {} reservado por {}", viaje.destino, ctx.accounts.comprador.key());
        Ok(())
    }

    pub fn actualizar_viaje(ctx: Context<ActualizarViaje>, nuevo_precio: u64) -> Result<()> {
        let viaje = &mut ctx.accounts.viaje;
        require!(!viaje.reservado, ErrorCode::ViajeYaReservado);
        viaje.precio = nuevo_precio;
        Ok(())
    }

    pub fn borrar_viaje(_ctx: Context<BorrarViaje>) -> Result<()> {
        Ok(())
    }
}

// --- CONTEXTOS DE CUENTAS ---

#[derive(Accounts)]
#[instruction(destino: String)]
pub struct CrearViaje<'info> {
    #[account(
        init, 
        payer = autor, 
        space = 8 + 32 + 33 + 54 + 8 + 1 + 1
    )]
    pub viaje: Account<'info, Viaje>,
    #[account(mut)]
    pub autor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReservarViaje<'info> {
    #[account(mut)]
    pub viaje: Account<'info, Viaje>,
    
    #[account(mut)]
    pub comprador: Signer<'info>, // El comprador es quien firma la transacción
    
    /// CHECK: Validamos que esta cuenta sea el autor original para que reciba el dinero
    #[account(mut, address = viaje.autor)]
    pub autor_viaje: AccountInfo<'info>,
    
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

// --- ESTRUCTURA DE DATOS ---

#[account]
pub struct Viaje {
    pub autor: Pubkey,           // Quien creó el viaje (la agencia)
    pub cliente: Option<Pubkey>, // Quien compró el viaje (el comprador)
    pub destino: String,
    pub precio: u64,
    pub duracion: u8,
    pub reservado: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("El destino es demasiado largo.")]
    DestinoMuyLargo,
    #[msg("Este viaje ya ha sido reservado.")]
    ViajeYaReservado,
}