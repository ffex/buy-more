# RustBuy

RustBuy is a simple command-line interface (CLI) e-commerce application built in Rust, designed specifically to help you understand and practice working with terminal user interfaces (TUI) in Rust.

## Overview

RustBuy allows users to browse available products, add them to an order, view the cart, and complete checkout. The application calculates taxes and discounts based on predefined thresholds.

### Key Features
- **Product Management**: Display all available products.
- **Order Management**: Add products to the cart and calculate the total.
- **Tax and Discount Calculation**: Automatically apply taxes and discounts based on purchase amount.

## Getting Started

### Prerequisites

- Rust programming language (>= 1.56.0)
- Cargo package manager

### Installation

```sh
git clone https://github.com/federicobassini/rust-buy-more.git
cd rust-buy-more
cargo build --release
```

### Running the Application

After building, run the application with:

```sh
cargo run --release
```

## Usage

The application presents a simple menu-driven interface. Users can navigate through various options using numbers (1 to 5) corresponding to each action.

### Menu Options
- **Show products**: Displays all available products.
- **Make an order**: Add selected products to the cart and start checkout.
- **Show cart**: View current items in the cart and their total.
- **Checkout**: Complete the purchase, including tax and discount calculations.
- **Exit**: Quit the application.

## Contributing

The project had the goal of learning Rust and practicing TUI development. Contributions are welcome!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
