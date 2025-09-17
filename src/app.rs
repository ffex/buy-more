use crate::models::{Order, Product, TaxAndDiscount};
use crate::ui::render_app;
use crossterm::event::{self, Event as CEvent, KeyCode, KeyEvent};
use ratatui::DefaultTerminal;
use std::io::Error;
use std::time::Duration;

#[derive(Debug)]
pub enum Screen {
    Main,
    Cart,
    Payment,
}

#[derive(Debug)]
pub struct App {
    pub available_products: Vec<Product>,
    pub order: Order,
    pub tax_and_discount: TaxAndDiscount,
    pub exit: bool,
    pub selected_product_index: usize,
    pub current_screen: Screen,
}

impl App {
    pub fn new(
        available_products: Vec<Product>,
        order: Order,
        tax_and_discount: TaxAndDiscount,
    ) -> Self {
        Self {
            available_products,
            order,
            tax_and_discount,
            exit: false,
            selected_product_index: 0,
            current_screen: Screen::Main,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), Error> {
        while !self.exit {
            // Your code here
            if event::poll(Duration::from_millis(16))? {
                if let CEvent::Key(key) = event::read()? {
                    self.handle_key_event(key);
                }
            }
            terminal.draw(|frame| render_app(frame, self));
        }
        Ok(())
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.kind == crossterm::event::KeyEventKind::Press {
            match key_event.code {
                KeyCode::Char('q') => self.exit = true,
                _ => {}
            }
            match self.current_screen {
                Screen::Main => match key_event.code {
                    KeyCode::Up => {
                        if self.selected_product_index > 0 {
                            self.selected_product_index -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if self.selected_product_index < self.available_products.len() - 1 {
                            self.selected_product_index += 1;
                        }
                    }
                    KeyCode::Char('a') => {
                        let product = &mut self.available_products[self.selected_product_index];
                        if !product.in_cart {
                            product.in_cart = true;
                            self.order.add_product(product.clone());
                        }
                    }
                    KeyCode::Char('r') => {
                        let product = &mut self.available_products[self.selected_product_index];
                        product.in_cart = false;
                        self.order.remove_product(&product);
                    }
                    KeyCode::Char('c') => {
                        self.order.calculate_totals(&self.tax_and_discount);
                        self.current_screen = Screen::Cart;
                    }
                    _ => {}
                },
                Screen::Cart => match key_event.code {
                    KeyCode::Char('m') => {
                        self.current_screen = Screen::Main;
                    }
                    KeyCode::Char('p') => {
                        self.current_screen = Screen::Payment;
                    }
                    _ => {}
                },
                Screen::Payment => match key_event.code {
                    KeyCode::Char('y') => {
                        let next_order_id = self.order.number + 1;
                        self.order = Order::new(next_order_id);
                        self.current_screen = Screen::Main;
                    }
                    KeyCode::Char('n') => {
                        self.current_screen = Screen::Cart;
                    }
                    _ => {}
                },
            }
        }
    }
}
