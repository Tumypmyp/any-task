{
  pkgs,
  lib,
  config,
  ...
}:

let
  dioxus-cli = pkgs.rustPlatform.buildRustPackage {
    name = "dioxus-cli";
    src = pkgs.fetchFromGitHub {
      owner = "DioxusLabs";
      repo = "dioxus";
      # to update first set rev to the commit id, hash = ""; and cargoHash = "";
      rev = "a626d609c1995601215f780431d315cf7c063d1e";
      hash = "sha256-xjna/0dfucFM50lVcjZvCmIBkg/09KHOAKOBHvF1MEs=";
      # hash = "";
    };
    buildAndTestSubdir = "packages/cli";
    # cargoHash = "";
    cargoHash = "sha256-bSnadNN3j13k+rzeCqsiu97UD4LwNqOhx5pngUIfD08=";
    checkFlags = [
      "--skip=wasm_bindgen::test::test_cargo_install"
      "--skip=wasm_bindgen::test::test_github_install"
      "--skip=cli::autoformat::test_auto_fmt"
      "--skip=test_harnesses::run_harness"
    ];

    buildFeatures = [
      "no-downloads"
    ];

    OPENSSL_NO_VENDOR = 1;
    nativeBuildInputs = [
      pkgs.pkg-config
      pkgs.cacert
    ];
    buildInputs =
      with pkgs;
      [ openssl ]
      ++ lib.optionals stdenv.isDarwin [
        darwin.apple_sdk.frameworks.CoreServices
      ];
  };
in
{
  android = {
    enable = true;
    ndk = {
        enable = true;
        version = [ "29.0.14206865" ];
      };
      # buildTools.version = [ "36.0.0" ];
      buildTools.version = [ "34.0.0" ];

    platforms.version = [ "37" "36" "35" "34" ];
    abis = [ "x86_64" "arm64-v8a" ];
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

    # bundle windows
    # https://github.com/euphemism/dirtywave-updater-releases-mirror/blob/main/devenv.nix
    # pkgs.pkgsCross.mingwW64.stdenv
    # pkgs.pkgsCross.mingwW64.pkgsStatic.stdenv.targetPlatform.config
    # pkgs.pkgsCross.mingwW64.stdenv.targetPlatform.config
    # pkgs.pkgsCross.mingwW64.gcc

    # This is the correct way to reference the 64-bit compiler package
    # pkgs.mingwW64.x86_64-w64-mingw32-gcc

    # This is the correct way to reference the 32-bit compiler package (if you need it)
    # pkgs.mingwW64.i686-w64-mingw32-gcc

    # For any C dependencies (like OpenSSL or similar), you might also need this:
    # pkgs.mingwW64.pkg-config

    # pkgs.pkgsCross.mingwW64.stdenv
    # pkgs.pkgsCross.mingwW64.libxcrypt
  ];
  # https://wiki.nixos.org/wiki/Tauri
  # https://devenv.sh/processes/
  # processes.dev.exec = "${lib.getExe pkgs.watchexec} -n -- ls -la";

  # https://devenv.sh/basics/
  enterShell = '''';

  # https://devenv.sh/tasks/
  # tasks = {
  #   "myproj:setup".exec = "mytool build";
  #   "devenv:enterShell".after = [ "myproj:setup" ];
  # };

  # https://devenv.sh/git-hooks/
  # git-hooks.hooks.shellcheck.enable = true;

  env.API_DIR = "${config.env.DEVENV_ROOT}/apis/";
  scripts.client-api-generate = {
    packages = [
      pkgs.openapi-generator-cli
    ];
    exec = ''
      openapi-generator-cli generate \
        -i "''${API_DIR}/openapi-2025-11-08.yaml" \
        -g rust \
        -o "''${API_DIR}/openapi-2025-11-08" \
        --skip-validate-spec \
        --additional-properties=packageVersion=0.0.1
    '';
  };

  scripts.bundle-windows = {
    packages = [
        pkgs.pkgsCross.mingwW64.stdenv.cc
        pkgs.pkgsCross.mingwW64.windows.pthreads
      ];
    exec = ''
      export LIBRARY_PATH="$LIBRARY_PATH:${pkgs.pkgsCross.mingwW64.windows.pthreads}/lib"
      dx bundle --release --windows --target x86_64-pc-windows-gnu --features bundle
    '';
  };

  dotenv.enable = true;
  dotenv.filename = ".env.devenv";
  # env.TEMP_DIR = "${config.devenv.runtime}/bundle-android";
  env.TEMP_DIR = "${config.devenv.root}/TEMP_DIR";
  env = {
    APP_NAME = "AnyTask";
    OUTPUT_DIR = "${config.devenv.root}/dist/android";
    OUTPUT_AAB = "${config.env.TEMP_DIR}/AnyTask-aarch64-linux-android.aab";
    # OUTPUT_AAB = "${config.env.TEMP_DIR}/AnyTask-x86_64-linux-android.aab";
    OUTPUT_APKS = "${config.env.OUTPUT_DIR}/${config.env.APP_NAME}-dev.apks";
    OUTPUT_APK = "${config.env.OUTPUT_DIR}/${config.env.APP_NAME}-universal.apk";
    AAPT2 = "${pkgs.android-tools}/bin/aapt2";
  };
  scripts.create-emulator.exec = ''
      avdmanager create avd -n android-simple -k "system-images;android-34;google_apis_playstore;x86_64" --device "pixel_6_pro"
    '';
    scripts.run-android = {
      packages = [
        pkgs.bundletool
        pkgs.unzip
        pkgs.steam-run
      ];
      exec = ''
        dx serve --android --device
  '';};

  scripts.bundle-android = {
    packages = [
      pkgs.bundletool
      pkgs.unzip
      pkgs.steam-run
    ];
    exec = ''
      dx bundle --android --release --debug-symbols=false --target  aarch64-linux-android --out-dir "$TEMP_DIR" || { echo "Failed to bundle AAB with dioxus"; exit 1; }

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
  scripts.unzip-bundle-android = {
    packages = [
      pkgs.bundletool
      pkgs.unzip
      pkgs.steam-run
    ];
    exec = ''

      if [ -d "$OUTPUT_DIR" ]; then
          echo "Removing existing Android files: $OUTPUT_DIR"
          rm -rf "$OUTPUT_DIR"
      fi

      steam-run bundletool build-apks --bundle="$OUTPUT_AAB" --output="$OUTPUT_APKS" --mode=universal \
          --ks="$HOME/.keys/upload-keystore.jks" \
          --ks-pass=pass:"$KS_PASS" \
          --ks-key-alias=app-key \
          --overwrite \
          || { echo "Failed to build APKS."; exit 1; }

      unzip "$OUTPUT_APKS" -d "$TEMP_DIR"
      mv "$TEMP_DIR/universal.apk" "$OUTPUT_APK" || { echo "Failed to find universal.apk in APKS."; rm -rf "$TEMP_DIR"; exit 1; }
      rm -rf "$TEMP_DIR"
      echo "Universal APK extracted to $OUTPUT_APK"
    '';
  };
}
