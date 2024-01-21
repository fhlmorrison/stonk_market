const FRACTIONAL_MAX_LENGTH: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Price {
    integral: u64,
    fractional: u64,
}

impl TryFrom<&str> for Price {
    type Error = &'static str;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut split = input.split('.');
        let integral = split
            .next()
            .ok_or("Failed to parse integral")?
            .parse::<u64>()
            .map_err(|_| "Failed to parse integral")?;

        let fractional = split.next().unwrap_or("0");
        let fractional_len = fractional.len();
        if fractional_len > FRACTIONAL_MAX_LENGTH {
            return Err("Fractional too large");
        }
        let fractional = fractional
            .parse::<u64>()
            .map_err(|_| "Failed to parse fractional")?
            * 10u64.pow((FRACTIONAL_MAX_LENGTH - fractional_len) as u32);

        Ok(Price {
            integral,
            fractional,
        })
    }
}

impl std::fmt::Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{:0width$}",
            self.integral,
            self.fractional,
            width = FRACTIONAL_MAX_LENGTH
        )
    }
}
