fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let output_base = std::env::var("OUTPUT_BASE")
        .unwrap_or_else(|_| format!("{manifest_dir}/go-engine/native-libs"));
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    match target_os.as_str() {
        "windows" => {
            println!("cargo:rustc-link-search=native={output_base}/windows");
            println!("cargo:rustc-link-lib=dylib=anytype_engine");
            println!("cargo:rerun-if-env-changed=OUTPUT_BASE");
            println!(
                "cargo:rerun-if-changed={output_base}/windows/libanytype_engine.dll",
            );
            let src_dll = std::path::PathBuf::from(&output_base)
                .join("windows")
                .join("libanytype_engine.dll");
            if src_dll.exists() {
                if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
                    let p_root = std::path::PathBuf::from(manifest_dir);
                    let _ = std::fs::copy(
                        &src_dll,
                        p_root.join("libanytype_engine.dll"),
                    );
                    let dx_windows_dir = p_root
                        .join("target")
                        .join("dx")
                        .join("any-task")
                        .join("debug")
                        .join("windows");
                    if !dx_windows_dir.exists() {
                        let _ = std::fs::create_dir_all(&dx_windows_dir);
                    }
                    let _ = std::fs::copy(
                        &src_dll,
                        dx_windows_dir.join("libanytype_engine.dll"),
                    );
                }
                if let Ok(out_dir) = std::env::var("OUT_DIR") {
                    let mut profile_dir = std::path::PathBuf::from(out_dir);
                    profile_dir.pop();
                    profile_dir.pop();
                    profile_dir.pop();
                    let _ = std::fs::copy(
                        &src_dll,
                        profile_dir.join("libanytype_engine.dll"),
                    );
                    let _ = std::fs::copy(
                        &src_dll,
                        profile_dir.join("deps").join("libanytype_engine.dll"),
                    );
                }
            } else {
                println!(
                    "cargo:warning=anytype_engine.dll was not found at the expected source path!",
                );
            }
        }
        "linux" => {
            println!("cargo:rustc-link-search=native={output_base}/linux");
            println!("cargo:rustc-link-lib=dylib=anytype_engine");
            println!("cargo:rerun-if-env-changed=OUTPUT_BASE");
            println!("cargo:rerun-if-changed={output_base}/linux/libanytype_engine.so");
        }
        "android" => {
            let (nix_arch, jni_abi) = match std::env::var("CARGO_CFG_TARGET_ARCH")
                .unwrap_or_default()
                .as_str()
            {
                "aarch64" => ("aarch64", "arm64-v8a"),
                "arm" => ("armv7", "armeabi-v7a"),
                "x86_64" => ("x86_64", "x86_64"),
                "x86" => ("i686", "x86"),
                _ => return Ok(()),
            };
            println!("cargo:rustc-link-search=native={output_base}/android/{nix_arch}");
            println!("cargo:rustc-link-lib=dylib=anytype_engine");
            println!("cargo:rerun-if-env-changed=OUTPUT_BASE");
            println!(
                "cargo:rerun-if-changed={output_base}/android/{nix_arch}/libanytype_engine.so",
            );
            let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap();
            let profile = std::env::var("PROFILE").unwrap();
            let dest_dir = format!(
                "{manifest_dir}/target/dx/{pkg_name}/{profile}/android/app/app/src/main/jniLibs/{jni_abi}",
            );
            std::fs::create_dir_all(&dest_dir).ok();
            let src = format!("{output_base}/android/{nix_arch}/libanytype_engine.so");
            let dst = format!("{dest_dir}/libanytype_engine.so");
            if let Err(e) = std::fs::copy(&src, &dst) {
                panic!("Failed to copy {src} to {dst}: {e}");
            }
            let ndk_home = std::env::var("ANDROID_NDK_HOME")
                .expect("ANDROID_NDK_HOME must be set");
            let host = if cfg!(target_os = "linux") {
                "linux-x86_64"
            } else if cfg!(target_os = "macos") {
                "darwin-x86_64"
            } else {
                "windows-x86_64"
            };
            let ndk_abi = match nix_arch {
                "aarch64" => "aarch64-linux-android",
                "armv7" => "arm-linux-androideabi",
                "x86_64" => "x86_64-linux-android",
                "i686" => "i686-linux-android",
                _ => return Ok(()),
            };
            let libcxx_src = format!(
                "{ndk_home}/toolchains/llvm/prebuilt/{host}/sysroot/usr/lib/{ndk_abi}/libc++_shared.so",
            );
            let libcxx_dst = format!("{dest_dir}/libc++_shared.so");
            if let Err(e) = std::fs::copy(&libcxx_src, &libcxx_dst) {
                panic!(
                    "Failed to copy libc++_shared.so from {libcxx_src} to {libcxx_dst}: {e}",
                );
            }
        }
        _ => {}
    }
    unsafe {
        std::env::set_var("PROTOC", protoc_bin_vendored::protoc_bin_path()?);
        println!("cargo:rerun-if-changed=protos");
    };
    tonic_prost_build::configure()
        .build_server(false)
        .build_client(false)
        .compile_protos(
            &[
                "protos/pkg/lib/pb/model/protos/models.proto",
                "protos/pkg/lib/pb/model/protos/localstore.proto",
            ],
            &["protos"],
        )?;
    tonic_prost_build::configure()
        .build_server(false)
        .build_client(true)
        .extern_path(".anytype.model", "crate::protos::anytype_model")
        .compile_protos(&["protos/pb/protos/service/service.proto"], &["protos"])?;
    Ok(())
}
