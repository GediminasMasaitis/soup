use std::fmt::{self, Display};
use std::io::Write;
use std::str::FromStr;

pub struct Spin<T: PartialOrd + Display + FromStr> {
    pub name: &'static str,
    pub min: T,
    pub max: T,
    pub value: T,
}

impl<T: PartialOrd + Display + FromStr> Spin<T> {
    pub fn set(&mut self, value: &str) {
        if let Ok(value) = value.parse::<T>() {
            if value >= self.min && value <= self.max {
                self.value = value;
            }
        }
    }
}

impl<T: PartialOrd + Display + FromStr> fmt::Display for Spin<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "option name {} type spin default {} min {} max {}",
            self.name, self.value, self.min, self.max
        )
    }
}

pub struct Options {
    pub hash: Spin<u64>,
    pub threads: Spin<u64>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            hash: Spin {
                name: "Hash",
                min: 0,
                max: 0,
                value: 0,
            },
            threads: Spin {
                name: "Threads",
                min: 1,
                max: 1,
                value: 1,
            },
        }
    }
}

impl Options {
    pub fn print(&self, writer: &mut impl Write) -> std::io::Result<()> {
        writeln!(writer, "{}", self.hash)?;
        writeln!(writer, "{}", self.threads)
    }

    pub fn set(&mut self, name: &str, value: &str) {
        match name {
            "Hash" => self.hash.set(value),
            "Threads" => self.threads.set(value),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spin_is_displayed() {
        let options = Options::default();
        assert_eq!(
            options.hash.to_string(),
            "option name Hash type spin default 0 min 0 max 0"
        );
    }

    #[test]
    fn set_within_range() {
        let mut spin = Spin {
            name: "Test",
            min: 1,
            max: 100,
            value: 10,
        };
        spin.set("50");
        assert_eq!(spin.value, 50);
    }

    #[test]
    fn set_out_of_range_is_ignored() {
        let mut options = Options::default();
        options.set("Hash", "16");
        options.set("Threads", "2");
        assert_eq!(options.hash.value, 0);
        assert_eq!(options.threads.value, 1);
    }

    #[test]
    fn set_unknown_is_ignored() {
        let mut options = Options::default();
        options.set("Nonsense", "1");
        assert_eq!(options.hash.value, 0);
        assert_eq!(options.threads.value, 1);
    }
}
