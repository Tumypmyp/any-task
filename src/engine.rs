use std::ffi::{c_char, c_int};
use std::time::Duration;

unsafe extern "C" {
    fn StartAnytypeEngine(grpc_addr: *const c_char) -> c_int;
    fn StopAnytypeEngine();
}

pub fn start_engine(grpc_addr: &str) -> Result<(), String> {
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
