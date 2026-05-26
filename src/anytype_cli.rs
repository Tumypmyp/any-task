// use dioxus::prelude::*;
// use std::env;
// use std::io::Read;
// #[cfg(target_os = "windows")]
// use std::os::windows::process::CommandExt;

// #[cfg(target_os = "windows")]
// const CREATE_NO_WINDOW: u32 = 0x08000000;

// use std::path::PathBuf;
// use std::process::{Child, Command, Stdio};
// use std::sync::Mutex;

// pub fn find_anytype_cli_executable() -> Result<PathBuf, String> {
//     let gui_exe_path =
//         env::current_exe().map_err(|e| format!("Failed to get current executable path: {}", e))?;
//     let gui_exe_dir = gui_exe_path
//         .parent()
//         .ok_or_else(|| "Failed to get parent directory of the current executable")?;

//     // // 1. Get the target triple string injected by Cargo at compile time
//     // let target_triple = std::env::var("TARGET");

//     // //     // 2. Format name to match Tauri's sidecar pattern: name-target_triple
//     // let bundled_name = format!("anytype-{:?}", target_triple);
//     // tracing::info!("Using bundled cli with name: {:?}", target_triple);
//     #[cfg(target_os = "android")]
//     let bundled_name = "anytype-aarch64-linux-android";
//     #[cfg(not(target_os = "android"))]
//     let bundled_name = "anytype-x86_64-pc-windows-msvc";

//     let possible_paths = vec![
//         gui_exe_dir.join(&bundled_name),
//         PathBuf::from(env!("CARGO_MANIFEST_DIR"))
//             .join("anytype-binaries")
//             .join(bundled_name),
//     ];

//     for path in possible_paths {
//         tracing::info!("Checking path: {:?}", path);
//         if path.exists() && path.is_file() {
//             tracing::info!("Found CLI executable at: {:?}", path);
//             return Ok(path);
//         }
//     }
//     Err("Failed to find in expected locations".to_string())
// }

// pub static SERVER_PROCESS: GlobalSignal<Mutex<Option<Child>>> = Signal::global(|| Mutex::new(None));

// pub async fn start_anytype_server() -> Result<(), String> {
//     tracing::info!("Starting anytype-cli");

//     let bin_path = find_anytype_cli_executable()
//         .map_err(|err| format!("Failed to find anytype-cli executable: {}", err))?;

//     let child = tokio::task::spawn_blocking(move || {
//         let mut cmd = std::process::Command::new(&bin_path);
//         cmd.arg("serve")
//             .arg("--listen-address")
//             .arg("127.0.0.1:31020")
//             .stdout(Stdio::piped())
//             .stderr(Stdio::piped());
//         #[cfg(target_os = "windows")]
//         {
//             cmd.creation_flags(CREATE_NO_WINDOW);
//         }
//         let mut child = cmd.spawn().map_err(|e| {
//             let err_msg = format!("OS failed to spawn binary: {e}");
//             tracing::error!("{err_msg}");
//             err_msg
//         })?;

//         tracing::info!("OS successfully spawned process with PID: {:?}", child.id());

//         std::thread::sleep(std::time::Duration::from_millis(500));

//         match child.try_wait() {
//             Ok(Some(status)) => {
//                 let mut err_msg = String::new();
//                 if let Some(mut stderr) = child.stderr.take() {
//                     let _ = stderr.read_to_string(&mut err_msg);
//                 }

//                 let diagnostic = format!(
//                     "Process exited immediately. Exit Code: {:?}. Stderr Output: {}",
//                     status.code(),
//                     err_msg.trim()
//                 );
//                 tracing::error!("{}", diagnostic);
//                 return Err(diagnostic);
//             }
//             Ok(None) => {
//                 tracing::info!(
//                     "Process is still running after 500ms. Checking for silent stalls..."
//                 );
//             }
//             Err(e) => {
//                 tracing::error!("Failed to query process status: {}", e);
//             }
//         }

//         let _stdout_pipe = child.stdout.take();
//         let _stderr_pipe = child.stderr.take();

//         Ok(child)
//     })
//     .await
//     .map_err(|e| format!("Tokio worker thread panicked or aborted: {}", e))??;

//     if let Ok(mut guard) = SERVER_PROCESS.read().lock() {
//         *guard = Some(child);
//     }

//     tracing::info!("Anytype server process registered globally and running cleanly.");
//     Ok(())
// }

// pub async fn login_to_account(account_key: String) -> Result<(), String> {
//     tracing::info!("Logging in to Anytype account...");

//     let bin_path = find_anytype_cli_executable()
//         .map_err(|err| format!("Failed to find anytype-cli executable: {}", err))?;

//     let mut cmd = Command::new(bin_path);
//     cmd.arg("auth")
//         .arg("login")
//         .arg("--listen-address")
//         .arg("127.0.0.1:31020")
//         .arg("--account-key")
//         .arg(account_key)
//         .stdout(Stdio::piped())
//         .stderr(Stdio::piped());
//     #[cfg(target_os = "windows")]
//     {
//         cmd.creation_flags(CREATE_NO_WINDOW);
//     }
//     let output = cmd.output();
//     match output {
//         Ok(out) if out.status.success() => {
//             tracing::info!("CLI login successful");
//             Ok(())
//         }
//         Ok(out) => {
//             let err = String::from_utf8_lossy(&out.stderr);
//             Err(err.to_string())
//         }
//         Err(e) => Err(format!("Failed to execute login command: {}", e)),
//     }
// }

// pub fn stop_anytype_server() {
//     if let Ok(mut guard) = SERVER_PROCESS.read().lock() {
//         if let Some(mut child) = guard.take() {
//             tracing::info!("Stopping Anytype server process (PID: {})...", child.id());
//             let _ = child.kill();
//             let _ = child.wait();
//             tracing::info!("Anytype server process cleaned up successfully.");
//             return;
//         }
//     }
//     tracing::error!("No active Anytype server process found to stop.");
// }
