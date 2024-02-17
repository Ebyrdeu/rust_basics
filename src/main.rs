use std::collections::BTreeMap;
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
    prices: BTreeMap<u16, u16>,
}

trait Calculator {
    fn new() -> Self;
    fn input(&mut self, read: &mut Stdin);
    fn min_max_average(&mut self);
    fn sort(&mut self);
    fn cheapest_4_hours(&mut self);
}

impl Calculator for Prices {
    fn new() -> Self {
        Self {
            prices: BTreeMap::new(),
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

    fn min_max_average(&mut self) {
        // min value
        match self.prices.iter().min_by_key(|entry| entry.1) {
            None => {
                println!("Could not find a value please insert values first");
            }
            Some(value) => {
                println!("{:?}", value);
            }
        }
        // max value
        match self.prices.iter().max_by_key(|entry| entry.1) {
            None => {
                println!("Could not find a value please insert values first");
            }
            Some(value) => {
                println!("{:?}", value);
            }
        }

        let average = self.prices.iter().map(|entry| entry.1).sum::<u16>() / 24;

        println!("{:?}", average as f32);
    }

    fn sort(&mut self) {
        let first_four: Vec<_> = self.prices.iter().take(4).collect();

        for (hour, price) in first_four {
            println!("{:02}-{:02} {} öre", hour, hour + 1, price);
        }
    }

    fn cheapest_4_hours(&mut self) {
        let mut min_total_price = u16::MAX;
        let mut best_hours: Vec<String> = Vec::new();

        for i in 0..=(self.prices.len() - 4) {
            let mut total = 0;
            let mut consecutive_hours: Vec<String> = Vec::new();

            for (hour, price) in self.prices.iter().skip(i).take(4) {
                total += *price;
                consecutive_hours.push(hour.to_string());
            }

            if total < min_total_price {
                min_total_price = total;
                best_hours = consecutive_hours;
            }
        }
        for i in 0..best_hours.len() {
            println!("{:?}", self.prices.get(&(i as u16)));
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
        "2" => prices.min_max_average(),
        "3" => prices.sort(),
        "4" => prices.cheapest_4_hours(),
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
