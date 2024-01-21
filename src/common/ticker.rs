#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TickerSymbol([u8; 4]);

impl TickerSymbol {
    pub fn new(input: &str) -> Self {
        let mut bytes = [0; 4];
        input
            .as_bytes()
            .iter()
            .take(4)
            .enumerate()
            .for_each(|(i, byte)| {
                bytes[i] = *byte;
            });
        TickerSymbol(bytes)
    }
}

impl std::fmt::Display for TickerSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            std::str::from_utf8(&self.0)
                .unwrap_or("NULL".into())
                .trim_end_matches("\0")
        )
    }
}
