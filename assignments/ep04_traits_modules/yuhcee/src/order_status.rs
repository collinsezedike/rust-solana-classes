
pub enum OrderStatus {
    Pending,
    Shipped,
    Delivered,
}

pub fn print_status(status: OrderStatus) {
    match status {
        OrderStatus::Pending => println!("Order is Pending"),
        OrderStatus::Shipped => println!("Order is Shipped"),
        OrderStatus::Delivered => println!("Order is Delivered"),
    }
}
