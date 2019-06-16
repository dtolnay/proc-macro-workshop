#[derive(Copy, Clone)]
pub enum Bounds {
    Exclusive(u64, u64),
    Inclusive(u64, u64),
}

impl IntoIterator for Bounds {
    type Item = u64;
    type IntoIter = Box<dyn Iterator<Item = u64>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Bounds::Exclusive(lo, hi) => Box::new(lo..hi),
            Bounds::Inclusive(lo, hi) => Box::new(lo..=hi),
        }
    }
}
