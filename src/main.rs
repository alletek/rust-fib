use std::io;

fn main() {
    loop {
        println!("Entrez un nombre limite pour la suite : ");
        let mut limite = String::new();

        io::stdin()
            .read_line(&mut limite)
            .expect("Impossible de lire la valeur");

        let limite: u32 = match limite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Affichage de la suite de fibonacci de 0 jusqu'a {}", limite);

        for i in 0..limite + 1 {
            let result_fib: u32 = fibonnaci(i);
            println!("La valeur de la suite de fibonnaci pour {} est {}", i, result_fib)
        }
        break;
    }
}

fn fibonnaci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonnaci(n - 2) + fibonnaci(n - 1)
    }
}
