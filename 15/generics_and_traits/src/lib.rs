pub enum Item {
    First,
    Second,
    Third,
}

impl Item {
    pub fn index(&self) -> usize {
        match self {
            Item::First => 0,
            Item::Second => 1,
            Item::Third => 2,
        }
    }
}

pub trait Operations {
    fn get_item(&self, item: Item) -> f64;
    fn set_item(&mut self, item: Item, value: f64);
    fn default_values() -> Self
    where
        Self: Sized;

    fn is_default(&self) -> bool
    where
        Self: Sized,
    {
        self.get_item(Item::First) == 0.0
            && self.get_item(Item::Second) == 0.0
            && self.get_item(Item::Third) == 0.0
    }

    fn sum(&self) -> f64
    where
        Self: Sized,
    {
        self.get_item(Item::First) + self.get_item(Item::Second) + self.get_item(Item::Third)
    }
}

pub struct Tuple(u32, f32, f64);

impl Operations for Tuple {
    fn get_item(&self, item: Item) -> f64 {
        match item {
            Item::First => self.0 as f64,
            Item::Second => self.1 as f64,
            Item::Third => self.2,
        }
    }

    fn set_item(&mut self, item: Item, value: f64) {
        match item {
            Item::First => self.0 = value as u32,
            Item::Second => self.1 = value as f32,
            Item::Third => self.2 = value,
        }
    }

    fn default_values() -> Self {
        Self(0, 0.0, 0.0)
    }
}

pub struct Array([f64; 3]);

impl Operations for Array {
    fn get_item(&self, item: Item) -> f64 {
        self.0[item.index()]
    }

    fn set_item(&mut self, item: Item, value: f64) {
        self.0[item.index()] = value;
    }

    fn default_values() -> Self {
        Self([0.0; 3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple() {
        let mut tuple = Tuple::default_values();

        assert!(tuple.is_default());
        assert_eq!(tuple.sum(), 0.0);

        tuple.set_item(Item::First, 1.0);
        tuple.set_item(Item::Second, 2.0);
        tuple.set_item(Item::Third, 3.0);

        assert_eq!(tuple.get_item(Item::First), 1.0);
        assert_eq!(tuple.get_item(Item::Second), 2.0);
        assert_eq!(tuple.get_item(Item::Third), 3.0);

        assert!(!tuple.is_default());
        assert_eq!(tuple.sum(), 6.0);

        tuple.set_item(Item::First, 0.0);
        tuple.set_item(Item::Second, 0.0);
        tuple.set_item(Item::Third, 0.0);

        assert!(tuple.is_default());
        assert_eq!(tuple.sum(), 0.0);
    }

    #[test]
    fn test_array() {
        let mut array = Array::default_values();

        assert!(array.is_default());
        assert_eq!(array.sum(), 0.0);

        array.set_item(Item::First, 1.0);
        array.set_item(Item::Second, 2.0);
        array.set_item(Item::Third, 3.0);

        assert_eq!(array.get_item(Item::First), 1.0);
        assert_eq!(array.get_item(Item::Second), 2.0);
        assert_eq!(array.get_item(Item::Third), 3.0);

        assert!(!array.is_default());
        assert_eq!(array.sum(), 6.0);

        array.set_item(Item::First, 0.0);
        array.set_item(Item::Second, 0.0);
        array.set_item(Item::Third, 0.0);

        assert!(array.is_default());
        assert_eq!(array.sum(), 0.0);
    }
}
