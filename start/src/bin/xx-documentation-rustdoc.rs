/// Esta função soma dois números inteiros e retorna o resultado.
///
/// # Argumentos
///
/// * `a`: O primeiro número inteiro.
/// * `b`: O segundo número inteiro.
///
/// # Retorna
///
/// O resultado da soma.
fn add(a: i32, b: i32) -> i32 {
    a + b
}
/// Esta função principal é a entrada do programa.
///
/// # Exemplos
///
/// ```
/// // Iniciar o programa com: cargo run
/// fn main() {
///     let x = 42;
///     let y = 17;
///     let result = add(x, y);
///     println!("Resultado da soma: {}", result);
/// }
/// ```
fn main() {
    let x = 42;
    let y = 17;
    let result = add(x, y);
    println!("Resultado da soma: {}", result);
}
