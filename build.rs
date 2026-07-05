fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let profile = std::env::var("PROFILE")?;

    let raw_engine_lib = match target_os.as_str() {
        "windows" => std::env::var("WINDOWS_ENGINE_LIB"),
        "linux" => std::env::var("LINUX_ENGINE_LIB"),
        "android" => std::env::var("ANDROID_ENGINE_LIB"),
        _ => return Err(format!("Unsupported target OS: {target_os}").into()),
    }?;

    // Normalize the mixed slashes from the justfile immediately
    let engine_lib = raw_engine_lib.replace('\\', "/");

    // Now engine_path will correctly parse the components on all host OS environments
    let engine_path = std::path::Path::new(&engine_lib);

    // Automatically extract and pass the directory to the linker search path
    if let Some(search_dir) = engine_path.parent() {
        let search_dir_normalized = search_dir.to_string_lossy().replace('\\', "/");
        println!("cargo:rustc-link-search=native={search_dir_normalized}");
    }
    // Automatically calculate the correct linker name based on OS rules
    if let Some(file_stem) = engine_path.file_stem().map(|s| s.to_string_lossy()) {
        let link_name = if target_os == "windows" {
            // Windows linkers look for the exact name (e.g., "lib_anytype_engine")
            file_stem.to_string()
        } else {
            // Unix linkers (Linux/Android) automatically prepend "lib",
            // so we strip it out (e.g., "lib_anytype_engine" -> "_anytype_engine")
            if file_stem.starts_with("lib") {
                file_stem["lib".len()..].to_string()
            } else {
                file_stem.to_string()
            }
        };
        println!("cargo:rustc-link-lib=dylib={link_name}");
    }
    println!("cargo:rerun-if-changed={engine_lib}");

    match target_os.as_str() {
        "windows" => {
            // Windows needs the companion .dll copied next to the executable
            let src_dll = engine_path.with_extension("dll");

            if src_dll.exists() {
                let pkg_name = std::env::var("CARGO_PKG_NAME")?;
                let dll_name = src_dll.file_name().unwrap();

                // 1. Copy to standard Cargo output (Required for `dx serve` and `cargo run`)
                let cargo_out_dir = std::path::PathBuf::from(&manifest_dir)
                    .join("target")
                    .join(&profile);
                std::fs::create_dir_all(&cargo_out_dir).ok();
                std::fs::copy(&src_dll, cargo_out_dir.join(dll_name)).ok();

                // 2. Copy to Dioxus bundle output (Required for `dx bundle`)
                let dx_app_dir = std::path::PathBuf::from(&manifest_dir)
                    .join("target/dx")
                    .join(&pkg_name)
                    .join(&profile)
                    .join("windows/app");
                std::fs::create_dir_all(&dx_app_dir).ok();
                std::fs::copy(&src_dll, dx_app_dir.join(dll_name)).ok();
            } else {
                // Add a warning so you see it in the terminal if the source DLL goes missing
                println!(
                    "cargo:warning=Companion DLL not found at {}",
                    src_dll.display()
                );
            }
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

            let dest_dir = std::path::PathBuf::from(&manifest_dir)
                .join("target/dx/any-task")
                .join(&profile)
                .join("android/app/app/src/main/jniLibs")
                .join(jni_abi);
            std::fs::create_dir_all(&dest_dir).expect("Failed to create jniLibs directory");

            // Copy the core engine
            let dst = dest_dir.join("lib_anytype_engine.so");
            std::fs::copy(engine_path, &dst).expect("Failed to copy engine lib");

            // Copy the NDK libc++ helper
            let ndk_home = std::env::var("ANDROID_NDK_HOME").expect("ANDROID_NDK_HOME must be set");
            let host = if cfg!(target_os = "linux") {
                "linux-x86_64"
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
                "{ndk_home}/toolchains/llvm/prebuilt/{host}/sysroot/usr/lib/{ndk_abi}/libc++_shared.so"
            );
            let libcxx_dst = dest_dir.join("libc++_shared.so");
            std::fs::copy(&libcxx_src, &libcxx_dst).expect("Failed to copy libc++_shared.so");
        }
        "linux" => {}
        _ => {}
    }
    unsafe {
        std::env::set_var("PROTOC", protoc_bin_vendored::protoc_bin_path()?);
        println!("cargo:rerun-if-changed=protos");
    };
    tonic_prost_build::configure()
        .build_server(false)
        .build_client(false)
        .enum_attribute(
            "anytype.model.RelationFormat",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
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
