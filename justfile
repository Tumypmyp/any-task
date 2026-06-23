set dotenv-load := true
set shell := ["nu", "-c"]

export NDK_HOME := env('NDK_HOME')
export ANDROID_HOME := env('ANDROID_HOME')
export JAVA_HOME := env('JAVA_HOME')
PROJECT_DIR := justfile_directory()
export ENGINE_NAME := "lib_anytype_engine"
export ENGINE_LIBS := PROJECT_DIR + "/go-engine/native-libs"
export WINDOWS_ENGINE_LIB := ENGINE_LIBS + "/windows/" + ENGINE_NAME + ".lib"
export LINUX_ENGINE_LIB := ENGINE_LIBS + "/linux/" + ENGINE_NAME + ".so"
export ANDROID_ENGINE_LIB := ENGINE_LIBS + "/android/aarch64/" + ENGINE_NAME + ".so"

serve-android:
    @echo "Running on android device..."
    dx serve --verbose --android --device --target aarch64-linux-android

serve-windows:
    @echo "Running windows desktop app..."
    dx serve --platform desktop

bundle-android:
    @echo "Building android aab bundle..."
    dx bundle --platform android  --release --verbose --out-dir ./dist/android-abb --target aarch64-linux-android

bundle-windows-nsis:
    @echo "Building windows nsis bundle..."
    dx bundle --package-types "nsis" --release --features bundle --verbose --out-dir ./dist/windows

bundle-windows-msi:
    @echo "Building windows msi bundle..."
    dx bundle --package-types "msi" --release --features bundle --verbose --out-dir ./dist/windows
