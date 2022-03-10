use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Adivinhe um número!\n");

    let numero_secreto = rand::thread_rng().gen_range(1..101);
    println!("O número secreto é: {}", numero_secreto);

    // loop para permitir mais de uma única tentativa
    loop {
        println!("Digite o palpite de um número : ");
        let mut palpite_numero = String::new();

        // lidando com possiveis falhas
        io::stdin()
            .read_line(&mut palpite_numero)
            .expect("Falha ao ler a linha");

        // sombrea o palpite_numero e passa o seu valor string para u32
        // match para lidar com error
        let palpite_numero: u32 = match palpite_numero.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // implementação obsoleta com a utilização de expect e crashing par atipos diferentes
       // let palpite_numero: u32 = palpite_numero.trim().parse().expect("Por favor insira um número!");

        println!("Seu palpite foi: {} ", palpite_numero);

        // utiliza o numero_secreto para selecionar um dos arms
        match palpite_numero.cmp(&numero_secreto) {
            Ordering::Less => println!("muito baixo!"),
            Ordering::Greater => println!("muito alto!"),
            Ordering::Equal => {
                println!("Você Venceu!!!");
                break;
            }
        }
    }
}
