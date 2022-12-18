use std::fmt::{Display, Write};

impl EasyWrite for String {}

pub trait EasyWrite: Write {
    fn write_slice_numbers<T: Display>(&mut self, slice: &[T], max_width: usize, indent: usize) -> core::fmt::Result {
        if slice.is_empty() {
            self.write_str("&[],\n")?;
        }
        else {
            self.write_str("&[")?;
            let mut line_width = usize::MAX;
            for byte in slice {
                if line_width > max_width {
                    self.write_new_line()?;
                    self.write_indent(8)?;
                    line_width = 8;
                }
                let char_str = format!("{}, ", byte);
                self.write_str(&char_str)?;
                line_width += char_str.len();
            }
            self.write_str("\n    ],\n")?;
        }
        Ok(())
    }
    fn write_indent(&mut self, spaces: usize) -> std::fmt::Result {
        for _ in 0..spaces {
            self.write_char(' ')?
        }
        Ok(())
    }
    fn write_new_line(&mut self) -> std::fmt::Result {
        self.write_char('\n')
    }
}
