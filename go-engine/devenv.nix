{
  pkgs,
  config,
  ...
}: let
  goEngineDir = builtins.toString ./.;
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

  languages.go.enable = true;

  packages = [
    pkgs.curl
    pkgs.gnutar
    pkgs.zig
  ];

  env = {
    LIBS_DIR = "${goEngineDir}/.devenv/libs";
    OUTPUT_BASE = "${goEngineDir}/native-libs";
    TANTIVY_VERSION = "v1.0.6";
  };
  tasks = {
    "deps:download" = {
      description = "Download prebuilt tantivy-go static libs for Linux, Android, and Windows";
      exec = ''
        mkdir -p $LIBS_DIR/linux-musl \
                 $LIBS_DIR/windows-amd64 \
                 $LIBS_DIR/android-arm64 \
                 $LIBS_DIR/android-x86_64 \
                 $LIBS_DIR/android-armv7 \
                 $LIBS_DIR/android-i686

        TMP_DIR=$(mktemp -d)

        download_and_extract() {
          local url=$1
          local dest=$2
          echo "Downloading $dest..."
          curl -sL "$url" | tar -xz -C $TMP_DIR
          # Find libtantivy_go.a in extracted files and move it
          find $TMP_DIR -name "libtantivy_go.a" -exec cp {} "$dest/" \;
          rm -rf $TMP_DIR/*
        }

        # Desktop
        download_and_extract "https://github.com/anyproto/tantivy-go/releases/download/$TANTIVY_VERSION/linux-amd64-musl.tar.gz" "$LIBS_DIR/linux-musl"
        download_and_extract "https://github.com/anyproto/tantivy-go/releases/download/$TANTIVY_VERSION/windows-amd64.tar.gz" "$LIBS_DIR/windows-amd64"

        # Android
        download_and_extract "https://github.com/anyproto/tantivy-go/releases/download/$TANTIVY_VERSION/android-arm64.tar.gz" "$LIBS_DIR/android-arm64"
        download_and_extract "https://github.com/anyproto/tantivy-go/releases/download/$TANTIVY_VERSION/android-amd64.tar.gz" "$LIBS_DIR/android-x86_64"
        download_and_extract "https://github.com/anyproto/tantivy-go/releases/download/$TANTIVY_VERSION/android-arm.tar.gz" "$LIBS_DIR/android-armv7"
        download_and_extract "https://github.com/anyproto/tantivy-go/releases/download/$TANTIVY_VERSION/android-386.tar.gz" "$LIBS_DIR/android-i686"

        rm -rf $TMP_DIR
      '';
    };
    "engine:build-windows" = {
      description = "Build anytype-heart lib for Windows";
      cwd = goEngineDir;
      exec = ''
        export CGO_ENABLED=1
        export GOOS=windows
        export GOARCH=amd64

        export CC="zig cc -target x86_64-windows-gnu"
        export CXX="zig c++ -target x86_64-windows-gnu"

        export CGO_LDFLAGS="-L$LIBS_DIR/windows-amd64 -ltantivy_go -Wl,--out-implib,$OUTPUT_BASE/windows/libanytype_engine.lib"
        # export CGO_LDFLAGS="-L$LIBS_DIR/windows-amd64 -ltantivy_go -Wl,--out-implib,$OUTPUT_BASE/windows/libanytype_engine.dll.a"
        export RUSTFLAGS="-Clink-arg=-mwindows"

        mkdir -p $OUTPUT_BASE/windows
        go build -buildmode=c-shared -o $OUTPUT_BASE/windows/libanytype_engine.dll main.go
      '';
    };
    "engine:build-linux" = {
      description = "Build anytype-heart lib for Linux";
      cwd = goEngineDir;
      exec = ''
        export CGO_ENABLED=1
        export GOOS=linux
        export GOARCH=amd64
        export CGO_LDFLAGS="-L$LIBS_DIR/linux-musl -ltantivy_go"

        mkdir -p $OUTPUT_BASE/linux
        go build -buildmode=c-shared -o $OUTPUT_BASE/linux/libanytype_engine.so main.go
      '';
    };

    "engine:build-android" = {
      description = "Build anytype-heart lib for Android";
      cwd = goEngineDir;
      exec = ''
        NDK_BIN="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin"
        export CGO_ENABLED=1
        export GOOS=android
        export AR="$NDK_BIN/llvm-ar"

        API_LEVEL=28

        ARCHS=("aarch64" "x86_64" "armv7" "i686")

        for ARCH in "''${ARCHS[@]}"; do
          echo "Building Go engine for Android ($ARCH)..."

          case "$ARCH" in
            "aarch64")
              export CC="$NDK_BIN/aarch64-linux-android$API_LEVEL-clang"
              export CXX="$NDK_BIN/aarch64-linux-android$API_LEVEL-clang++"
              export GOARCH=arm64
              export GOARM=""
              export CGO_LDFLAGS="-L$LIBS_DIR/android-arm64 -ltantivy_go"
              ;;
            "x86_64")
              export CC="$NDK_BIN/x86_64-linux-android$API_LEVEL-clang"
              export CXX="$NDK_BIN/x86_64-linux-android$API_LEVEL-clang++"
              export GOARCH=amd64
              export GOARM=""
              export CGO_LDFLAGS="-L$LIBS_DIR/android-x86_64 -ltantivy_go"
              ;;
            "armv7")
              export CC="$NDK_BIN/armv7a-linux-androideabi$API_LEVEL-clang"
              export CXX="$NDK_BIN/armv7a-linux-androideabi$API_LEVEL-clang++"
              export GOARCH=arm
              export GOARM=7
              export CGO_LDFLAGS="-L$LIBS_DIR/android-armv7 -ltantivy_go"
              ;;
            "i686")
              export CC="$NDK_BIN/i686-linux-android$API_LEVEL-clang"
              export CXX="$NDK_BIN/i686-linux-android$API_LEVEL-clang++"
              export GOARCH=386
              export GOARM=""
              export CGO_LDFLAGS="-L$LIBS_DIR/android-i686 -ltantivy_go"
              ;;
          esac

          OUT_DIR="$OUTPUT_BASE/android/$ARCH"
          mkdir -p "$OUT_DIR"

          go build -buildmode=c-shared -o "$OUT_DIR/libanytype_engine.so" main.go
        done

        echo "Successfully built libanytype_engine.so for all Android architectures!"
      '';
    };

    "engine:build" = {
      description = "Build anytype-heart lib for Windows, Linux and Android";
      exec = "echo '✅ All platform targets successfully evaluated.'";
      after = [
        "engine:build-linux"
        "engine:build-windows"
        "engine:build-android"
      ];
    };
  };
  git-hooks.hooks = {
    alejandra.enable = true;
  };
}
