#[derive(Debug)]
pub enum ProtocolError {
  UnexpectedCharacter,
  Io(std::io::Error),
  StrUtf8(std::str::Utf8Error),
  StringUtf8(std::string::FromUtf8Error),
  ParseInt(std::num::ParseIntError),
  SerdeJson(serde_json::Error),
}

impl std::fmt::Display for ProtocolError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ProtocolError::UnexpectedCharacter => f.write_str("unexpected character"),
      ProtocolError::Io(err) => err.fmt(f),
      ProtocolError::StrUtf8(err) => err.fmt(f),
      ProtocolError::StringUtf8(err) => err.fmt(f),
      ProtocolError::ParseInt(err) => err.fmt(f),
      ProtocolError::SerdeJson(err) => err.fmt(f),
    }
  }
}

impl std::error::Error for ProtocolError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      ProtocolError::UnexpectedCharacter => None,
      ProtocolError::Io(err) => Some(err),
      ProtocolError::StrUtf8(err) => Some(err),
      ProtocolError::StringUtf8(err) => Some(err),
      ProtocolError::ParseInt(err) => Some(err),
      ProtocolError::SerdeJson(err) => Some(err),
    }
  }

  fn cause(&self) -> Option<&dyn std::error::Error> {
    self.source()
  }
}

impl From<std::io::Error> for ProtocolError {
  fn from(value: std::io::Error) -> Self {
    Self::Io(value)
  }
}

impl From<std::str::Utf8Error> for ProtocolError {
  fn from(value: std::str::Utf8Error) -> Self {
    Self::StrUtf8(value)
  }
}

impl From<std::string::FromUtf8Error> for ProtocolError {
  fn from(value: std::string::FromUtf8Error) -> Self {
    Self::StringUtf8(value)
  }
}

impl From<std::num::ParseIntError> for ProtocolError {
  fn from(value: std::num::ParseIntError) -> Self {
    Self::ParseInt(value)
  }
}

impl From<serde_json::Error> for ProtocolError {
  fn from(value: serde_json::Error) -> Self {
    Self::SerdeJson(value)
  }
}
