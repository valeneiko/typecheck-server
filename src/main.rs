use std::{
  error::Error,
  process::{Command, Stdio},
};

use tsserver_client::tsserver::{
  client::TSServerClient,
  requests::{FileRequest, LocationRequest, NodeRequest, OpenRequest},
};

const SAMPLE_CONTENT: &'static str = r#"async function funcAsync() {
  return Promise.resolve();
}

function func() {
  return 7;
}

export async function test() {
  func();
  funcAsync();
}
"#;

fn main() -> Result<(), Box<dyn Error>> {
  let cwd = std::env::current_dir()?;
  println!("Working Dir: {}", cwd.display());

  // Start the TSServer
  let mut tsserver = Command::new("node")
    .args([
      "--max-old-space-size=4096",
      // "./node_modules/typescript/lib/tsserver.js",
      "./dist/server.js",
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

  // Just a dummy command to check communication is working, not really needed
  let response = client.status()?;
  println!("{}", response);

  // First we need to Open the file to lint
  // Sending content is optional, if not sent TSServer will read the file from disk
  let file = cwd.join("./src_js/sample.ts");
  let file = file.to_str().expect("path join should succeed");
  println!("Sample file: {file}");
  client.open(OpenRequest { file, file_content: Some(SAMPLE_CONTENT) })?;

  // The following 2 calls are not needed, but could be useful for debugging
  let response = client.get_node(NodeRequest { file, line: 9, col: 5, kind: "CallExpression" })?;
  println!("{}", response);

  let response = client.get_node(NodeRequest { file, line: 10, col: 5, kind: "CallExpression" })?;
  println!("{}", response);

  // Actually run the check the type of Node (like you would do in ESLint rule)
  let response = client.is_promise_like(LocationRequest { file, line: 9, col: 5 })?;
  println!("{}", response);

  let response = client.is_promise_like(LocationRequest { file, line: 10, col: 5 })?;
  println!("{}", response);

  // Close the file to allow TSServer to cleanup resources
  client.close(FileRequest { file })?;

  // Terminate the server
  client.exit()?;
  let status = tsserver.wait()?;
  println!("Server exited with {}", status);

  Ok(())
}
