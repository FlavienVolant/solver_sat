use std::fmt::Display;

pub struct Literal {
    v: i32
}

impl Literal {
    pub fn new(v: i32) -> Self {
        Self { v }
    }

    pub fn evaluate(&self, boolean: bool) -> bool {
        if self.v < 0{
            ! boolean
        } else {
            boolean
        }
    }

    pub fn get_v(&self) -> i32 {
        self.v.abs()
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", if self.v < 0 {"¬"} else {""}, self.v.abs())?;
        Ok(())
    }
}