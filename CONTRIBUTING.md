# Contributing to AnyTask

```
.
├── build.rs                      # build script to add compiled engine inside the app
├── go-engine/                    # Go layer on top of anytype-heart (compiled to lib)
├── protos/                       # Proto files used for speaking with anytype-heart
├── src/                          # Dioxus app
│   ├── components/
│   ├── engine.rs
│   ├── helpers
│   ├── main.rs
│   ├── persistent_history.rs
│   ├── components                # Repeating UI components (from dioxus-components and custom ones)
│   └── views/                    # Possible UI views in the AnyTask
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
  - Connect to device (on WSL you should first start adb service in Windows)
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
devenv tasks run bundle:windows
```

#### Android

```bash
devenv tasks run bundle:android
```
