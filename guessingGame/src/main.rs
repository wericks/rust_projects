use std::io;
use rand::Rng;

fn main() {
    println!("Adivinhe um número!");

    let numero_secreto = rand::thread_rng().gen_range(1..101);
    println!("O número secreto é: {}", numero_secreto);

    println!("Digite o palpite de um número : ");
    let mut palpite_numero = String::new();

    io::stdin()
        .read_line(&mut palpite_numero)
        .expect("Falha ao ler a linha");

    println!("Seu palpite foi: {} ", palpite_numero);

}
