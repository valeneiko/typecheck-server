use std::process::{Child, ChildStdin, ChildStdout};

use crate::ProtocolError;

use super::{requests::*, utils::read_message};

pub struct TSServerClient<W: std::io::Write, R: std::io::Read> {
  seq: usize,
  command_stream: W,
  result_stream: R,
}

impl<W: std::io::Write, R: std::io::Read> TSServerClient<W, R> {
  pub fn status(&mut self) -> Result<String, ProtocolError> {
    self.send_command("status", None)?;

    let response = read_message(&mut self.result_stream)?;
    Ok(response)
  }

  pub fn exit(&mut self) -> Result<(), ProtocolError> {
    self.send_command("exit", None)?;
    Ok(())
  }

  pub fn open(&mut self, opts: OpenRequest<'_>) -> Result<(), ProtocolError> {
    let args = serde_json::to_string(&opts)?;
    self.send_command("open", Some(args.as_str()))?;
    Ok(())
  }

  pub fn close(&mut self, opts: FileRequest<'_>) -> Result<(), ProtocolError> {
    let args = serde_json::to_string(&opts)?;
    self.send_command("close", Some(args.as_str()))?;
    Ok(())
  }

  pub fn get_node(&mut self, opts: NodeRequest<'_>) -> Result<String, ProtocolError> {
    let args = serde_json::to_string(&opts)?;
    self.send_command("getNode", Some(args.as_str()))?;

    let response = read_message(&mut self.result_stream)?;
    Ok(response)
  }

  pub fn is_promise_array(&mut self, opts: LocationRequest<'_>) -> Result<String, ProtocolError> {
    let args = serde_json::to_string(&opts)?;
    self.send_command("noFloatingPromises::isPromiseArray", Some(args.as_str()))?;

    let response = read_message(&mut self.result_stream)?;
    Ok(response)
  }

  pub fn is_promise_like(&mut self, opts: LocationRequest<'_>) -> Result<String, ProtocolError> {
    let args = serde_json::to_string(&opts)?;
    self.send_command("noFloatingPromises::isPromiseLike", Some(args.as_str()))?;

    let response = read_message(&mut self.result_stream)?;
    Ok(response)
  }

  fn send_command(&mut self, command: &str, args: Option<&str>) -> Result<(), std::io::Error> {
    self.seq += 1;
    let seq = self.seq;
    let args_str = args.map(|x| format!(r#","arguments":{x}"#)).unwrap_or("".to_string());
    let msg =
      format!("{{\"seq\":{seq},\"type\":\"request\",\"command\":\"{command}\"{args_str}}}\n");

    self.command_stream.write_all(msg.as_bytes())
  }
}

#[derive(Debug)]
pub enum FromChildError {
  MissingStdoutStream,
  MissingStdinStream,
}

impl std::fmt::Display for FromChildError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      FromChildError::MissingStdoutStream => f.write_str("child stdout must be piped"),
      FromChildError::MissingStdinStream => f.write_str("child stdin must be piped"),
    }
  }
}

impl std::error::Error for FromChildError {}

impl TryFrom<&mut Child> for TSServerClient<ChildStdin, ChildStdout> {
  type Error = FromChildError;

  fn try_from(value: &mut Child) -> Result<Self, Self::Error> {
    let command_stream = value.stdin.take().ok_or(FromChildError::MissingStdinStream)?;
    let result_stream = value.stdout.take().ok_or(FromChildError::MissingStdoutStream)?;

    Ok(Self { seq: 0, command_stream, result_stream })
  }
}
