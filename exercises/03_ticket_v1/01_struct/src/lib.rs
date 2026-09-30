// Define a struct named `Order` with the following fields:
// - `price`, an unsigned integer
// - `quantity`, an unsigned integer
//
// It should also have a method named `is_available` that returns a `true` if the quantity is
// greater than 0, otherwise `false`.

/// Order with price of a unit and the quantity of units.
pub struct Order {
    price: u32,
    quantity: u32,
}

impl Order {
    /// Returns `true` if the `quantity` is greater than `0`, otherwise `false`.
    pub fn is_available(self) -> bool {
        self.quantity > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_is_available() {
        let order = Order {
            price: 100,
            quantity: 10,
        };
        assert!(order.is_available());
    }

    #[test]
    fn test_order_is_not_available() {
        let order = Order {
            price: 100,
            quantity: 0,
        };
        assert!(!order.is_available());
    }
}
