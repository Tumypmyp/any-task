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
      rev = "f7e102a0b4868f51f35059ddacb19d78f10f0fa6";
      hash = "sha256-xDgXHRnye88voMJ4Fuw58gmu0N6U73K5QzniMF2+b00=";
    };
    buildAndTestSubdir = "packages/cli";
    cargoHash = "sha256-TC9NtCoJDG8tuRgwV9psBVKqz8OQFvMAKXkCHSTqXws=";

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
    platforms.version = [ "33" ];
  };

  languages.rust = {
    enable = true;
    channel = "stable";
    targets = [
      "wasm32-unknown-unknown"
      "aarch64-linux-android"
      "aarch64-apple-darwin"
      "armv7-linux-androideabi"
      "i686-linux-android"
      "i686-pc-windows-msvc"
      "x86_64-linux-android"
      "x86_64-unknown-linux-gnu"
      "x86_64-apple-darwin"
      "x86_64-pc-windows-msvc"
    ];
  };

  packages = [
    # dx
    dioxus-cli
    pkgs.glib
    pkgs.gdk-pixbuf
    pkgs.gtk3
    pkgs.xdotool
    pkgs.openssl
    pkgs.libsoup_3

    pkgs.webkitgtk_4_1

    # bundle windows
    # https://github.com/euphemism/dirtywave-updater-releases-mirror/blob/main/devenv.nix
    # pkgs.pkgsCross.mingwW64.stdenv.cc
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
          # pkgs.pkgsCross.mingwW64.windows.pthreads
          # pkgs.pkgsCross.mingwW64.libxcrypt
  ];
  # https://wiki.nixos.org/wiki/Tauri
# https://devenv.sh/processes/
  # processes.dev.exec = "${lib.getExe pkgs.watchexec} -n -- ls -la";

  # https://devenv.sh/services/
  # services.postgres.enable = true;

  # https://devenv.sh/basics/
  enterShell = ''
  '';

  # https://devenv.sh/tasks/
  # tasks = {
  #   "myproj:setup".exec = "mytool build";
  #   "devenv:enterShell".after = [ "myproj:setup" ];
  # };

  # https://devenv.sh/git-hooks/
  # git-hooks.hooks.shellcheck.enable = true;

  env.API_DIR = "${config.env.DEVENV_ROOT}/apis/2025-05-20";
  scripts.client-api-generate = {
    packages = [
      pkgs.openapi-generator-cli
    ];
    exec = ''
      openapi-generator-cli generate \
        -i "''${API_DIR}/openapi.yaml" \
        -g rust \
        -o "''${API_DIR}/openapi" \
        --skip-validate-spec
    '';
   };

   scripts.bundle-windows = {
    exec = ''
       dx bundle --windows --target x86_64-pc-windows-msvc
    '';
  };


  # env.TEMP_DIR = "${config.devenv.runtime}/bundle-android";
  env.TEMP_DIR = "${config.devenv.root}/.tmp";
  env = {
    APP_NAME = "AnyTask";
    OUTPUT_DIR = "${config.devenv.root}/dist/android";
    OUTPUT_AAB = "${config.env.TEMP_DIR}/AnyTask-aarch64-linux-android.aab";
    OUTPUT_APKS = "${config.env.OUTPUT_DIR}/${config.env.APP_NAME}-dev.apks";
    OUTPUT_APK = "${config.env.OUTPUT_DIR}/${config.env.APP_NAME}-universal.apk";
  };

  scripts.bundle-android-apk = {
    packages = [
      pkgs.bundletool
      pkgs.unzip
    ];
    exec = ''
      dx bundle --android --release --target  aarch64-linux-android --out-dir "$TEMP_DIR" || { echo "Failed to bundle AAB with dioxuss"; exit 1; }

      if [ -d "$OUTPUT_DIR" ]; then
          echo "Removing existing Android files: $OUTPUT_DIR"
          rm -rf "$OUTPUT_DIR"
      fi

      bundletool build-apks --bundle="$OUTPUT_AAB" --output="$OUTPUT_APKS" --mode=universal || { echo "Failed to build APKS."; exit 1; }
      unzip "$OUTPUT_APKS" -d "$TEMP_DIR"
      mv "$TEMP_DIR/universal.apk" "$OUTPUT_APK" || { echo "Failed to find universal.apk in APKS."; rm -rf "$TEMP_DIR"; exit 1; }
      rm -rf "$TEMP_DIR"
      echo "Universal APK extracted to $OUTPUT_APK"
    '';
  };
}
