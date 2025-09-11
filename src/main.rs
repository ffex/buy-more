mod models;

use models::{Product, Order, TaxAndDiscount};
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
    let mut current_order = Order::new(1); // add automatic numeration
    let tax_and_discount = TaxAndDiscount::origin();
    loop{
        let choice = make_a_choice();

        if choice == 1 {
            print_products(products.as_slice()); 
            pause!();
        }else if choice == 2 {
            choice_products(products.as_slice(), &mut current_order);

        }else if choice == 3 {
            print_cart(&current_order);
            pause!();
        }else if choice == 4 {
            co_print_first_step(&mut current_order, &tax_and_discount);
            pause!();
        }else if choice == 5{
            break;
        }
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

fn choice_products(products: &[Product], order: &mut Order){
    for product in products {
        product.print();

        if yes_no("Add this product to the cart? (Y/n)> ", "Please answer yes or no..."){
            order.add_product(product.clone());
        }
    }
}
fn yes_no(message: &str, error_message: &str)->bool{// USE MESSAGE, ADD ERROR MESSAGE
    let mut response = None;
    let mut readline= String::new();

    while response.is_none(){ 
        print!("{}", message);
        stdout().flush().unwrap();

        readline.clear();
        stdin().read_line(&mut readline).unwrap();
        readline = readline.to_lowercase().trim().to_string();
        if readline.is_empty() || readline == "y" || readline == "yes" {
            response = Some(true);
        }else if readline == "n" || readline == "no"{
            response = Some(false);
        }else{
            print!("{}", error_message);
            stdout().flush().unwrap();
            pause!();
        }
    }
    response.unwrap()
}
fn print_cart(current_order: &Order){
    println!("+{:-<100}+", "");
    println!("|{:<100}|","CART");
    println!("+{:-<100}+", "");
    print_products(current_order.products.as_slice());
    println!("+{:-<100}+", "");
}

fn co_print_first_step(current_order: &mut Order, tax_and_discount: &TaxAndDiscount){
    print_cart(current_order);
    current_order.print_totals(tax_and_discount);
}

