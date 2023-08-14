use crate::error::{XError, XResult};

pub struct Coords {
    pub row: u32,
    pub col: u16,
}

impl Coords {
    pub fn from_a1(coords: &str) -> XResult<Self> {
        let coords = coords.replace('$', "");
        let mut chars = coords.chars();
        let mut split = None;
        for (i, c) in chars.by_ref().enumerate() {
            match (c.is_alphabetic(), c.is_ascii_digit(), split) {
                (true, _, None) | (false, true, Some(_)) => continue,
                (false, true, None) => split = Some(i),
                (true, _, Some(_)) | (false, false, _) => {
                    return Err(XError::InvalidCoords(coords));
                }
            }
        }
        let split = match split {
            Some(split) => split,
            None => return Err(XError::InvalidCoords(coords)),
        };

        let row = coords[split..].parse::<usize>()? as u32 - 1;
        let col = Coords::a1_column_to_index(&coords[..split])? as u16;

        Ok(Self { row, col })
    }

    /// Convert a 0-based row and column to A1 notation.
    ///
    /// # Examples
    /// ```ignore
    /// assert_eq!(to_a1_coords(0, 0), "A1");
    /// assert_eq!(to_a1_coords(0, 1), "B1");
    /// assert_eq!(to_a1_coords(0, 25), "Z1");
    /// assert_eq!(to_a1_coords(5, 26), "AA6");
    /// assert_eq!(to_a1_coords(100, 27), "AB101");
    /// assert_eq!(to_a1_coords(100, 28), "AC101");
    /// ```
    pub fn to_a1_coords(&self) -> String {
        let col = self.col + 1;
        let mut s = String::new();
        let mut n = col;
        while n > 0 {
            let rem = n % 26;
            if rem == 0 {
                s.push('Z');
                n = n / 26 - 1;
            } else {
                s.push((rem as u8 + b'A' - 1) as char);
                n /= 26;
            }
        }
        s = s.chars().rev().collect();

        s.push_str(&(self.row + 1).to_string());
        s
    }

    /// Converts an A1 column notation to a 0-based index.
    ///
    /// Example:
    /// ```ignore
    /// Coords::a1_column_to_index("A")?; // 0
    /// Coords::a1_column_to_index("Z")?; // 25
    /// Coords::a1_column_to_index("AA")?; // 26
    /// ```
    fn a1_column_to_index(column: &str) -> XResult<usize> {
        if column.is_empty() {
            return Err(XError::InvalidCoords(
                "Invalid column notations: empty".into(),
            ));
        }

        let column = column.to_ascii_uppercase();
        let mut result = 0;

        for c in column.chars() {
            if !c.is_ascii_alphabetic() {
                return Err(XError::InvalidCoords(format!(
                    "Invalid column notations: {}",
                    column
                )));
            }

            let value = (c as u8) - b'A' + 1;
            result = result * 26 + value as usize;
        }

        // Adjust for 0-based indexing
        Ok(result - 1)
    }
}

#[cfg(test)]
mod coords_tests {
    use crate::error::XResult;
    use crate::xlsx::coords::Coords;

    #[test]
    fn from_a1() -> XResult<()> {
        let c = Coords::from_a1("A1")?;
        assert_eq!(c.row, 0);
        assert_eq!(c.col, 0);

        let c = Coords::from_a1("A3")?;
        assert_eq!(c.row, 2);
        assert_eq!(c.col, 0);

        let c = Coords::from_a1("C10")?;
        assert_eq!(c.row, 9);
        assert_eq!(c.col, 2);

        let c = Coords::from_a1("AA3")?;
        assert_eq!(c.row, 2);
        assert_eq!(c.col, 26);

        let c = Coords::from_a1("$AA$3")?;
        assert_eq!(c.row, 2);
        assert_eq!(c.col, 26);

        assert_eq!(Coords::from_a1("A").is_err(), true);
        assert_eq!(Coords::from_a1("1").is_err(), true);
        assert_eq!(Coords::from_a1("A1A").is_err(), true);
        assert_eq!(Coords::from_a1(".").is_err(), true);

        Ok(())
    }

    #[test]
    fn a1_to_col_index() -> XResult<()> {
        let tests = vec![("A", 0), ("Z", 25), ("AA", 26), ("BA", 52)];

        for (a1, index) in tests {
            let result = Coords::a1_column_to_index(a1)?;
            assert_eq!(result, index);
        }
        Ok(())
    }

    #[test]
    fn to_a1_coords() {
        let coords = |row, col| Coords { row, col }.to_a1_coords();
        assert_eq!(coords(0, 0), "A1");
        assert_eq!(coords(0, 1), "B1");
        assert_eq!(coords(0, 25), "Z1");
        assert_eq!(coords(5, 26), "AA6");
        assert_eq!(coords(100, 27), "AB101");
        assert_eq!(coords(100, 28), "AC101");
    }

    #[test]
    fn col_index() {
        assert_eq!(0, Coords::a1_column_to_index("A").unwrap());

        assert!(Coords::a1_column_to_index("1").is_err());
    }
}
