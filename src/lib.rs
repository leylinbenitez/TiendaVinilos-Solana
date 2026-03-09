use anchor_lang::prelude::*;

// ID del programa en Solana, se actualiza automáticamente al hacer el build/deploy
declare_id!("Gtq38taHxEMzNQKmz5otEjFhwdt9V7PgQWmC8UXbbmtj");


#[program] // Macro que indica que este módulo es un programa de Solana
pub mod tienda_vinilos {
    use super::*; // Importa lo definido fuera del módulo

    //////////////////////////// Instrucción: Crear Tienda /////////////////////////////////////
    /*
    Crea una PDA (Program Derived Address) que representará la tienda de vinilos.
    Esta cuenta almacenará un struct Tienda con el owner, nombre y un vector de vinilos.
    */
    pub fn crear_tienda(context: Context<NuevaTienda>, nombre: String) -> Result<()> {
        let owner_id = context.accounts.owner.key(); // Obtiene la llave pública del dueño
        msg!("Owner id: {}", owner_id); // Mensaje de verificación en el log

        let vinilos: Vec<Vinilo> = Vec::new(); // Crea un vector vacío de vinilos

        // Se guarda un struct Tienda en la cuenta PDA
        context.accounts.tienda.set_inner(Tienda {
            owner: owner_id, // Asigna el dueño
            nombre,          // Asigna el nombre de la tienda
            vinilos,         // Inicializa con vector vacío
        });
        Ok(()) // Transacción exitosa
    }

    //////////////////////////// Instrucción: Agregar Vinilo /////////////////////////////////////
    /*
    Agrega un vinilo al vector de la tienda.
    Solo el owner puede hacerlo.
    */
    pub fn agregar_vinilo(context: Context<NuevoVinilo>, nombre: String, artista: String) -> Result<()> {
        // Validación: solo el dueño puede modificar la tienda
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        // Se crea un nuevo vinilo con los datos recibidos
        let vinilo = Vinilo {
            nombre,
            artista,
            disponible: true, // Por defecto está disponible
        };

        // Se agrega el vinilo al vector dentro de la tienda
        context.accounts.tienda.vinilos.push(vinilo);

        Ok(()) // Transacción exitosa
    }

    //////////////////////////// Instrucción: Eliminar Vinilo /////////////////////////////////////
    /*
    Elimina un vinilo de la tienda a partir de su nombre.
    */
    pub fn eliminar_vinilo(context: Context<NuevoVinilo>, nombre: String) -> Result<()> {
        // Validación: solo el dueño puede eliminar
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let vinilos = &mut context.accounts.tienda.vinilos; // Referencia mutable al vector

        // Se recorre el vector buscando el nombre
        for i in 0..vinilos.len() {
            if vinilos[i].nombre == nombre {
                vinilos.remove(i); // Elimina el vinilo encontrado
                msg!("Vinilo {} eliminado!", nombre);
                return Ok(()); // Éxito
            }
        }
        Err(Errores::ViniloNoExiste.into()) // Error si no se encontró
    }

    //////////////////////////// Instrucción: Ver Vinilos /////////////////////////////////////
    /*
    Muestra en el log todos los vinilos de la tienda.
    */
    pub fn ver_vinilos(context: Context<NuevoVinilo>) -> Result<()> {
        // Validación: solo el dueño puede ver
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        // Muestra en el log todos los vinilos
        msg!("Lista de vinilos: {:#?}", context.accounts.tienda.vinilos);
        Ok(())
    }

    //////////////////////////// Instrucción: Alternar Estado /////////////////////////////////////
    /*
    Cambia el estado de disponibilidad de un vinilo (true → false, false → true).
    */
    pub fn alternar_estado(context: Context<NuevoVinilo>, nombre: String) -> Result<()> {
        // Validación: solo el dueño puede modificar
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let vinilos = &mut context.accounts.tienda.vinilos; // Referencia mutable al vector
        for i in 0..vinilos.len() {
            let estado = vinilos[i].disponible; // Estado actual

            if vinilos[i].nombre == nombre {
                let nuevo_estado = !estado; // Cambia true → false o false → true
                vinilos[i].disponible = nuevo_estado;
                msg!("El vinilo: {} ahora disponible: {}", nombre, nuevo_estado);
                return Ok(()); // Éxito
            }
        }

        Err(Errores::ViniloNoExiste.into()) // Error si no existe
    }
}

// Enum de errores personalizados
#[error_code]
pub enum Errores {
    #[msg("Error, no eres el propietario de la tienda")]
    NoEresElOwner,
    #[msg("Error, el vinilo no existe")]
    ViniloNoExiste,
}

// Cuenta principal: la tienda
#[account] // Indica que este struct se guarda en la blockchain
#[derive(InitSpace)] // Calcula el espacio necesario
pub struct Tienda {
    owner: Pubkey, // Llave pública del dueño

    #[max_len(60)]
    nombre: String, // Nombre de la tienda

    #[max_len(50)]
    vinilos: Vec<Vinilo>, // Vector de vinilos
}

// Struct secundario: cada vinilo
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Vinilo {
    #[max_len(60)]
    nombre: String, // Nombre del vinilo

    #[max_len(60)]
    artista: String, // Nombre del artista

    disponible: bool, // Estado de disponibilidad
}

// Contexto para crear tienda
#[derive(Accounts)]
pub struct NuevaTienda<'info> {
    #[account(mut)]
    pub owner: Signer<'info>, // El dueño paga la transacción

    #[account(
        init, // Se crea la cuenta al llamar la instrucción
        payer = owner, // El dueño paga la creación
        space = Tienda::INIT_SPACE + 8, // Espacio necesario
        seeds = [b"tienda_vinilos", owner.key().as_ref()], // PDA derivada de string + owner
        bump // Método para calcular el bump de la PDA
    )]
    pub tienda: Account<'info, Tienda>, // La cuenta que guarda la tienda

    pub system_program: Program<'info, System>, // Programa necesario para inicializar cuentas
}

// Contexto para agregar/modificar vinilos
#[derive(Accounts)]
pub struct NuevoVinilo<'info> {
    pub owner: Signer<'info>, // El dueño firma la transacción

    #[account(mut)]
    pub tienda: Account<'info, Tienda>, // La tienda se marca mutable porque se modifica tanto el vector como los vinilos que contiene
}

