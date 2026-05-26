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
            println!("cargo:rerun-if-changed={output_base}/windows/libanytype_engine.dll");
            let src_dll = std::path::PathBuf::from(&output_base)
                .join("windows")
                .join("anytype_engine.dll");

            if src_dll.exists() {
                // 1. Copy to the project manifest root directory
                if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
                    let p_root = std::path::PathBuf::from(manifest_dir);
                    let _ = std::fs::copy(&src_dll, p_root.join("anytype_engine.dll"));

                    // 2. Explicitly target Dioxus's custom development layout directory
                    let dx_windows_dir = p_root
                        .join("target")
                        .join("dx")
                        .join("any-task")
                        .join("debug")
                        .join("windows");

                    if !dx_windows_dir.exists() {
                        let _ = std::fs::create_dir_all(&dx_windows_dir);
                    }
                    let _ = std::fs::copy(&src_dll, dx_windows_dir.join("anytype_engine.dll"));
                }

                // 3. Copy to traditional cargo profile paths as a fallback
                if let Ok(out_dir) = std::env::var("OUT_DIR") {
                    let mut profile_dir = std::path::PathBuf::from(out_dir);
                    profile_dir.pop(); // pop "out"
                    profile_dir.pop(); // pop "any-task-[hash]"
                    profile_dir.pop(); // pop "build"

                    let _ = std::fs::copy(&src_dll, profile_dir.join("anytype_engine.dll"));
                    let _ = std::fs::copy(
                        &src_dll,
                        profile_dir.join("deps").join("anytype_engine.dll"),
                    );
                }
            } else {
                println!(
                    "cargo:warning=anytype_engine.dll was not found at the expected source path!"
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
            // Map Rust's target arch to (Nix output folder, Android JNI folder)
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

            // Link against the specific architecture folder
            println!("cargo:rustc-link-search=native={output_base}/android/{nix_arch}");
            println!("cargo:rustc-link-lib=dylib=anytype_engine");
            println!("cargo:rerun-if-env-changed=OUTPUT_BASE");
            println!(
                "cargo:rerun-if-changed={output_base}/android/{nix_arch}/libanytype_engine.so"
            );

            // Copy to jniLibs for APK bundling
            let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap();
            let profile = std::env::var("PROFILE").unwrap();
            let dest_dir = format!(
                "{manifest_dir}/target/dx/{pkg_name}/{profile}/android/app/app/src/main/jniLibs/{jni_abi}"
            );
            std::fs::create_dir_all(&dest_dir).ok();

            // Read from the specific architecture folder
            let src = format!("{output_base}/android/{nix_arch}/libanytype_engine.so");
            let dst = format!("{dest_dir}/libanytype_engine.so");

            if let Err(e) = std::fs::copy(&src, &dst) {
                panic!("Failed to copy {src} to {dst}: {e}");
            }
            let ndk_home = std::env::var("ANDROID_NDK_HOME").expect("ANDROID_NDK_HOME must be set");

            // Detect host OS for the prebuilt toolchain folder
            let host = if cfg!(target_os = "linux") {
                "linux-x86_64"
            } else if cfg!(target_os = "macos") {
                "darwin-x86_64"
            } else {
                "windows-x86_64"
            };

            // Map Rust arch to the NDK sysroot ABI folder name
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

            if let Err(e) = std::fs::copy(&libcxx_src, &libcxx_dst) {
                panic!("Failed to copy libc++_shared.so from {libcxx_src} to {libcxx_dst}: {e}");
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
        .build_client(true)
        .out_dir("src/protos")
        .extern_path(".anytype.model", "crate::protos::anytype_model")
        .compile_protos(&["protos/pb/protos/service/service.proto"], &["protos"])?;
    Ok(())
}

// fn main() {
//     let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
//     let output_base = std::env::var("OUTPUT_BASE")
//         .unwrap_or_else(|_| format!("{manifest_dir}/go-engine/native-libs"));
//     let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

//     match target_os.as_str() {
//         "linux" => {
//             println!("cargo:rustc-link-search=native={output_base}/linux");
//             println!("cargo:rustc-link-lib=dylib=anytype_engine");
//             println!("cargo:rerun-if-env-changed=OUTPUT_BASE");
//             println!("cargo:rerun-if-changed={output_base}/linux/libanytype_engine.so");
//         }

//         "android" => {
//             let abi = match std::env::var("CARGO_CFG_TARGET_ARCH")
//                 .unwrap_or_default()
//                 .as_str()
//             {
//                 "aarch64" => "arm64-v8a",
//                 "arm" => "armeabi-v7a",
//                 "x86_64" => "x86_64",
//                 "x86" => "x86",
//                 _ => return,
//             };

//             println!("cargo:rustc-link-search=native={output_base}/android");
//             println!("cargo:rustc-link-lib=dylib=anytype_engine");
//             println!("cargo:rerun-if-env-changed=OUTPUT_BASE");
//             println!("cargo:rerun-if-changed={output_base}/android/libanytype_engine.so");

//             // Copy to jniLibs for APK bundling
//             let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap();
//             let profile = std::env::var("PROFILE").unwrap();
//             let dest_dir = format!(
//                 "{manifest_dir}/target/dx/{pkg_name}/{profile}/android/app/app/src/main/jniLibs/{abi}"
//             );
//             std::fs::create_dir_all(&dest_dir).ok();
//             let src = format!("{output_base}/android/libanytype_engine.so");
//             let dst = format!("{dest_dir}/libanytype_engine.so");
//             if let Err(e) = std::fs::copy(&src, &dst) {
//                 eprintln!("Warning: Failed to copy {src} to {dst}: {e}");
//             }
//         }

//         _ => {}
//     }
// }
// fn main() {
//     let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
//     let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

//     match target_os.as_str() {
//         "android" => {
//             let abi = match std::env::var("CARGO_CFG_TARGET_ARCH")
//                 .unwrap_or_default()
//                 .as_str()
//             {
//                 "aarch64" => "arm64-v8a",
//                 "arm" => "armeabi-v7a",
//                 "x86" => "x86",
//                 "x86_64" => "x86_64",
//                 _ => return,
//             };

//             println!("cargo:rustc-link-search=native={manifest_dir}/go-engine/native-libs/{abi}");
//             println!("cargo:rustc-link-lib=dylib=anytypeheart");

//             // Copy to jniLibs for APK bundling
//             let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap();
//             let profile = std::env::var("PROFILE").unwrap();
//             let dest_dir = format!(
//                 "{manifest_dir}/target/dx/{pkg_name}/{profile}/android/app/app/src/main/jniLibs/{abi}"
//             );
//             std::fs::create_dir_all(&dest_dir).ok();
//             let src = format!("{manifest_dir}/go-engine/native-libs/{abi}/libanytypeheart.so");
//             let dst = format!("{dest_dir}/libanytypeheart.so");
//             if let Err(e) = std::fs::copy(&src, &dst) {
//                 eprintln!("Warning: Failed to copy {src} to {dst}: {e}");
//             }
//         }

//         "linux" => {
//             let output_base =
//                 std::env::var("OUTPUT_BASE").unwrap_or_else(|_| format!("{manifest_dir}/targets"));
//             println!("cargo:rustc-link-search=native={output_base}/linux");
//             println!("cargo:rustc-link-lib=dylib=anytype_engine");
//             println!("cargo:rerun-if-env-changed=OUTPUT_BASE");
//         }

//         _ => {}
//     }
// }
// // fn main() {
// //     // Only run for Android builds
// //     if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("android") {
// //         return;
// //     }

// //     let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
// //     let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap();
// //     let profile = std::env::var("PROFILE").unwrap(); // "debug" or "release"
// //     let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();

// //     // Map Rust architecture to Android ABI
// //     let abi = "android";
// //     //     match arch.as_str() {
// //     //     "aarch64" => "arm64-v8a",
// //     //     "arm" => "armeabi-v7a",
// //     //     "x86" => "x86",
// //     //     "x86_64" => "x86_64",
// //     //     _ => return,
// //     // };

// //     // Source .so file in your project
// //     let src_so = format!("{manifest_dir}/go-engine/native-libs/{abi}/libanytypeheart.so");

// //     // Tell the linker where to find the library (for linking)
// //     println!("cargo:rustc-link-search=native={manifest_dir}/go-engine/native-libs/{abi}");

// //     // Tell the linker to link against the library
// //     println!("cargo:rustc-link-lib=dylib=anytypeheart");

// //     // Copy to jniLibs staging directory (for APK bundling)
// //     let dest_dir = format!(
// //         "{manifest_dir}/target/dx/{pkg_name}/{profile}/android/app/app/src/main/jniLibs/{abi}"
// //     );

// //     let dest_so = format!("{dest_dir}/libanytypeheart.so");

// //     // Create destination directory if it doesn't exist
// //     std::fs::create_dir_all(&dest_dir).ok();

// //     // Copy the .so file
// //     if let Err(e) = std::fs::copy(&src_so, &dest_so) {
// //         eprintln!("Warning: Failed to copy {src_so} to {dest_so}: {e}");
// //     }
// // }
// // fn main() {
// //     if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("android") {
// //         let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap();
// //         // let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
// //         // let abi = match arch.as_str() {
// //         //     "aarch64" => "arm64-v8a",
// //         //     "x86_64" => "x86_64",
// //         //     _ => return,
// //         // };
// //         // Tell the linker where the .so lives
// //         // println!("cargo:rustc-link-search=native={manifest}/engine-go/native-libs/{abi}");
// //         println!("cargo:rustc-link-search=native={manifest}/engine-go/native-libs/android");
// //         // Link against it (assumes the file is named libanytypeheart.so)
// //         println!("cargo:rustc-link-lib=dylib=libanytype_engine");
// //     }
// // }

// // fn main() {
// //     if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("android") {
// //         let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap();
// //         let pkg = std::env::var("CARGO_PKG_NAME").unwrap();
// //         let profile = std::env::var("PROFILE").unwrap(); // "debug" or "release"

// //         for abi in ["arm64-v8a", "x86_64"] {
// //             let dest = format!(
// //                 "{manifest}/target/dx/{pkg}/{profile}/android/app/app/src/main/jniLibs/{abi}"
// //             );
// //             std::fs::create_dir_all(&dest).ok();
// //             std::fs::copy(
// //                 format!("{manifest}/native-libs/{abi}/libcustom.so"),
// //                 format!("{dest}/libcustom.so"),
// //             )
// //             .ok();
// //         }
// //     }
// // }
