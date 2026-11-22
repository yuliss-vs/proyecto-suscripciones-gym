# Gestor de Suscripciones para Gimnasio 

**Alumno:** Yuliss
**Program ID:** J9dL6jC5u65vpaSafAG3xUqzPC8HUKhW6k9R2UbnAKDE

## Descripción del Proyecto
Este proyecto es una aplicación descentralizada (dApp) construida en Solana (Devnet) para gestionar las suscripciones de un gimnasio o plataforma SaaS. Permite a un administrador crear la plataforma, definir precios y capacidad, y a los usuarios suscribirse y registrar su asistencia.

## Aporte Propio (Las 3 Mejoras)
Para este proyecto, se implementaron 3 mejoras funcionales sobre el modelo básico visto en clase:

1. **Límite de Aforo (Capacidad Máxima):** El gimnasio no permite más suscriptores de los que puede albergar. El programa rechaza la transacción si se supera el límite.
2. **Múltiples Planes:** Los usuarios pueden elegir entre 3 planes (Básico, Pro, Premium) con diferentes precios definidos por el administrador.
3. **Check-in con Rachas:** Los usuarios pueden registrar su asistencia diaria. El programa usa el reloj de Solana (`Clock::get()`) para validar que no hagan check-in dos veces el mismo día y calcular su racha de días consecutivos.

## Pruebas y Evidencias
A continuación se detallan las pruebas realizadas en Devnet. Todas las transacciones exitosas cuentan con su enlace verificable en Solana Explorer.

| Caso de Prueba | Operación | Resultado Esperado | Evidencia |
| :--- | :--- | :--- | :--- |

| Crear Plataforma | `crearPlataforma` | La cuenta se crea con capacidad 3. | <img width="662" height="675" alt="image" src="https://github.com/user-attachments/assets/72c980f9-343a-4257-8c73-3e0467c92519" />|

| Suscripción Exitosa | `suscribirse(0,1)` | Usuario creado, contador sube. | <img width="657" height="676" alt="image" src="https://github.com/user-attachments/assets/a83e3a42-6e47-452f-b63e-4a5a40672727" />|

| **Límite de Aforo** | `suscribirse(0,1)` | Error: `PlataformaLlena` al intentar el 4to. | <img width="657" height="675" alt="image" src="https://github.com/user-attachments/assets/cff82537-29ac-400c-a6eb-51d6f834ef4e" />|

| **Check-in Exitoso** | `hacerCheckin(1)` | Racha sube a 1. | <img width="660" height="665" alt="image" src="https://github.com/user-attachments/assets/97ddba90-dd51-49f0-a889-1cfef5b8d80c" />|
 
| **Check-in Duplicado** | `hacerCheckin(1)` | Error: `YaHicisteCheckinHoy`. | <img width="662" height="676" alt="image" src="https://github.com/user-attachments/assets/23f387a2-f602-41bb-b184-3207a222018b" />|

| **Cierre Condicionado** | `cancelarSuscripcion(1)` | Error: `SuscripcionActiva` (aún no vence). | <img width="655" height="672" alt="image" src="https://github.com/user-attachments/assets/6448a975-64d5-4ddc-a7bf-c4363e730ebd" />|

| Lectura de Datos | `verPlataforma()` | Se leen los 3 suscriptores correctamente. | <img width="657" height="696" alt="image" src="https://github.com/user-attachments/assets/4cb6488c-5934-45b6-a2c3-b1114f80fc32" />|

## Instrucciones de Instalación y Ejecución
1. Clonar el repositorio.
2. Abrir el proyecto en Solana Playground (importando desde GitHub).
3. Hacer Build y Deploy en Devnet.
4. Ejecutar el archivo `client.ts` para probar las funciones.
