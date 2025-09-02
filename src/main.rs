mod product;

use product::Product;
use std::io::{Write, stdin, stdout};

macro_rules! pause {
    () => {
        let mut buffer = String::new();
        
//        println!("Press enter to continue...");
        stdin().read_line(&mut buffer).expect("Failed to read line");
    };
}

fn main() {
    let products = init_avaiable_products();
    loop{
        let choice = make_a_choice();

        if choice == 1 {
            print_products(products.as_slice());
        }else if choice == 5{
            break;
        }
        pause!();
    }
    println!("bye bye!");
}

fn make_a_choice() -> u8 {
    let mut choice = String::new();
    loop {
        clear_screen();
        print_menu();

        print!("> ");
        stdout().flush().unwrap();

        choice.clear();
        stdin().read_line(&mut choice).unwrap();

        println!("Choice: {}", choice.trim());
        if choice.trim().parse::<u8>().is_ok(){
            let choice_u8: u8 = choice.trim().parse().unwrap();
            if choice_u8 <= 5 && choice_u8 > 0{ //TODO put 5 in a constant?
                break;
            }
        }
        println!("Wrong choice, press Enter to retry...");
        pause!();
    }
    choice.trim().parse::<u8>().unwrap()
}

fn print_menu() {
    println!("+{:-<100}+", "");
    println!("|{:<100}|", "RustBuy!");
    println!("+{:-<100}+", "");
    println!("|{:-<100}|", "1. Show products");
    println!("|{:-<100}|", "2. Make an order");
    println!("|{:-<100}|", "3. Show cart");
    println!("|{:-<100}|", "4. Checkout");
    println!("|{:-<100}|", "5. Exit");
    println!("+{:-<100}+", "");
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn print_products(products: &[Product]){
    for product in products {
        product.print();
    }
}

fn init_avaiable_products() -> Vec<Product>{
    let products = vec![
        Product::new("Apple I", "The first Apple Computer!", 1499.0),
        Product::new("Apple II", "The second Apple Computer!", 1499.0),
    ];
    products
}
