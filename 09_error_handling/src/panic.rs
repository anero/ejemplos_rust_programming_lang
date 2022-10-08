fn main() {
  panic!("crash and burn!"); // Error irrecuperable. Rust se encarga de limpiar la memoria que el programa había alocado
                             // Usando la variable de ambiente RUST_BACKTRACE=1, se imprime en pantalla el backtrace completo
}
