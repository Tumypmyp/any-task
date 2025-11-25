# AnyTask

[Anytype](https://github.com/anyproto/) has great desktop, Android and iOS clients. This project focuses on cross-platform compatibility between clients using [Dioxus](https://github.com/DioxusLabs/dioxus), a modern Rust framework.

The client is currently in active development and supports building for desktop and Android. The full portable version of the client will be available after the release of headless Anytype. Currently the app asks for network connection to official Anytype desktop app.

<div align="center">
  <img src="./notes/ui.png" width="1000">
</div>

## Features

- [x] Login/Logout 4-digit code
- [x] Views: Spaces, Queries, Objects
- [x] Properties: Text, Checkbox, Select, Date
- [x] Choose viewed properties
- [x] Configure viewed properties
- [x] Choose query views
- [x] Build for Android
- [ ] Timeline view


## Running 
### Android
To use android app you should first setup a reverse proxy on a device with desktop Anytype installed.
The example config is in `sozu.toml`.
```bash
sozu start -c sozu.toml
```
Then connect to the same network and write ip:port of your server to the Android app (example: 10.0.0.45:31029).
- Request 4-digit code
- Enter the code in the app


## Developing

### Installing Dependencies

- API code is generated with `openapi-generator-cli`.
- [dioxus-cli](https://github.com/DioxusLabs/dioxus)

#### Windows, Linux

```bash
cargo install --git https://github.com/DioxusLabs/dioxus dioxus-cli --locked
```

#### NixOS

```bash
devenv shell
```

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
