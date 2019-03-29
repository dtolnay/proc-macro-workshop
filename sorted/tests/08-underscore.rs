use sorted::sorted;

#[sorted]
pub enum Conference {
    RustBeltRust,
    RustConf,
    RustFest,
    RustLatam,
    RustRush,
}

impl Conference {
    #[sorted::check]
    pub fn region(&self) -> &str {
        use self::Conference::*;

        #[sorted]
        match self {
            RustFest => "Europe",
            RustLatam => "Latin America",
            _ => "elsewhere",
        }
    }
}

fn main() {}
