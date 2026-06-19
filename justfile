set dotenv-load := true
set shell := ["nu", "-c"]

export NDK_HOME := env_var('NDK_HOME')
export ANDROID_HOME := env_var('ANDROID_HOME')
export JAVA_HOME := env_var('JAVA_HOME')

bundle-android:
    @echo "Using NDK from: {{ NDK_HOME }}"
    dx bundle --platform android  --release --verbose --out-dir ./dist/android-abb

serve-android:
    @echo "Running on android device..."
    dx serve --verbose --android --device

bundle-windows-nsis:
    @echo "Building windows nsis bundle..."
    dx bundle --package-types "nsis" --release --features bundle --verbose --out-dir ./dist/windows

bundle-windows-msi:
    @echo "Building windows msi bundle..."
    dx bundle --package-types "msi" --release --features bundle --verbose --out-dir ./dist/windows
