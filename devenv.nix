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
    buildInputs = [pkgs.openssl];
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
    # android
    pkgs.bundletool
    pkgs.unzip
    pkgs.steam-run

    # windows
  ];

  tasks."proto:download" = {
    env = {
      BASE = "https://raw.githubusercontent.com/anyproto/anytype-heart/main";
      DEST = "./protos";
    };
    package = pkgs.nushell;
    binary = "nu";
    exec = ''
      mkdir $"($env.DEST)/pb/protos/service"
      mkdir $"($env.DEST)/pb/protos"
      mkdir $"($env.DEST)/pkg/lib/pb/model/protos"

      let files = [
          ["path",                                        "dest"],
          ["pb/protos/service/service.proto",             $"($env.DEST)/pb/protos/service/service.proto"],
          ["pb/protos/commands.proto",                    $"($env.DEST)/pb/protos/commands.proto"],
          ["pb/protos/events.proto",                      $"($env.DEST)/pb/protos/events.proto"],
          ["pkg/lib/pb/model/protos/models.proto",        $"($env.DEST)/pkg/lib/pb/model/protos/models.proto"],
          ["pkg/lib/pb/model/protos/localstore.proto",    $"($env.DEST)/pkg/lib/pb/model/protos/localstore.proto"],
      ]

      for row in $files {
          let url = $"($env.BASE)/($row.path)"
          print $"Downloading ($row.path)..."
          http get $url | save -f $row.dest
      }
    '';
  };

  dotenv.enable = true;
  dotenv.filename = ".env.devenv";

  env = {
    ANDROID_NDK_HOME = config.env.ANDROID_NDK_ROOT;
    TEMP_DIR = "${config.devenv.root}/.devenv/bundle-temp";
    APP_NAME = "AnyTask";

    ANDROID_AAB = "${config.env.TEMP_DIR}/${config.env.APP_NAME}.aab";
    ANDROID_APKS = "${config.env.TEMP_DIR}/${config.env.APP_NAME}.apks";

    ANDROID_APK_DIR = "${config.devenv.root}/dist/android";
    ANDROID_APK = "${config.env.ANDROID_APK_DIR}/${config.env.APP_NAME}.apk";
    JNI_DIR = "target/dx/any-task/release/android/app/app/src/main/jniLibs/arm64-v8a";
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
    package = pkgs.nushell;
    binary = "nu";
    exec = ''
      dx bundle --android --release --debug-symbols=false --target aarch64-linux-android

      print "Re-injecting native libraries into the generated Android project..."
      mkdir $env.JNI_DIR
      cp "go-engine/native-libs/android/aarch64/lib_anytype_engine.so" $env.JNI_DIR
      cp $"($env.ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/aarch64-linux-android/libc++_shared.so" $env.JNI_DIR

      print "Building AAB via Gradle..."
      do {
        cd "target/dx/any-task/release/android/app"
        try {
          ^./gradlew bundleRelease
        } catch {
          error make "Gradle build failed."
        }
      }

      rm -rf $env.TEMP_DIR
      mkdir $env.TEMP_DIR

      cp "target/dx/any-task/release/android/app/app/build/outputs/bundle/release/app-release.aab" $env.ANDROID_AAB

      print "Building APKS from AAB using bundletool..."
      try {
        (steam-run bundletool build-apks
          --bundle $env.ANDROID_AAB
          --output $env.ANDROID_APKS
          --mode universal
          --ks $"($env.HOME)/.keys/upload-keystore.jks"
          --ks-pass $"pass:($env.KS_PASS)"
          --ks-key-alias upload
          --overwrite
        )
      } catch {
        error make "Failed to build APKS."
      }

      print "Extracting Universal APK..."
      mkdir ($env.ANDROID_APK | path dirname)
      ^unzip -p $env.ANDROID_APKS universal.apk | save -f $env.ANDROID_APK

      print $"Universal APK successfully extracted to ($env.ANDROID_APK)"
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
