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
    pkgs.gnutar
    pkgs.zig
  ];

  env = {
    LIBS_DIR = "${goEngineDir}/.devenv/libs";
    ENGINE_LIBS = "${goEngineDir}/native-libs";
    TANTIVY_VERSION = "v1.0.6";
    ENGINE_NAME = "lib_anytype_engine";
    WINDOWS_ENGINE_LIB = "${config.env.ENGINE_LIBS}/windows/${config.env.ENGINE_NAME}.dll";
    LINUX_ENGINE_LIB = "${config.env.ENGINE_LIBS}/linux/${config.env.ENGINE_NAME}.so";
    ANDROID_ENGINE_LIB = "${config.env.ENGINE_LIBS}/android/aarch64/${config.env.ENGINE_NAME}.so";
  };
  tasks = {
    "deps:download" = {
      description = "Download prebuilt tantivy-go static libs for Linux, Android, and Windows";
      package = pkgs.nushell;
      binary = "nu";
      exec = ''
        let base_url = $"https://github.com/anyproto/tantivy-go/releases/download/($env.TANTIVY_VERSION)"

        let targets = [
            {archive: "linux-amd64-musl",  dest: "linux-musl"},
            {archive: "windows-amd64",     dest: "windows-amd64"},
            {archive: "android-arm64",     dest: "android-arm64"},
            {archive: "android-amd64",     dest: "android-x86_64"},
            {archive: "android-arm",       dest: "android-armv7"},
            {archive: "android-386",       dest: "android-i686"},
        ]

        $targets | each { |t| mkdir $"($env.LIBS_DIR)/($t.dest)" }

        def download-and-extract [url: string, dest: string] {
            let tmp = (^mktemp -d | str trim)
            print $"Downloading ($dest)..."
            http get $url | ^tar -xz -C $tmp
            glob $"($tmp)/**/libtantivy_go.a" | each { |f| cp $f $dest }
            rm -rf $tmp
        }

        for t in $targets {
            download-and-extract $"($base_url)/($t.archive).tar.gz" $"($env.LIBS_DIR)/($t.dest)"
        }
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

        export CGO_LDFLAGS="-L$LIBS_DIR/windows-amd64 -ltantivy_go -Wl,--out-implib,$ENGINE_LIBS/windows/lib_anytype_engine.lib"
        export RUSTFLAGS="-Clink-arg=-mwindows"

        mkdir -p $ENGINE_LIBS/windows
        go build -ldflags="-s -w" -buildmode=c-shared -o $WINDOWS_ENGINE_LIB main.go
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

        mkdir -p $ENGINE_LIBS/linux
        go build -buildmode=c-shared -o $LINUX_ENGINE_LIB main.go
      '';
    };

    "engine:build-android" = {
      description = "Build anytype-heart lib for Android";
      cwd = goEngineDir;
      package = pkgs.nushell;
      binary = "nu";
      exec = ''
        let ndk_bin = $"($env.ANDROID_NDK_ROOT)/toolchains/llvm/prebuilt/linux-x86_64/bin"
        let api_level = 28

        let archs = [
            {arch: "aarch64", triple: $"aarch64-linux-android($api_level)",     goarch: "arm64", goarm: "",  libs_subdir: "android-arm64"}
            {arch: "x86_64",  triple: $"x86_64-linux-android($api_level)",      goarch: "amd64", goarm: "",  libs_subdir: "android-x86_64"}
            {arch: "armv7",   triple: $"armv7a-linux-androideabi($api_level)",  goarch: "arm",   goarm: "7", libs_subdir: "android-armv7"}
            {arch: "i686",    triple: $"i686-linux-android($api_level)",        goarch: "386",   goarm: "",  libs_subdir: "android-i686"}
        ]

        $env.CGO_ENABLED = "1"
        $env.GOOS = "android"
        $env.AR = $"($ndk_bin)/llvm-ar"

        for a in $archs {
            print $"Building Go engine for Android \(($a.arch)\)..."

            let out_dir = $"($env.ENGINE_LIBS)/android/($a.arch)"
            mkdir $out_dir

            with-env {
                CC:          $"($ndk_bin)/($a.triple)-clang"
                CXX:         $"($ndk_bin)/($a.triple)-clang++"
                GOARCH:      $a.goarch
                GOARM:       $a.goarm
                CGO_LDFLAGS: $"-L($env.LIBS_DIR)/($a.libs_subdir) -ltantivy_go"
            } {
              ^go build -buildmode=c-shared -ldflags="-s -w" -o $"($out_dir)/lib_anytype_engine.so" main.go
            }
        }
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
