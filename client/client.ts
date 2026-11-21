import * as anchor from "@coral-xyz/anchor";
import * as web3 from "@solana/web3.js";
import { PublicKey } from "@solana/web3.js";
import type { GestionSuscripciones } from "../target/types/gestion_suscripciones";

// Configure the client to use the local cluster
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.GestionSuscripciones as anchor.Program<GestionSuscripciones>;


// Configuración inicial
const plataformaSeed = "plataforma";
const owner = program.provider.publicKey;

// Función para obtener la PDA de la plataforma
function getPlataformaPDA() {
  return PublicKey.findProgramAddressSync(
    [Buffer.from(plataformaSeed), owner.toBuffer()],
    program.programId
  );
}

// --- FUNCIONES PARA PROBAR ---

// Crear la plataforma (Gimnasio)
async function crearPlataforma(
  nombre,
  capacidad,
  precioBasico,
  precioPro,
  precioPremium
) {
  const [pda] = getPlataformaPDA();
  const tx = await program.methods
    .crearPlataforma(nombre, capacidad, precioBasico, precioPro, precioPremium)
    .accounts({ owner: owner, plataforma: pda })
    .rpc();
  console.log("✅ Plataforma creada. Tx:", tx);
}

// Suscribirse (Nuevo usuario)
async function suscribirse(idUsuario, tipoPlan) {
  const [pda] = getPlataformaPDA();
  const tx = await program.methods
    .suscribirse(idUsuario, tipoPlan)
    .accounts({ owner: owner, plataforma: pda })
    .rpc();
  console.log("✅ Suscripción exitosa. Tx:", tx);
}

// Hacer Check-in (Mejora 3)
async function hacerCheckin(idUsuario) {
  const [pda] = getPlataformaPDA();
  const tx = await program.methods
    .hacerCheckin(idUsuario)
    .accounts({ owner: owner, plataforma: pda })
    .rpc();
  console.log("✅ Check-in exitoso. Tx:", tx);
}

// Cancelar Suscripción
async function cancelarSuscripcion(idUsuario) {
  const [pda] = getPlataformaPDA();
  const tx = await program.methods
    .cancelarSuscripcion(idUsuario)
    .accounts({ owner: owner, plataforma: pda })
    .rpc();
  console.log("✅ Suscripción cancelada. Tx:", tx);
}

// Leer el estado actual de la plataforma
async function verPlataforma() {
  const [pda] = getPlataformaPDA();
  const cuenta = await program.account.plataforma.fetch(pda);
  console.log("Datos de la plataforma:", cuenta);
}

//crearPlataforma("Gym WayLearn", 3, 10, 20, 30);
//suscribirse(0, 1); // El 0 significa usuario nuevo, el 1 es el plan Básico
//suscribirse(0, 2); // el 2 es el plan Pro
//suscribirse(0, 3); // el 3 es el plan Premium
//suscribirse(0, 1);
//hacerCheckin(1);
//hacerCheckin(1);
//cancelarSuscripcion(1);
verPlataforma();
