fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let profile = std::env::var("PROFILE")?;
    match target_os.as_str() {
        "windows" => {
            let engine_lib =
                std::env::var("WINDOWS_ENGINE_LIB").expect("WINDOWS_ENGINE_LIB not set");
            println!("cargo:rustc-link-arg={engine_lib}");
            println!("cargo:rerun-if-changed={engine_lib}");

            // dx serve --windows needs manual copying of the engine lib
            let src_dll = std::path::PathBuf::from(&engine_lib).with_extension("dll");

            if src_dll.exists() {
                let pkg_name = std::env::var("CARGO_PKG_NAME")?;
                let dx_app_dir = std::path::PathBuf::from(&manifest_dir)
                    .join("target/dx")
                    .join(&pkg_name)
                    .join(&profile)
                    .join("windows/app");
                std::fs::create_dir_all(&dx_app_dir).ok();
                std::fs::copy(&src_dll, dx_app_dir.join("lib_anytype_engine.dll")).ok();
            }
        }
        "linux" => {
            let engine_lib = std::env::var("LINUX_ENGINE_LIB").expect("LINUX_ENGINE_LIB not set");
            println!("cargo:rustc-link-arg={engine_lib}");
            println!("cargo:rerun-if-changed={engine_lib}");
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
            let engine_lib =
                std::env::var("ANDROID_ENGINE_LIB").expect("ANDROID_ENGINE_LIB not set");
            println!("cargo:rustc-link-arg={engine_lib}");
            println!("cargo:rerun-if-changed={engine_lib}");

            let dest_dir = std::path::PathBuf::from(&manifest_dir)
                .join("target/dx/any-task")
                .join(&profile)
                .join("android/app/app/src/main/jniLibs")
                .join(jni_abi);
            std::fs::create_dir_all(&dest_dir).expect("Failed to create jniLibs directory");
            let src = std::env::var("ANDROID_ENGINE_LIB").unwrap();
            let dst = dest_dir.join("lib_anytype_engine.so");
            std::fs::copy(&src, &dst).expect("Failed to copy engine lib");

            let ndk_home = std::env::var("ANDROID_NDK_HOME").expect("ANDROID_NDK_HOME must be set");
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
                "{ndk_home}/toolchains/llvm/prebuilt/{host}/sysroot/usr/lib/{ndk_abi}/libc++_shared.so"
            );
            let libcxx_dst = dest_dir.join("libc++_shared.so");
            std::fs::copy(&libcxx_src, &libcxx_dst).expect("Failed to copy libc++_shared.so");
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
