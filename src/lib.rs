pub mod error;
pub mod handler;
pub mod view;

pub type Result<T> = std::result::Result<T, error::AppError>;
