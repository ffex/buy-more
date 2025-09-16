#[derive(Clone, Debug)]
pub struct Product {
    pub name: String,
    pub description: String,
    pub price: f32,
    pub in_cart: bool,
}

#[allow(dead_code)]
impl Product {
    pub fn origin() -> Product {
        Product {
            name: String::new(),
            description: String::new(),
            price: 0.0,
            in_cart: false,
        }
    }
    pub fn new(name: &str, description: &str, price: f32) -> Product {
        Product {
            name: name.to_string(),
            description: description.to_string(),
            price,
            in_cart: false,
        }
    }
    pub fn print(&self) {
        println!(
            "Prodotto: {:<10} | {:<50} | {} $",
            self.name, self.description, self.price
        );
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Order {
    pub number: u32,
    pub products: Vec<Product>,
    pub taxable: f32,
    pub iva_calculated: f32,
    pub discount_perc: i32,
    pub discount_calculated: f32,
    pub delivery_cost: f32,
    pub totals: f32,
    pub done: bool,
}

impl Order {
    pub fn new(number: u32) -> Order {
        Order {
            number,
            products: Vec::new(),
            done: false,
            taxable: 0.0,
            iva_calculated: 0.0,
            discount_perc: 0,
            discount_calculated: 0.0,
            delivery_cost: 0.0,
            totals: 0.0,
        }
    }
    pub fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }
    pub fn print_totals(&mut self, tax_and_discount: &TaxAndDiscount) {
        self.calculate_totals(tax_and_discount);
        println!("Taxable: {:.2} $", self.taxable);
        println!("IVA: {:.2} $", self.iva_calculated);
        println!(
            "Discount: {:.2} $ ({}%)",
            self.discount_calculated, self.discount_perc
        );
        println!("Delivery cost: {:.2} $", self.delivery_cost);
        println!("Total: {:.2} $", self.totals);
    }

    pub fn calculate_totals(&mut self, tax_and_discount: &TaxAndDiscount) {
        self.taxable = self
            .products
            .iter()
            .map(|product| product.price)
            .sum::<f32>();

        //calculate iva
        self.iva_calculated = self.taxable * tax_and_discount.iva as f32 / 100.00;
        let mut discount: i32 = 0;
        //discount
        for (threshold, perc) in &tax_and_discount.discount {
            if self.taxable >= *threshold {
                discount = *perc;
            }
        }
        self.discount_perc = discount;
        self.discount_calculated = discount as f32 * self.taxable / 100.0;

        if self.taxable > tax_and_discount.free_delivery_min {
            self.delivery_cost = 0.0;
        } else {
            self.delivery_cost = tax_and_discount.delivery_cost;
        }
        self.totals =
            self.taxable + self.iva_calculated + self.delivery_cost - self.discount_calculated;
    }

    pub fn remove_product(&mut self, product: &Product) {
        if let Some(index) = self.products.iter().position(|p| p.name == product.name) {
            self.products.remove(index);
        }
    }
}

#[derive(Debug)]
pub struct TaxAndDiscount {
    pub iva: i8,
    pub discount: Vec<(f32, i32)>,
    pub free_delivery_min: f32,
    pub delivery_cost: f32,
}

impl TaxAndDiscount {
    pub fn origin() -> TaxAndDiscount {
        TaxAndDiscount {
            iva: 22,
            discount: vec![(1000.00, 5), (1500.00, 10), (2000.00, 20)],
            free_delivery_min: 100.00,
            delivery_cost: 45.00,
        }
    }
}
