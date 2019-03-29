// Here we add the final features of `#[stored]`, support for underscore
// patterns.
//
// In `#[sorted]` on `match` statements you may not always want to `match` on
// all variants. We'll want to support a feature that, optionally, the last
// pattern can be a wildcard variant. All other arms should still be sorted
// though!
//
//
// Resources:
//
//  - The `Pat` struct definition
//    https://docs.rs/syn/0.15/syn/enum.Pat.html

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
