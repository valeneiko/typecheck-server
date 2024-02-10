use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AutoOnOff {
  Auto,
  On,
  Off,
}

/// https://github.com/microsoft/TypeScript/blob/61200368bb440ba8a40641be87c44d875ca31f69/src/server/protocol.ts#L3519
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPreferences {
  #[serde(skip_serializing_if = "Option::is_none")]
  include_package_json_auto_imports: Option<AutoOnOff>,
}

/// https://github.com/microsoft/TypeScript/blob/61200368bb440ba8a40641be87c44d875ca31f69/src/server/protocol.ts#L1635
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchOptions {
  #[serde(skip_serializing_if = "Option::is_none")]
  exclude_directories: Option<Vec<String>>,
}

/// https://github.com/microsoft/TypeScript/blob/61200368bb440ba8a40641be87c44d875ca31f69/src/server/protocol.ts#L1585
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigureRequest {
  #[serde(skip_serializing_if = "Option::is_none")]
  preferences: Option<UserPreferences>,
  #[serde(skip_serializing_if = "Option::is_none")]
  watch_options: Option<WatchOptions>,
}

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ScriptKindName {
  Ts,
  Js,
  Tsx,
  Jsx,
}

/// https://github.com/microsoft/TypeScript/blob/61200368bb440ba8a40641be87c44d875ca31f69/src/server/protocol.ts#L1715
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenRequest<'a> {
  file: &'a str,
  #[serde(skip_serializing_if = "Option::is_none")]
  project_file_name: Option<&'a str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  file_content: Option<&'a str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  script_kind_name: Option<ScriptKindName>,
  #[serde(skip_serializing_if = "Option::is_none")]
  project_root_path: Option<&'a str>,
}

/// https://github.com/microsoft/TypeScript/blob/61200368bb440ba8a40641be87c44d875ca31f69/src/server/protocol.ts#L292
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileRequest<'a> {
  file: &'a str,
  #[serde(skip_serializing_if = "Option::is_none")]
  project_file_name: Option<&'a str>,
}
