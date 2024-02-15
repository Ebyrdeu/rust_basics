use std::collections::HashMap;
use std::io;
use std::io::Stdin;

const MENU: &str = "Elpriser
========
1. Inmatning
2. Min, Max och Medel
3. Sortera
4. Bästa Laddningstid (4h)
e. Avsluta
";

struct Prices {
    prices: HashMap<u16, u16>,
}

trait Calculator {
    fn new() -> Self;
    fn input(&mut self, read: &mut Stdin);
}

impl Calculator for Prices {
    fn new() -> Self {
        Self {
            prices: HashMap::new(),
        }
    }
    fn input(&mut self, read: &mut Stdin) {
        let mut answer = String::new();

        for hour in 0..24 {
            println!("{:02}-{:02} o'clock price: ", hour, hour + 1);
            read.read_line(&mut answer)
                .expect("Somthing went wrong with reading a String");

            let price_result = match answer.trim().parse::<u16>() {
                Ok(parsed_number) => parsed_number,
                Err(e) => {
                    panic!("Somthing went wrong while parsing : {}", e)
                }
            };

            self.prices.insert(hour, price_result);
            answer.clear();
        }
    }
}

fn main() {
    let mut prices = Prices::new();

    let mut read = io::stdin();

    menu(&mut read, &mut prices);
}

fn menu(read: &mut Stdin, prices: &mut Prices) {
    let mut continue_menu = true;

    while continue_menu {
        println!("{}", MENU);

        continue_menu = menu_option(read, prices);
    }
}

fn menu_option(read: &mut Stdin, prices: &mut Prices) -> bool {
    let mut answer = String::new();
    read.read_line(&mut answer)
        .expect("Somthing went wrong with reading a String");

    match answer.trim() {
        "1" => prices.input(read),
        "2" => println!("average"),
        "3" => println!("sorting"),
        "4" => println!("best_four_hours"),
        "e" | "E" => {
            println!("Exiting");
            return false;
        }
        _ => {
            println!("Wrong key!")
        }
    }

    return true;
}
