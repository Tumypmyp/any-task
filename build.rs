fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let engine_libs = std::env::var("ENGINE_LIBS")
        .unwrap_or_else(|_| format!("{manifest_dir}/go-engine/native-libs"));
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    match target_os.as_str() {
        "windows" => {
            println!("cargo:rustc-link-search=native={engine_libs}/windows");
            println!("cargo:rustc-link-lib=dylib=anytype_engine");
            println!("cargo:rerun-if-env-changed=ENGINE_LIBS");
            println!("cargo:rerun-if-changed={engine_libs}/windows/anytype_engine.dll",);
            // dx serve --windows needs manual copying of the engine lib
            let src_dll = std::path::PathBuf::from(&engine_libs)
                .join("windows")
                .join("anytype_engine.dll");

            if src_dll.exists() {
                let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap();
                let profile = std::env::var("PROFILE").unwrap();
                let dx_app_dir = std::path::PathBuf::from(&manifest_dir)
                    .join("target")
                    .join("dx")
                    .join(&pkg_name)
                    .join(&profile)
                    .join("windows")
                    .join("app");
                std::fs::create_dir_all(&dx_app_dir).ok();
                std::fs::copy(&src_dll, dx_app_dir.join("anytype_engine.dll")).ok();
            }
        }
        "linux" => {
            println!("cargo:rustc-link-search=native={engine_libs}/linux");
            println!("cargo:rustc-link-lib=dylib=anytype_engine");
            println!("cargo:rerun-if-env-changed=ENGINE_LIBS");
            println!("cargo:rerun-if-changed={engine_libs}/linux/anytype_engine.so");
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
            println!("cargo:rustc-link-search=native={engine_libs}/android/{nix_arch}");
            println!("cargo:rustc-link-lib=dylib=anytype_engine");
            println!("cargo:rerun-if-env-changed=ENGINE_LIBS");
            println!("cargo:rerun-if-changed={engine_libs}/android/{nix_arch}/anytype_engine.so",);
            // let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap();
            // let profile = std::env::var("PROFILE").unwrap();
            // let dest_dir = format!(
            //     "{manifest_dir}/target/dx/{pkg_name}/{profile}/android/app/app/src/main/jniLibs/{jni_abi}",
            // );
            // Change this line in your build.rs:
            // let dest_dir = format!("{manifest_dir}/app/src/main/jniLibs/{jni_abi}",);
            // 1. Define the exact Dioxus target directory
            let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
            let profile = std::env::var("PROFILE").unwrap();

            let dest_dir = format!(
                "{manifest_dir}/target/dx/any-task/{profile}/android/app/app/src/main/jniLibs/{jni_abi}"
            );
            // 2. Safely create directory and crash cleanly if it fails
            std::fs::create_dir_all(&dest_dir).expect(&format!(
                "Failed to create jniLibs directory at {}",
                dest_dir
            ));
            let src = format!("{engine_libs}/android/{nix_arch}/anytype_engine.so");
            let dst = format!("{dest_dir}/anytype_engine.so");
            std::fs::copy(&src, &dst).expect(&format!("Failed to copy {} to {}", src, dst));

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
            let libcxx_dst = format!("{dest_dir}/libc++_shared.so");
            std::fs::copy(&libcxx_src, &libcxx_dst).expect(&format!(
                "Failed to copy libc++_shared.so to {}",
                libcxx_dst
            ));
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
