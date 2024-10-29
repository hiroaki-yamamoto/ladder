use ::std::string::ToString;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Ladder {
  pub price: f64,
  pub qty: u64,
}

impl Ladder {
  pub fn new(price: f64, qty: u64) -> Self {
    Self { price, qty }
  }
}

impl ToString for Ladder {
  fn to_string(&self) -> String {
    format!("Price: {}, Qty: {}", self.price, self.qty)
  }
}
