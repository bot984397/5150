#[derive(Debug)]
pub struct u20 {
    val: u32,
}

impl u20 {
    pub fn new(val:u32) -> Self {
        assert!(val <= 0xFFFF, "Value exceeds 20-bit limit.");
        Self { val }
    }

    pub fn get(&self) -> u32 { self.val }

    pub fn set(&mut self, val:u32) {
        assert!(val <= 0xFFFF, "Value exceeds 20-bit limit.");
        self.val = val;
    }
}

impl std::fmt::Display for u20 {
    fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}
