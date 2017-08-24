//! Defines the VMF file abstract syntax tree

use std::fmt::{self, Write, Display, Formatter, Result};

/// Utility struct for pretty-printing blocks
/// from https://github.com/rust-lang/rust/blob/master/src/libcore/fmt/builders.rs
struct PadAdapter<'a> {
    fmt: &'a mut Write,
    on_newline: bool,
}

impl<'a> PadAdapter<'a> {
    fn new(fmt: &'a mut Write) -> PadAdapter<'a> {
        PadAdapter {
            fmt: fmt,
            on_newline: false,
        }
    }
}

impl<'a> Write for PadAdapter<'a> {
    fn write_str(&mut self, mut s: &str) -> Result {
        while !s.is_empty() {
            if self.on_newline {
                self.fmt.write_str("\t")?;
            }

            let split = match s.find('\n') {
                Some(pos) => {
                    self.on_newline = true;
                    pos + 1
                }
                None => {
                    self.on_newline = false;
                    s.len()
                }
            };
            self.fmt.write_str(&s[..split])?;
            s = &s[split..];
        }

        Ok(())
    }
}

/// Simple key-value pair
#[derive(Clone, Debug, Default, Hash)]
pub struct Property<K> {
    pub key: K,
    pub value: String,
}

impl<K: Display> Display for Property<K> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        writeln!(fmt, "\"{}\" \"{}\"", self.key, self.value)
    }
}

/// A named construct containing properties and other blocks
#[derive(Clone, Debug, Default, Hash)]
pub struct Block<K> {
    pub name: K,
    pub props: Vec<Property<K>>,
    pub blocks: Vec<Block<K>>,
}

impl<K: Display> Display for Block<K> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        writeln!(fmt, "{}", self.name)?;

        {
            let mut writer = PadAdapter::new(fmt);
            writeln!(writer, "{{")?;

            for prop in &self.props {
                write!(&mut writer, "{}", prop)?;
            }
            for prop in &self.blocks {
                write!(&mut writer, "{}", prop)?;
            }
        }

        writeln!(fmt, "}}")
    }
}
