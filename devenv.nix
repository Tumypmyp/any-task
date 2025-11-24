{
  pkgs,
  lib,
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
  appName = "any-task";
  outputDir = "dist/android";
  # aabOutputPath = "target/dx/any-task/release/android/app/app/build/outputs/bundle/release/AnyTask-x86_64-linux-android.aab";
  aabOutputPath = "target/dx/any-task/release/android/app/app/build/outputs/bundle/release/AnyTask-aarch64-linux-android.aab";
  # aabOutputPath = "target/dx/${appName}/release/android/app/app/build/outputs/bundle/release/${appName}-release.aab";
  outputApksPath = "${outputDir}/${appName}-dev.apks";
  outputApkPath = "${outputDir}/${appName}-universal.apk";
in
{
  # --- 3. Expose the variables to the Shell Environment (env) ---
  env.APP_NAME = appName;
  env.AAB_OUTPUT = aabOutputPath;
  env.OUTPUT_APKS = outputApksPath;
  env.OUTPUT_APK = outputApkPath;
  env.OUTPUT_DIR = outputDir; # Needed if you still want to use it in scripts
  env.TEMP_DIR = ".tmp";
  env.GREET = "devenv";

  android = {
    enable = true;
    platforms.version = [ "33" ];
  };
  # https://devenv.sh/packages/
  packages = [
    pkgs.git
    pkgs.fish

    # dx
    dioxus-cli
    pkgs.glib
    pkgs.gdk-pixbuf
    pkgs.xdotool

    # scripts
    pkgs.bundletool
    pkgs.unzip

    pkgs.sozu
  ];
  # https://wiki.nixos.org/wiki/Tauri

  languages.rust = {
    enable = true;
    channel = "stable";
    targets = [
      "wasm32-unknown-unknown" # WebAssembly (for Dioxus web, etc.)
      "aarch64-linux-android" # 64-bit Android ARM
      "aarch64-apple-darwin" # Apple Silicon (M1/M2)
      "armv7-linux-androideabi" # 32-bit Android ARM (compatibility)
      "i686-linux-android" # Android x86
      "x86_64-linux-android" # Android x86_64
      "x86_64-unknown-linux-gnu" # Standard Linux (already likely included, but good to ensure)
      "x86_64-apple-darwin"
    ];
  };
  # https://devenv.sh/processes/
  # processes.dev.exec = "${lib.getExe pkgs.watchexec} -n -- ls -la";

  # https://devenv.sh/services/
  # services.postgres.enable = true;

  # https://devenv.sh/scripts/
  scripts.hello.exec = ''
    echo hello from $GREET
  '';

  # https://devenv.sh/basics/
  enterShell = ''
    hello         # Run scripts directly
    git --version # Use packages

  '';

  # https://devenv.sh/tasks/
  # tasks = {
  #   "myproj:setup".exec = "mytool build";
  #   "devenv:enterShell".after = [ "myproj:setup" ];
  # };

  # https://devenv.sh/git-hooks/
  # git-hooks.hooks.shellcheck.enable = true;

  # See full reference at https://devenv.sh/reference/options/
  # env.APP_NAME = "AnyTask";
  # env.ABB_OUTPUR = "/mnt/c/Users/Timur/Code/github/tumypmyp/any/target/dx/any-task/release/android/app/app/build/outputs/bundle/release/AnyTask-x86_64-linux-android.aab";
  # env.OUTPUT_APKS = "${env.PROJECT_ROOT}/${env.OUTPUT_DIR}/${env.APP_NAME}-dev.apks";
  # env.OUTPUT_APK = "$PROJECT_ROOT/$OUTPUT_DIR/$APP_NAME-univeral.apk";
  scripts.bundle-android-apk = {
    packages = [
      pkgs.bundletool
      pkgs.unzip
    ];
    exec = ''
      # dx build android
      export APKS_PATH="''${DEVENV_ROOT}/$OUTPUT_APKS"
      # Remove the output APKS file if it exists
      if [ -f "$APKS_PATH" ]; then
          echo "Removing existing APKS file: $APKS_PATH"
          rm "$APKS_PATH"
      fi
      bundletool build-apks --bundle="''${DEVENV_ROOT}/$AAB_OUTPUT" --output="''${DEVENV_ROOT}/$OUTPUT_APKS" --mode=universal || { echo "Failed to build APKS."; exit 1; }
      mkdir -p "''${DEVENV_ROOT}/TEMP_DIR"
      unzip "''${DEVENV_ROOT}/$OUTPUT_APKS" -d "''${DEVENV_ROOT}/$TEMP_DIR"
      mv "''${DEVENV_ROOT}/$TEMP_DIR/universal.apk" "''${DEVENV_ROOT}/$OUTPUT_APK" || { echo "Failed to find universal.apk in APKS."; rm -rf "''${DEVENV_ROOT}/$TEMP_DIR"; exit 1; }
      rm -rf "''${DEVENV_ROOT}/$TEMP_DIR"
      echo "Universal APK extracted to $OUTPUT_APK"
    '';
  };
}
