{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";

    dioxus.url = "github:DioxusLabs/dioxus/main";
    dioxus.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, flake-parts, dioxus, ... } @inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "x86_64-darwin" "aarch64-darwin" "aarch64-linux" ];

      perSystem = { self', config, pkgs, lib, system, ... }: let
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" ];
          targets = [
            "wasm32-unknown-unknown" 
            "aarch64-linux-android"
            "armv7-linux-androideabi"
            "i686-linux-android"
            "x86_64-linux-android"
          ];
        };
 
        # packages.emulateAndroid = pkgs.androidenv.emulateApp {
        #   name = "emulateAndroid";
        #   platformVersion = "33"; # Use a recent API level, like 34 for Pixel 6
        #   abiVersion = "arm64-v8a";
        #   systemImageType = "default";
        #   # You can also automatically deploy your app once it's built
        #   # app = self.packages.${system}.default;
        #   # package = "com.yourcompany.yourapp";
        #   # activity = "MainActivity";
        # };     
        #
        buildToolsVersion = "34.0.0";
        androidComposition = pkgs.androidenv.composeAndroidPackages {
          includeEmulator = true;
          abiVersions = [ "armeabi-v7a" "arm64-v8a" ];
          includeNDK = true;
          buildToolsVersions = [ buildToolsVersion ];
          platformVersions = [ "33" ];
         # accept_license = true;
        };
        # androidEnv = pkgs.androidenv.androidPkgs.androidsdk;
        rustBuildInputs = (with pkgs; [ openssl libiconv pkg-config jdk ])
          ++ lib.optionals pkgs.stdenv.isLinux (with pkgs; [
            glib gtk3 libsoup_3 webkitgtk_4_1 xdotool
            androidComposition.androidsdk
          ])
          ++ lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk.frameworks; [
            SystemConfiguration IOKit Carbon WebKit Security Cocoa
          ]);
      in
      {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [ 
            inputs.rust-overlay.overlays.default 
          ];
          config = {
           android_sdk.accept_license = true;
           allowUnfree = true;
          };
        };

        # Inside the `perSystem` block, next to `packages.default`
        # packages.emulateAndroid = pkgs.androidenv.emulateApp {
        #   name = "emulateAndroid";
        #   platformVersion = "34"; # Use a recent API level, like 34 for Pixel 6
        #   abiVersion = "arm64-v8a";
        #   systemImageType = "google_apis_playstore";
        #   # You can also automatically deploy your app once it's built
        #   app = self.packages.${system}.default;
        #   # package = "com.yourcompany.yourapp";
        #   # activity = "MainActivity";
        # };
        formatter = pkgs.nixfmt-rfc-style;
        # packages.default = let
        #   cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        #   rev = toString (self.shortRev or self.dirtyShortRev or self.lastModified or "unknown");
        # in
        # pkgs.rustPlatform.buildRustPackage {
        #   pname = cargoToml.package.name;
        #   version = "${cargoToml.package.version}-${rev}";
        #   src = ./.;
        #   strictDeps = true;
        #   buildInputs = rustBuildInputs;
        #   nativeBuildInputs = with pkgs; [
        #     dioxus.packages.${system}.dioxus-cli
        #     rustToolchain
        #     rustPlatform.bindgenHook
        #     wasm-bindgen-cli_0_2_100
        #     binaryen
        #   ] ++ rustBuildInputs;
        #   buildPhase = ''
        #     dx build --release --platform web
        #   '';
        #   installPhase = ''
        #     mkdir -p $out
        #     cp -r target/dx/$pname/release/web $out/bin
        #   '';
        #   cargoLock.lockFile = ./Cargo.lock;
        #   meta.mainProgram = "server";
        # };

        devShells.default = pkgs.mkShell rec {
          name = "dioxus-dev";
          buildInputs = rustBuildInputs ++ [
#            self.packages.${system}.emulateAndroid
          ];
          nativeBuildInputs = with pkgs; [
            rustToolchain
            wasm-bindgen-cli_0_2_100
            dioxus.packages.${system}.dioxus-cli
            binaryen
            # packages.emulateAndroid
          ];

          ANDROID_SDK_ROOT = "${androidComposition.androidsdk}/libexec/android-sdk";
          ANDROID_NDK_ROOT = "${ANDROID_SDK_ROOT}/ndk-bundle";
          GRADLE_OPTS = "-Dorg.gradle.project.android.aapt2FromMavenOverride=${ANDROID_SDK_ROOT}/build-tools/${buildToolsVersion}/aapt2";

          RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library";
          JAVA_HOME="${pkgs.jdk17}";

          shellHook = ''
          '';
        };
      };
    };
}
