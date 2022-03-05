use std::io;

fn main() {
    println!("Adivinhe um número! ");

    println!("Digite um número: ");
    let mut chute= String::new();

    io::stdin()
        .read_line(&mut chute )
        .expect("Falha ao ler a linha");

    println("Seu chute foi: {} ", chute);

}
