use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use crate::model::store;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
	Store(store::Error),
	EntityNotFound { entity: &'static str, id: i64 },
	Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

impl From<store::Error> for Error {
	fn from(value: store::Error) -> Self {
		Self::Store(value)
	}
}

impl From<sqlx::Error> for Error {
	fn from(value: sqlx::Error) -> Self {
		Self::Sqlx(value)
	}
}