use std::ffi::{c_char, c_int};
use std::net::TcpStream;
use std::time::Duration;
unsafe extern "C" {
    fn StartAnytypeEngine(grpc_addr: *const c_char) -> c_int;
    fn StopAnytypeEngine();
}
pub fn start_engine(grpc_addr: &str) -> Result<(), String> {
    if TcpStream::connect_timeout(&grpc_addr.parse().unwrap(), Duration::from_millis(50))
        .is_ok()
    {
        tracing::info!(
            "Anytype Engine is already running from a previous session. Reusing it."
        );
        return Ok(());
    }
    unsafe {
        std::env::set_var("ANYTYPE_GATEWAY_ADDR", "127.0.0.1:0");
    };
    tracing::info!("No active engine detected. Spawning new process...");
    let grpc_c = std::ffi::CString::new(grpc_addr).map_err(|_| "Invalid string")?;
    let ptr = grpc_c.as_ptr();
    std::mem::forget(grpc_c);
    tracing::info!("Starting in-process gRPC engine at {}", grpc_addr);
    let ret = unsafe { StartAnytypeEngine(ptr) };
    if ret == 0 {
        std::thread::sleep(Duration::from_millis(200));
        Ok(())
    } else {
        Err("Failed to start engine".into())
    }
}
pub fn stop_engine() {
    tracing::info!("Stopping in-process gRPC engine");
    unsafe { StopAnytypeEngine() };
}
