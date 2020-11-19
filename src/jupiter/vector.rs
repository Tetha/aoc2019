
use std::error::Error;
use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "<x={},y={},z={}>", self.x, self.y, self.z)
    }
}

impl FromStr for Vector {
    type Err = VectorParseError;
    fn from_str(input: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let mut x = None;
        let mut y = None;
        let mut z = None;
        for chunk in input.trim().trim_matches(|p| p == '<' || p == '>')
                          .split(',') {
            let sub_chunks = chunk.split('=').collect::<Vec<&str>>();
            if sub_chunks.len() != 2 {
                return Err(VectorParseError{message: format!("unable to understand chunk {}", chunk)});
            }
            match sub_chunks[0].trim() {
                "x" => {
                    if x.is_some() {
                        return Err(VectorParseError{ message: "duplicate x".to_string() })
                    } else {
                        let x_val = sub_chunks[1].parse::<i32>()?;
                        x = Some(x_val);
                    }
                },
                "y" => {
                    if y.is_some() {
                        return Err(VectorParseError{ message: "duplicate y".to_string() })
                    } else {
                        let y_val = sub_chunks[1].parse::<i32>()?;
                        y = Some(y_val);
                    }
                },
                "z" => {
                    if z.is_some() {
                        return Err(VectorParseError{ message: "duplicate z".to_string() })
                    } else {
                        let z_val = sub_chunks[1].parse::<i32>()?;
                        z = Some(z_val);
                    }

                },
                other => return Err(VectorParseError{message: format!("unknown component {} in chunk {}" , other, chunk)})
            }
        }
        if let (Some(x), Some(y), Some(z)) = (x,y,z) {
            Ok(Vector{x, y, z})
        } else {
            Err(VectorParseError{message: format!("incomplete vector - x={:?}, y={:?}, z={:?}", x, y, z)})
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct VectorParseError {
    message: String
}
impl Display for VectorParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "unable to parse vector: {}", self.message)
    }
}

impl Error for VectorParseError {}

impl From<ParseIntError> for VectorParseError {
    fn from(err: ParseIntError) -> Self {
        VectorParseError{message: format!("cannot parse component: {}", err)}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_parse() {
        let subject = Vector::from_str("<x=1,y=2,z=3>");
        assert_eq!(subject, Ok(Vector{x: 1, y: 2, z: 3}));
    }
}