use dioxus::prelude::*;
use std::env;
use std::io::Read;
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;

pub fn find_cli_executable() -> Option<PathBuf> {
    let gui_exe_path = env::current_exe().ok()?;
    let gui_exe_dir = gui_exe_path.parent()?;

    let bundled_name = "anytype-x86_64-pc-windows-msvc";

    let possible_paths = vec![
        gui_exe_dir.join(bundled_name),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("anytype-binaries")
            .join(bundled_name),
    ];

    for path in possible_paths {
        if path.exists() && path.is_file() {
            tracing::info!("Found CLI executable at: {:?}", path);
            return Some(path);
        }
    }

    tracing::error!(
        "CLI not found. Place '{}' in {:?}",
        bundled_name,
        env!("CARGO_MANIFEST_DIR")
    );
    None
}

pub static SERVER_PROCESS: GlobalSignal<Mutex<Option<Child>>> = Signal::global(|| Mutex::new(None));

const CREATE_NO_WINDOW: u32 = 0x08000000;

pub async fn start_anytype_server() -> Result<(), String> {
    let Some(bin_path) = find_cli_executable() else {
        tracing::error!("Executable path resolution failed.");
        return Err("Could not find anytype-cli executable in any expected location.".into());
    };

    tracing::info!("Starting Anytype node from path: {:?}", bin_path);

    let child = tokio::task::spawn_blocking(move || {
        let mut child = std::process::Command::new(&bin_path)
            .arg("serve")
            .arg("--listen-address")
            .arg("127.0.0.1:31020")
            .creation_flags(CREATE_NO_WINDOW)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| {
                let err_msg = format!("OS failed to spawn binary: {e}");
                tracing::error!("{err_msg}");
                err_msg
            })?;

        tracing::info!("OS successfully spawned process with PID: {:?}", child.id());

        std::thread::sleep(std::time::Duration::from_millis(500));

        match child.try_wait() {
            Ok(Some(status)) => {
                let mut err_msg = String::new();
                if let Some(mut stderr) = child.stderr.take() {
                    let _ = stderr.read_to_string(&mut err_msg);
                }

                let diagnostic = format!(
                    "Process exited immediately. Exit Code: {:?}. Stderr Output: {}",
                    status.code(),
                    err_msg.trim()
                );
                tracing::error!("{}", diagnostic);
                return Err(diagnostic);
            }
            Ok(None) => {
                tracing::info!(
                    "Process is still running after 500ms. Checking for silent stalls..."
                );
            }
            Err(e) => {
                tracing::error!("Failed to query process status: {}", e);
            }
        }

        let _stdout_pipe = child.stdout.take();
        let _stderr_pipe = child.stderr.take();

        Ok(child)
    })
    .await
    .map_err(|e| format!("Tokio worker thread panicked or aborted: {}", e))??; // Unwraps the JoinError AND the underlying Result

    if let Ok(mut guard) = SERVER_PROCESS.read().lock() {
        *guard = Some(child);
    }

    tracing::info!("Anytype server process registered globally and running cleanly.");
    Ok(())
}

pub async fn login_to_account(account_key: String) -> Result<(), String> {
    let Some(bin_path) = find_cli_executable() else {
        return Err("CLI executable not found".into());
    };

    tracing::info!("Logging in to Anytype account...");

    let output = Command::new(bin_path)
        .arg("auth")
        .arg("login")
        .arg("--listen-address")
        .arg("127.0.0.1:31020")
        .arg("--account-key")
        .arg(account_key)
        .creation_flags(CREATE_NO_WINDOW)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();
    match output {
        Ok(out) if out.status.success() => {
            tracing::info!("CLI login successful");
            Ok(())
        }
        Ok(out) => {
            let err = String::from_utf8_lossy(&out.stderr);
            tracing::error!("CLI login failed: {}", err);
            Err(err.to_string())
        }
        Err(e) => Err(format!("Failed to execute login command: {}", e)),
    }
}

pub fn stop_anytype_server() {
    if let Ok(mut guard) = SERVER_PROCESS.read().lock() {
        if let Some(mut child) = guard.take() {
            tracing::info!("Stopping Anytype server process (PID: {})...", child.id());
            let _ = child.kill();
            let _ = child.wait();
            tracing::info!("Anytype server process cleaned up successfully.");
            return;
        }
    }
    tracing::info!("No active Anytype server process found to stop.");
}
