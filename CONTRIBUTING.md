# Contributing to AnyTask

```
.
├── build.rs                      # build script to add compiled engine inside the app
├── go-engine/                    # Go layer on top of anytype-heart (compiled to lib)
├── src/                          # Dioxus app
│   ├── components/
│   ├── engine.rs
│   ├── helpers
│   ├── main.rs
│   ├── persistent_history.rs
│   └── views/
```

## Dependencies

NixOS is the primary target for development. But you can build and run the app localy after installing the development tools manually.

### Running

#### Desktop

```bash
dx serve --platform desktop
```

#### Android

- Android
  - Settings
  - Developer options
  - Wireless debugging (turn on, then hold)
  - Pair device with pairing code (`ip:port`, `code`)
- Terminal
  - Connect to device
    - `adb pair ip:port` - connect to device with wireless debugging
    - `code` - verify with code
    - `adb devices` - check connected devices
  - Run the app on the connected device
    ```bash
    dx serve --platform android --device
    ```
  - Debug
    - `adb shell run-as com.Tumypmyp.AnyTask` - login to Android shell
    - `ls files` - check app files directory

### Building installer

#### Desktop

```bash
dx bundle --desktop
```

#### Android

```bash
dx bundle --android
```

or

```bash
(devenv) bundle-android-apk
```
