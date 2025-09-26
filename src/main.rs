use crate::app::App;
use models::{Order, Product, TaxAndDiscount};
use std::fs;
use std::io::{Result, Write, stdin, stdout};

mod app;
mod models;
mod ui;

macro_rules! pause {
    () => {
        let mut buffer = String::new();

        stdin().read_line(&mut buffer).expect("Failed to read line");
    };
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "--help" {
        display_help();
        return Ok(());
    }

    let mode = if args.len() > 1 && (args[1] == "--tui" || args[1] == "--cli") {
        &args[1]
    } else {
        "--tui"
    };
    let products = init_avaiable_products();
    let current_order = Order::new(1); // add automatic numeration
    let tax_and_discount = TaxAndDiscount::origin();

    match mode {
        "--tui" => run_tui(products,current_order,tax_and_discount),
        "--cli" => {
            run_cli(products,current_order,tax_and_discount);
            Ok(())
        }
        _ => {
            eprintln!("Invalid mode. Use --tui, --cli, or --help");
            std::process::exit(1);
        }
    }
}

fn display_help() {
    println!("Usage: buymore [OPTIONS]");
    println!();
    println!("Options:");
    println!("  --tui      Run the application in TUI mode (default)");
    println!("  --cli      Run the application in CLI mode");
    println!("  --help     Display this help message and exit");
}

fn run_tui(products: Vec<Product>, current_order: Order, tax_and_discount: TaxAndDiscount) -> Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new(products, current_order, tax_and_discount);

    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}

fn run_cli(products: Vec<Product>, mut current_order: Order, tax_and_discount: TaxAndDiscount) {
    loop {
        let choice = make_a_choice();

        if choice == 1 {
            print_products(products.as_slice());
            pause!();
        } else if choice == 2 {
            choice_products(products.as_slice(), &mut current_order);
        } else if choice == 3 {
            print_cart(&current_order);
            pause!();
        } else if choice == 4 {
            co_print_first_step(&mut current_order, &tax_and_discount);
            pause!();
        } else if choice == 5 {
            break;
        }
    }
    println!("bye bye!");
}

fn make_a_choice() -> u8 {
    let mut choice = String::new();
    let max_choice = 5;
    loop {
        clear_screen();
        print_menu();

        print!("> ");
        stdout().flush().unwrap();

        choice.clear();
        stdin().read_line(&mut choice).unwrap();

        println!("Choice: {}", choice.trim());
        if choice.trim().parse::<u8>().is_ok() {
            let choice_u8: u8 = choice.trim().parse().unwrap();
            if choice_u8 <= max_choice && choice_u8 > 0 {
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

fn print_products(products: &[Product]) {
    for product in products {
        product.print();
    }
}

fn init_avaiable_products() -> Vec<Product> {
    let json = fs::read_to_string("assets/products.json").expect("Failed to read the JSON file.");
    let products: Vec<Product> = serde_json::from_str(&json).expect("Faile to parse JSON file");

    products
}

fn choice_products(products: &[Product], order: &mut Order) {
    for product in products {
        product.print();

        if yes_no(
            "Add this product to the cart? (Y/n)> ",
            "Please answer yes or no...",
        ) {
            order.add_product(product.clone());
        }
    }
}
fn yes_no(message: &str, error_message: &str) -> bool {
    // USE MESSAGE, ADD ERROR MESSAGE
    let mut response = None;
    let mut readline = String::new();

    while response.is_none() {
        print!("{}", message);
        stdout().flush().unwrap();

        readline.clear();
        stdin().read_line(&mut readline).unwrap();
        readline = readline.to_lowercase().trim().to_string();
        if readline.is_empty() || readline == "y" || readline == "yes" {
            response = Some(true);
        } else if readline == "n" || readline == "no" {
            response = Some(false);
        } else {
            print!("{}", error_message);
            stdout().flush().unwrap();
            pause!();
        }
    }
    response.unwrap()
}
fn print_cart(current_order: &Order) {
    println!("+{:-<100}+", "");
    println!("|{:<100}|", "CART");
    println!("+{:-<100}+", "");
    print_products(current_order.products.as_slice());
    println!("+{:-<100}+", "");
}

fn co_print_first_step(current_order: &mut Order, tax_and_discount: &TaxAndDiscount) {
    print_cart(current_order);
    current_order.print_totals(tax_and_discount);
}
