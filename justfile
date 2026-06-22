set dotenv-load := true
set shell := ["nu", "-c"]

export NDK_HOME := env_var('NDK_HOME')
export ANDROID_HOME := env_var('ANDROID_HOME')
export JAVA_HOME := env_var('JAVA_HOME')
export ENGINE_NAME := "lib_anytype_engine"
PROJECT_DIR := justfile_directory()
export ENGINE_LIBS := PROJECT_DIR + "/go-engine/native-libs"
export WINDOWS_ENGINE_LIB := ENGINE_LIBS + "/windows/" + ENGINE_NAME + ".lib"
export LINUX_ENGINE_LIB := ENGINE_LIBS + "/linux/" + ENGINE_NAME + ".so"
export ANDROID_ENGINE_LIB := ENGINE_LIBS + "/android/aarch64/" + ENGINE_NAME + ".so"

serve-android:
    @echo "Running on android device..."
    dx serve --verbose --android --device

serve-windows:
    @echo "Running windows desktop app..."
    dx serve --platform desktop

bundle-android:
    @echo "Using NDK from: {{ NDK_HOME }}"
    dx bundle --platform android  --release --verbose --out-dir ./dist/android-abb

bundle-windows-nsis:
    @echo "Building windows nsis bundle..."
    dx bundle --package-types "nsis" --release --features bundle --verbose --out-dir ./dist/windows

bundle-windows-msi:
    @echo "Building windows msi bundle..."
    dx bundle --package-types "msi" --release --features bundle --verbose --out-dir ./dist/windows
