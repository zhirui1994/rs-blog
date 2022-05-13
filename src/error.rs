#[derive(Debug)]
pub enum AppErrorType {
  Db,
  Template,
  NotFound,
}

#[derive(Debug)]
pub struct AppError {
  pub message: Option<String>,
  pub cause: Option<Box<dyn std::error::Error>>,
  pub types: AppErrorType,
}

impl AppError {
  fn new(message: Option<String>, cause: Option<Box<dyn std::error::Error>>, types: AppErrorType) -> Self {
    Self { message, cause, types }
  }
  fn from_err(cause: Option<Box<dyn std::error::Error>>, types: AppErrorType) -> Self {
    Self::new(None, Some(cause), types)
  }
  fn from_str(msg:&str, types: AppErrorType) -> Self {
    Self::new(Some(msg.to_string()), None, types)
  }
  pub fn notfound_opt(message:Option<String>) -> Self {
    Self::new(message, None, AppErrorType::Notfound)
  }
  pub fn notfound_msg(msg:&str) -> Self {
    Self::notfound_opt(Some(msg.to_string()))
  }
  pub fn notfound()->Self {
    Self::notfound_msg("No matching data found!")
  }
}
