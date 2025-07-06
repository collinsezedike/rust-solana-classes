
pub struct Product {
    pub name: String,
    pub price: f64,
    pub stock: u32,
}

impl Product {
    pub fn is_in_stock(&self) -> bool {
        self.stock > 0
    }
}
