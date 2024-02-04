use std::{
  error::Error,
  io::Write,
  process::{Command, Stdio},
};

use tsserver_client::tsserver::utils::read_message;

fn main() -> Result<(), Box<dyn Error>> {
  println!("Working Dir: {}", std::env::current_dir()?.display());

  let mut tsserver = Command::new("node")
    .args([
      "--max-old-space-size=4096",
      "./node_modules/typescript/lib/tsserver.js",
      "--disableAutomaticTypingAcquisition",
      "--suppressDiagnosticEvents",
    ])
    .env_clear()
    .env("NODE_ENV", "production")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::null())
    .spawn()?;

  let mut command_stream = tsserver.stdin.take().expect("stdin should be piped");
  let mut result_stream = tsserver.stdout.take().expect("stdout should be piped");

  println!("Writing commands");
  command_stream
    .write_all(format!("{}\n", r#"{"seq":0,"type":"request","command":"status"}"#,).as_bytes())
    .unwrap();

  println!("Waiting for response");
  let response = read_message(&mut result_stream)?;
  println!("{}", response);

  command_stream
    .write_all(format!("{}\n", r#"{"seq":999,"type":"request","command":"exit"}"#).as_bytes())
    .unwrap();

  println!("Waiting for close");
  let status = tsserver.wait()?;
  println!("Server exited with {}", status);

  Ok(())
}

// fn execute_command(
//   command_stream: &mut ChildStdin,
//   result_stream: &mut ChildStdout,
//   request: &str,
// ) -> Result<(), Box<dyn Error>> {
//   command_stream.write_all(format!("{}\n", request))?;

//   Ok(())
// }
