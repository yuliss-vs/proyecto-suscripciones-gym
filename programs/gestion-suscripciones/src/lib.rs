use anchor_lang::prelude::*;

declare_id!("J9dL6jC5u65vpaSafAG3xUqzPC8HUKhW6k9R2UbnAKDE");

#[program]
pub mod gestion_suscripciones {
    use super::*;

    // 1. CREAR LA PLATAFORMA (El Gimnasio / SaaS)
    pub fn crear_plataforma(
        ctx: Context<NuevaPlataforma>, 
        nombre: String, 
        capacidad_maxima: u16,
        precio_basico: u32,
        precio_pro: u32,
        precio_premium: u32
    ) -> Result<()> {
        let plataforma = &mut ctx.accounts.plataforma;
        plataforma.owner = ctx.accounts.owner.key();
        plataforma.nombre = nombre;
        plataforma.capacidad_maxima = capacidad_maxima;
        plataforma.suscriptores_actuales = 0;
        plataforma.precio_basico = precio_basico;
        plataforma.precio_pro = precio_pro;
        plataforma.precio_premium = precio_premium;
        plataforma.id_counter = 0;
        plataforma.suscriptores = Vec::new();
        Ok(())
    }

    // 2. SUSCRIBIRSE (Con MEJORA 1: Límite de Aforo y MEJORA 2: Selección de Plan)
    pub fn suscribirse(
        ctx: Context<ModificarPlataforma>,
        id_usuario: u16,
        tipo_plan: u8, // 1 = Basico, 2 = Pro, 3 = Premium
    ) -> Result<()> {
        let plataforma = &mut ctx.accounts.plataforma;

        // --- MEJORA 1: Validación de Aforo ---
        require!(
            plataforma.suscriptores_actuales < plataforma.capacidad_maxima,
            Errores::PlataformaLlena
        );

        let monto = match tipo_plan {
            1 => plataforma.precio_basico,
            2 => plataforma.precio_pro,
            3 => plataforma.precio_premium,
            _ => return Err(Errores::PlanInvalido.into()),
        };

        let ahora = Clock::get()?.unix_timestamp;
        let vencimiento = ahora + (30 * 24 * 60 * 60); // 30 días en segundos

        let nueva_suscripcion = Suscripcion {
            tipo_plan,
            monto,
            fecha_inicio: ahora,
            fecha_vencimiento: vencimiento,
            ultimo_checkin: 0, // 0 significa que nunca ha hecho check-in
            racha_dias: 0,
        };

        if id_usuario == 0 {
            // Usuario nuevo
            plataforma.id_counter += 1;
            let id = plataforma.id_counter;
            plataforma.suscriptores_actuales += 1;
            plataforma.suscriptores.push(Suscriptor {
                id,
                historial: vec![nueva_suscripcion],
            });
            msg!("Suscriptor nuevo creado con ID: {}", id);
        } else {
            // Usuario existente (Renovación)
            for i in 0..plataforma.suscriptores.len() {
                if plataforma.suscriptores[i].id == id_usuario {
                    plataforma.suscriptores[i].historial.push(nueva_suscripcion);
                    msg!("Suscripción renovada para el ID: {}", id_usuario);
                    return Ok(());
                }
            }
            return Err(Errores::NoUser.into());
        }
        Ok(())
    }

    // 3. HACER CHECK-IN (Con MEJORA 3: Rachas y validación de tiempo)
    pub fn hacer_checkin(
        ctx: Context<ModificarPlataforma>,
        id_usuario: u16,
    ) -> Result<()> {
        let plataforma = &mut ctx.accounts.plataforma;
        let ahora = Clock::get()?.unix_timestamp;
        let un_dia = 24 * 60 * 60;

        for i in 0..plataforma.suscriptores.len() {
            if plataforma.suscriptores[i].id == id_usuario {
                let historial = &mut plataforma.suscriptores[i].historial;
                let ultima_suscripcion = historial.last_mut().unwrap();

                // Validar que la suscripción esté vigente
                require!(ultima_suscripcion.fecha_vencimiento > ahora, Errores::SuscripcionVencida);

                // --- MEJORA 3: Lógica de Rachas ---
                if ultima_suscripcion.ultimo_checkin == 0 {
                    // Primer check-in
                    ultima_suscripcion.racha_dias = 1;
                } else {
                    let tiempo_desde_ultimo = ahora - ultima_suscripcion.ultimo_checkin;
                    require!(tiempo_desde_ultimo >= un_dia, Errores::YaHicisteCheckinHoy); // Evita doble check-in el mismo día

                    if tiempo_desde_ultimo <= (2 * un_dia) {
                        // Check-in consecutivo (ayer)
                        ultima_suscripcion.racha_dias += 1;
                    } else {
                        // Perdió la racha
                        ultima_suscripcion.racha_dias = 1;
                    }
                }
                ultima_suscripcion.ultimo_checkin = ahora;
                msg!("Check-in exitoso. Racha actual: {} días", ultima_suscripcion.racha_dias);
                return Ok(());
            }
        }
        Err(Errores::NoUser.into())
    }

    // 4. CANCELAR SUSCRIPCIÓN (Cierre Condicionado)
    pub fn cancelar_suscripcion(
        ctx: Context<ModificarPlataforma>,
        id_usuario: u16,
    ) -> Result<()> {
        let plataforma = &mut ctx.accounts.plataforma;
        let ahora = Clock::get()?.unix_timestamp;

        for i in 0..plataforma.suscriptores.len() {
            if plataforma.suscriptores[i].id == id_usuario {
                let ultima_suscripcion = plataforma.suscriptores[i].historial.last().unwrap();
                
                // Validación: No puedes cancelar si aún está activa
                require!(ultima_suscripcion.fecha_vencimiento <= ahora, Errores::SuscripcionActiva);

                plataforma.suscriptores.remove(i); // Elimina al suscriptor
                plataforma.suscriptores_actuales -= 1;
                msg!("Suscripción cancelada y cuenta cerrada.");
                return Ok(());
            }
        }
        Err(Errores::NoUser.into())
    }
}

// --- ESTRUCTURAS DE DATOS ---
#[account]
#[derive(InitSpace)]
pub struct Plataforma {
    pub owner: Pubkey,
    #[max_len(30)]
    pub nombre: String,
    pub capacidad_maxima: u16,
    pub suscriptores_actuales: u16,
    pub precio_basico: u32,
    pub precio_pro: u32,
    pub precio_premium: u32,
    pub id_counter: u16,
    #[max_len(10)]
    pub suscriptores: Vec<Suscriptor>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Suscriptor {
    pub id: u16,
    #[max_len(10)]
    pub historial: Vec<Suscripcion>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Suscripcion {
    pub tipo_plan: u8, // 1: Basico, 2: Pro, 3: Premium
    pub monto: u32,
    pub fecha_inicio: i64,
    pub fecha_vencimiento: i64,
    pub ultimo_checkin: i64,
    pub racha_dias: u16,
}

// --- ERRORES PERSONALIZADOS ---
#[error_code]
pub enum Errores {
    #[msg("No eres el owner de la plataforma")]
    NoOwner,
    #[msg("Usuario no encontrado")]
    NoUser,
    #[msg("La plataforma ha alcanzado su capacidad máxima de suscriptores")]
    PlataformaLlena,
    #[msg("El plan seleccionado no es válido")]
    PlanInvalido,
    #[msg("Ya hiciste check-in hoy, vuelve mañana")]
    YaHicisteCheckinHoy,
    #[msg("Tu suscripción está vencida, debes renovar")]
    SuscripcionVencida,
    #[msg("No puedes cancelar una suscripción que aún está activa")]
    SuscripcionActiva,
}

// --- CONTEXTOS ---
#[derive(Accounts)]
pub struct NuevaPlataforma<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = Plataforma::INIT_SPACE + 8,
        seeds = [b"plataforma", owner.key().as_ref()],
        bump
    )]
    pub plataforma: Account<'info, Plataforma>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModificarPlataforma<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub plataforma: Account<'info, Plataforma>,
}