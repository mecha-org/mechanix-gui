/*
Copyright 2025 Jeremie Drouet <jeremie.drouet@gmail.com>

MIT License

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use std::fmt;
use toml::Value;
use toml::map::Map;

#[derive(Debug, PartialEq)]
pub struct Error {
    pub path: String,
    pub expected: &'static str,
    pub existing: &'static str,
}

impl Error {
    pub fn new(path: String, expected: &'static str, existing: &'static str) -> Self {
        Self {
            path,
            expected,
            existing,
        }
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "Incompatible types at path \"{}\", expected \"{}\" received \"{}\".",
            self.path, self.expected, self.existing
        )
    }
}

fn merge_into_table_inner(
    value: &mut Map<String, Value>,
    other: Map<String, Value>,
    path: &str,
) -> Result<(), Error> {
    for (name, inner) in other {
        if let Some(existing) = value.remove(&name) {
            let inner_path = format!("{path}.{name}");
            value.insert(name, merge_inner(existing, inner, &inner_path)?);
        } else {
            value.insert(name, inner);
        }
    }
    Ok(())
}

/// Merges two toml tables into a single one.
pub fn merge_tables(
    mut value: Map<String, Value>,
    other: Map<String, Value>,
) -> Result<Map<String, Value>, Error> {
    merge_into_table_inner(&mut value, other, "$")?;
    Ok(value)
}

/// Merges two toml tables into a single one.
pub fn merge_into_table(
    value: &mut Map<String, Value>,
    other: Map<String, Value>,
) -> Result<(), Error> {
    merge_into_table_inner(value, other, "$")
}

fn merge_inner(value: Value, other: Value, path: &str) -> Result<Value, Error> {
    match (value, other) {
        (Value::String(_), Value::String(inner)) => Ok(Value::String(inner)),
        (Value::Integer(_), Value::Integer(inner)) => Ok(Value::Integer(inner)),
        (Value::Float(_), Value::Float(inner)) => Ok(Value::Float(inner)),
        (Value::Boolean(_), Value::Boolean(inner)) => Ok(Value::Boolean(inner)),
        (Value::Datetime(_), Value::Datetime(inner)) => Ok(Value::Datetime(inner)),
        (Value::Array(mut existing), Value::Array(inner)) => {
            existing.extend(inner);
            Ok(Value::Array(existing))
        }
        (Value::Table(mut existing), Value::Table(inner)) => {
            merge_into_table_inner(&mut existing, inner, path)?;
            Ok(Value::Table(existing))
        }
        (v, o) => Err(Error::new(path.to_owned(), v.type_str(), o.type_str())),
    }
}

/// Merges two toml values into a single one.
pub fn merge(value: Value, other: Value) -> Result<Value, Error> {
    merge_inner(value, other, "$")
}

#[cfg(test)]
mod tests {
    use crate::toml_merge::{Error, merge};

    macro_rules! should_fail {
        ($first: expr, $second: expr) => {
            should_fail!($first, $second,)
        };
        ($first: expr, $second: expr,) => {{
            let first = $first.parse::<toml::Table>().unwrap();
            let second = $second.parse::<toml::Table>().unwrap();
            merge(first.into(), second.into()).unwrap_err()
        }};
    }

    macro_rules! should_match {
        ($first: expr, $second: expr, $result: expr) => {
            should_match!($first, $second, $result,)
        };
        ($first: expr, $second: expr, $result: expr,) => {
            let first = $first.parse::<toml::Table>().unwrap();
            let second = $second.parse::<toml::Table>().unwrap();
            let result = $result.parse::<toml::Table>().unwrap();
            assert_eq!(merge(first.into(), second.into()).unwrap(), result.into());
        };
    }

    #[test]
    fn with_basic() {
        should_match!(
            r#"string = 'foo'
integer = 42
float = 42.24
boolean = true
keep_me = true
"#,
            r#"string = 'bar'
integer = 43
float = 24.42
boolean = false
missing = true
"#,
            r#"string = 'bar'
integer = 43
float = 24.42
boolean = false
keep_me = true
missing = true
"#,
        );
    }

    #[test]
    fn with_array() {
        should_match!(
            r#"foo = ["a", "b"]"#,
            r#"foo = ["c", "d"]"#,
            r#"foo = ["a", "b", "c", "d"]"#,
        );
    }

    #[test]
    fn with_table() {
        should_match!(
            r#"
            [foo]
            bar = "baz"
        "#,
            r#"
            [foo]
            hello = "world"
        "#,
            r#"
            [foo]
            bar = "baz"
            hello = "world"
        "#,
        );
    }

    #[test]
    fn invalid_kinds() {
        assert_eq!(
            should_fail!("foo = true", "foo = 42"),
            Error::new("$.foo".to_owned(), "boolean", "integer")
        );
        assert_eq!(
            should_fail!("foo = \"true\"", "foo = 42.5"),
            Error::new("$.foo".to_owned(), "string", "float")
        );
    }
}
