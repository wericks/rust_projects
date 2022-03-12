use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Adivinhe um número!\n");

    let numero_secreto = rand::thread_rng().gen_range(1..101);
    // println!("O número secreto é: {}", numero_secreto);
    let soma = 32 + 9;
    let resto = 40 % 6;
    println!("soma {}\nresto divisão: {}", soma, resto);

    let tup : (i32, f64, u8) = (500, 6.4, 2);
        // acesando tupla atravez de destructing
        let (x, y, z) = tup;
        println!("\nvalor Z na tupla : {} ", z);
        println!("\nvalor Y na tupla : {} ", y);
        println!("\nvalor X na tupla : {} \n", x);

    // acessando tupla atravez do elemento e index
    let quinhentos = tup.0;
    let seis_quatro = tup.1;
    let dois = tup.2;
        println!("\nvalor X na tupla : {} \n", quinhentos);
        println!("\nvalor X na tupla : {} \n", seis_quatro);
        println!("\nvalor X na tupla : {} \n", dois);






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
