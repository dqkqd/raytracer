use serde::Deserialize;

pub(crate) mod object;

pub(crate) mod camera;

pub(crate) mod light;

pub(super) mod material;

pub(crate) mod shape;

pub(crate) mod transform;

pub(crate) mod color;

pub(crate) mod point;

pub(crate) mod vector;

pub(crate) mod pattern;

#[allow(dead_code)]
pub(super) type ParseResult<T> = Result<T, serde_yaml::Error>;

pub(crate) trait ObjectParser<T>: Sized + for<'de> Deserialize<'de> {
    fn parse(&self) -> T;
    fn from_value(value: serde_yaml::Value) -> Result<T, serde_yaml::Error> {
        let parser: Self = serde_yaml::from_value(value)?;
        Ok(parser.parse())
    }
}
