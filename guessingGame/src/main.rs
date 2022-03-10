use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Adivinhe um número!\n");

    let numero_secreto = rand::thread_rng().gen_range(1..101);
    println!("O número secreto é: {}", numero_secreto);

    println!("Digite o palpite de um número : ");
    let mut palpite_numero = String::new();

    io::stdin()
        .read_line(&mut palpite_numero)
        .expect("Falha ao ler a linha");

    let palpite_numero: u32 = palpite_numero.trim().parse().expect("Por favor insira um número!");

    println!("Seu palpite foi: {} ", palpite_numero);

    match palpite_numero.cmp(&numero_secreto) {
        Ordering::Less => println!("muito baixo!"),
        Ordering::Greater => println!("muito alto!"),
        Ordering::Equal => println!("Você Venceu!!!"),
    }

}
