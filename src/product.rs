pub struct Product{
    pub name: String,
    pub description: String,
    pub price: f32
}

impl Product{
    pub fn origin() -> Product {
       Product { name: String::new(), description: String::new(), price: 0.0 } 
    }
    pub fn new( name: &str, description: &str, price: f32) -> Product {
        Product { name : name.to_string(), description: description.to_string(), price: price }
    }
    pub fn print(&self){
        println!("Prodotto: {:<10} | {:<50} | {} $", self.name, self.description, self.price);
    }
}
