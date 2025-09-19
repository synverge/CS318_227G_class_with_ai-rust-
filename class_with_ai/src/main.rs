// Trait สำหรับการกำหนด interface
trait Priceable {
    fn get_price(&self) -> f64;
    fn set_price(&mut self, p: f64);
}

// Struct Item โดย price เป็น private (field ไม่ pub)
pub struct Item {
    price: Option<f64>, // ใช้ Option เพื่อความซับซ้อน
}

// Implement constructor และ methods
impl Item {
    // Constructor แบบ public
    pub fn new() -> Self {
        // ใช้ Option และ Some(0.0) เพื่อความซับซ้อน
        Item { price: Some(0.0) }
    }
}

// Implement trait เพื่อแยก logic
impl Priceable for Item {
    // getPrice
    fn get_price(&self) -> f64 {
        // ใช้ match เพื่อ handle None (ซับซ้อน)
        match self.price {
            Some(val) => val,
            None => 0.0, // fallback
        }
    }

    // setPrice
    fn set_price(&mut self, p: f64) {
        // อัปเดท price ผ่าน Option
        self.price = Some(p);
    }
}
