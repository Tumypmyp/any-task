{
  pkgs,
  lib,
  config,
  inputs,
  ...
}: let
  dioxus-src = pkgs.fetchFromGitHub {
    owner = "DioxusLabs";
    repo = "dioxus";
    rev = "v0.7.9";
    hash = "sha256-4XkV/gkytFefvliaK/JMERFqWIp6LycM/O/Dm294xS4=";
  };
  dioxus-cli = pkgs.rustPlatform.buildRustPackage {
    name = "dioxus-cli";
    src = dioxus-src;
    buildAndTestSubdir = "packages/cli";
    cargoLock.lockFile = "${dioxus-src}/Cargo.lock";
    buildFeatures = ["no-downloads"];
    doCheck = false;
    OPENSSL_NO_VENDOR = 1;
    nativeBuildInputs = [pkgs.pkg-config pkgs.cacert];
    buildInputs = with pkgs;
      [openssl]
      ++ lib.optionals stdenv.isDarwin [darwin.apple_sdk.frameworks.CoreServices];
  };
in {
  android = {
    enable = true;
    ndk = {
      enable = true;
      version = ["29.0.14206865"];
    };
    buildTools.version = ["34.0.0"];
    platforms.version = ["34"];
    abis = ["x86_64" "arm64-v8a"];
  };
  languages.rust = {
    enable = true;
    channel = "nightly";
    targets = [
      "wasm32-unknown-unknown"
      "aarch64-linux-android"
      "aarch64-apple-darwin"
      "armv7-linux-androideabi"
      "i686-linux-android"
      "i686-pc-windows-msvc"
      "x86_64-unknown-linux-gnu"
      "x86_64-linux-android"
      "x86_64-apple-darwin"
      "x86_64-pc-windows-msvc"
      "x86_64-pc-windows-gnu"
    ];
  };

  languages.go = {
    enable = true;
  };
  packages = [
    dioxus-cli
    pkgs.glib
    pkgs.gdk-pixbuf
    pkgs.gtk3
    pkgs.xdotool
    pkgs.openssl
    pkgs.libsoup_3
    pkgs.webkitgtk_4_1
    pkgs.openjdk
    pkgs.protobuf

    pkgs.pkgsCross.mingwW64.stdenv.cc
    #   pkgs.pkgsCross.mingwW64.windows.pthreads
    #         pkgs.bundletool
    pkgs.bundletool
    pkgs.unzip
    pkgs.steam-run
  ];
  # https://wiki.nixos.org/wiki/Tauri
  # https://devenv.sh/processes/
  # processes.dev.exec = "${lib.getExe pkgs.watchexec} -n -- ls -la";

  # https://devenv.sh/basics/
  enterShell = '''';

  tasks."proto:download" = {
    env = {
      BASE = "https://raw.githubusercontent.com/anyproto/anytype-heart/main";
      DEST = "./protos";
    };
    exec = ''
      mkdir -p $DEST/pb/protos/service
      mkdir -p $DEST/pb/protos
      mkdir -p $DEST/pkg/lib/pb/model/protos

      curl -sL $BASE/pb/protos/service/service.proto   -o $DEST/pb/protos/service/service.proto
      curl -sL $BASE/pb/protos/commands.proto          -o $DEST/pb/protos/commands.proto
      curl -sL $BASE/pb/protos/events.proto            -o $DEST/pb/protos/events.proto
      curl -sL $BASE/pkg/lib/pb/model/protos/models.proto            -o $DEST/pkg/lib/pb/model/protos/models.proto
      curl -sL $BASE/pkg/lib/pb/model/protos/localstore.proto -o $DEST/pkg/lib/pb/model/protos/localstore.proto
    '';
  };

  # tasks."bundle:windows" = {
  #   # packages = [
  #   #   pkgs.pkgsCross.mingwW64.stdenv.cc
  #   #   pkgs.pkgsCross.mingwW64.windows.pthreads
  #   # ];
  #   exec = ''
  #     # export CC="zig cc -target x86_64-windows-gnu"
  #     # export CXX="zig c++ -target x86_64-windows-gnu"

  #     # # export LIBRARY_PATH="$LIBRARY_PATH:${pkgs.pkgsCross.mingwW64.windows.pthreads}/lib"
  #     # dx bundle --release --windows --target x86_64-pc-windows-gnu --features bundle
  #     export CC_x86_64_pc_windows_gnu="x86_64-w64-mingw32-gcc"
  #         export CXX_x86_64_pc_windows_gnu="x86_64-w64-mingw32-g++"
  #         export AR_x86_64_pc_windows_gnu="x86_64-w64-mingw32-ar"

  #         # Tell Cargo to use the MinGW linker for the final executable
  #         export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER="x86_64-w64-mingw32-gcc"

  #         dx bundle --release --windows --target x86_64-pc-windows-gnu --features bundle
  #   '';
  # };

  dotenv.enable = true;
  dotenv.filename = ".env.devenv";
  # env.TEMP_DIR = "${config.devenv.runtime}/bundle-android";
  env.TEMP_DIR = "${config.devenv.root}/TEMP_DIR";
  env = {
    ANDROID_NDK_HOME = config.env.ANDROID_NDK_ROOT;
    APP_NAME = "AnyTask";
    OUTPUT_DIR = "${config.devenv.root}/dist/android";
    OUTPUT_AAB = "${config.env.TEMP_DIR}/AnyTask-aarch64-linux-android.aab";
    # OUTPUT_AAB = "${config.devenv.root}/dist/android-abb/AnyTask-x86_64-linux-android.aab";
    OUTPUT_APKS = "${config.env.OUTPUT_DIR}/${config.env.APP_NAME}-dev.apks";
    OUTPUT_APK = "${config.env.OUTPUT_DIR}/${config.env.APP_NAME}-universal.apk";
    AAPT2 = "${pkgs.android-tools}/bin/aapt2";
  };
  # scripts.create-emulator.exec = ''
  #   avdmanager create avd -n android-simple -k "system-images;android-34;google_apis_playstore;x86_64" --device "pixel_6_pro"
  # '';
  # scripts.run-android = {
  #  exec = ''
  #     dx serve --android --device
  #   '';
  # };
  tasks."bundle:android" = {
    exec = ''
      # Clean up
      rm -rf "$TEMP_DIR"

      # 1. Let dx generate the project and compile Rust
      # This generates an initial AAB, but without our .so files
      echo "Running dx bundle (allowing failure)..."
      dx bundle --android --release --debug-symbols=false --target aarch64-linux-android --out-dir "$TEMP_DIR" || true

      # 2. Re-copy .so files into the generated project
      echo "Re-injecting native libraries into the generated Android project..."
      JNI_DIR="target/dx/any-task/release/android/app/app/src/main/jniLibs/arm64-v8a"
      mkdir -p "$JNI_DIR"

      cp "go-engine/native-libs/android/aarch64/libanytype_engine.so" "$JNI_DIR/"

      # Fallback to NDK_HOME if ANDROID_NDK_HOME is not set
      NDK_PATH="''${ANDROID_NDK_HOME:-$NDK_HOME}"
      if [ -z "$NDK_PATH" ]; then
          echo "Error: ANDROID_NDK_HOME or NDK_HOME is not set in your environment."
          exit 1
      fi
      cp "$NDK_PATH/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/aarch64-linux-android/libc++_shared.so" "$JNI_DIR/"

      # 3. Run Gradle manually from the CORRECT root directory
      echo "Building AAB via Gradle..."
      pushd target/dx/any-task/release/android/app
      ./gradlew bundleRelease || { echo "Gradle build failed."; popd; exit 1; }
      popd

      # Move the newly generated AAB to the expected TEMP_DIR output location
      mkdir -p "$TEMP_DIR"
      cp target/dx/any-task/release/android/app/app/build/outputs/bundle/release/app-release.aab "$OUTPUT_AAB"

      # 4. Process the AAB into a Universal APK using bundletool
      if [ -d "$OUTPUT_DIR" ]; then
          echo "Removing existing Android files: $OUTPUT_DIR"
          rm -rf "$OUTPUT_DIR"
      fi
      mkdir -p "$OUTPUT_DIR"

      echo "Building APKS from AAB using bundletool..."
      steam-run bundletool build-apks --bundle="$OUTPUT_AAB" --output="$OUTPUT_APKS" --mode=universal \
          --ks="$HOME/.keys/upload-keystore.jks" \
          --ks-pass=pass:"$KS_PASS" \
          --ks-key-alias=upload \
          --overwrite \
          || { echo "Failed to build APKS."; exit 1; }

      echo "Extracting Universal APK..."
      unzip -q "$OUTPUT_APKS" -d "$TEMP_DIR"
      mv "$TEMP_DIR/universal.apk" "$OUTPUT_APK" || { echo "Failed to find universal.apk in APKS."; rm -rf "$TEMP_DIR"; exit 1; }

      # Clean up
      rm -rf "$TEMP_DIR"
      echo "Universal APK successfully extracted to $OUTPUT_APK"
    '';
  };
  tasks."bundle:unzip-android" = {
    exec = ''
      if [ -d "$OUTPUT_DIR" ]; then
          echo "Removing existing Android files: $OUTPUT_DIR"
          rm -rf "$OUTPUT_DIR"
      fi

      steam-run bundletool build-apks --bundle="$OUTPUT_AAB" --output="$OUTPUT_APKS" --mode=universal \
          --ks="$HOME/.keys/upload-keystore.jks" \
          --ks-pass=pass:"$KS_PASS" \
          --ks-key-alias=upload \
          --overwrite \
          || { echo "Failed to build APKS."; exit 1; }

      unzip "$OUTPUT_APKS" -d "$TEMP_DIR"
      mv "$TEMP_DIR/universal.apk" "$OUTPUT_APK" || { echo "Failed to find universal.apk in APKS."; rm -rf "$TEMP_DIR"; exit 1; }
      rm -rf "$TEMP_DIR"
      echo "Universal APK extracted to $OUTPUT_APK"
    '';
  };

  git-hooks.hooks = {
    alejandra.enable = true;
    prettier = {
      enable = true;
      files = "\\.css$";
    };
    dx-fmt = {
      enable = true;
      name = "Dioxus Format";
      entry = "dx fmt";
      files = "\\.rs$";
      pass_filenames = false;
    };
  };
}
