
pub struct SimpleMatcher {
    pattern: String,
    act_pos: usize,
    length: usize,
    found: bool
}

#[allow(dead_code)]
impl SimpleMatcher {
    pub fn new(pattern: &str) -> Self {
        Self { 
            pattern: String::from(pattern), 
            act_pos: 0,
            length: pattern.len(),
            found: false
        } 
    }

    pub fn step(&mut self, input: &char) -> bool {
        if self.found {
            return true
        }

        let c = self.pattern.chars().nth(self.act_pos).unwrap();
        if c == *input {
            self.act_pos += 1;
        } else {
            self.act_pos = 0;
        }
        if self.act_pos >= self.length {
            self.found = true;
            return true
        } else {
            return false          
        }
    }

    pub fn reset(&mut self) {
        self.found = false;
        self.act_pos = 0;
    }
}