# Simulador (versión inicial)

Pequeña demo escrita en Rust usando Macroquad. Permite crear "cuerpos" con el clic izquierdo y simula gravedad simple con rebotes en el suelo.

## Requisitos
- Rust (stable)
- Dependencias gestionadas por Cargo (macroquad)

## Ejecutar (Windows)
1. Abrir terminal
2. Ejecutar:
```bash
cargo run
```

## Controles
- Clic izquierdo: crea un cuerpo en la posición del ratón.

## Estructura principal
- `src/main.rs` — bucle principal, entrada y dibujo.
- `src/game/world.rs` — mundo: lista de cuerpos, gravedad y lógica de actualización.
- `src/physics/` — módulo de física (ej. `body.rs`, `integrator.rs`).

## Notas importantes
- Asegúrate de que los campos usados en `Body` (por ejemplo `position`, `size`, `velocity`, `acc`) sean `pub` si los accede `main` o `world`.
- Si `body` es un submódulo privado, exporta el tipo con `pub use body::Body;` en `physics/mod.rs` o marca el módulo `pub mod body;` para poder instanciarlo desde `main`.
- Si no ves nada en pantalla, comprueba que `clear_background()` se llame antes del dibujo y que no haya errores de compilación.

## Próximos pasos
- Añadir colisiones entre cuerpos.
- Mejorar integrador y parámetros físicos.
- Opciones para eliminar cuerpos o ajustar gravedad en tiempo de ejecución.