pub struct MoneroAmount {
    piconero: u32,
}

impl MoneroAmount {
    #[allow(non_snake_case)]
    pub fn XMR(&self) -> f64 {
        (self.piconero as f64) / (u32::pow(10, 12) as f64)
    }
}
