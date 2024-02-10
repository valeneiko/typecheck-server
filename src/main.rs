use std::{
  error::Error,
  process::{Command, Stdio},
};

use tsserver_client::tsserver::client::TSServerClient;

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

  let mut client = TSServerClient::try_from(&mut tsserver)?;

  let response = client.status()?;
  println!("{}", response);

  client.exit()?;

  let status = tsserver.wait()?;
  println!("Server exited with {}", status);

  Ok(())
}
