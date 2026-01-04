# AnyTask: Cross-Platform Client for Anytype
**A modern, lightweight client for Anytype built with Rust and Dioxus.**

[Anytype](https://github.com/anyproto/) has a great desktop, Android and iOS clients. This project focuses on cross-platform compatibility between clients using [Dioxus](https://github.com/DioxusLabs/dioxus), a modern Rust framework.

**⚠️ AnyTask is currently in active development.**

<div align="center">
  <img src="./notes/ui.png" width="1000">
</div>

## Features

- [x] Login/Logout with 4-digit code
- [x] Views: Spaces, Queries, Objects
- [x] Properties: Text, Checkbox, Select, Date
- [x] Add/edit viewed properties
- [x] Choose query views
- [ ] Timeline view


## How to use

**⚠️ Requirement:** The current version of AnyTask requires a **running official Anytype desktop client** on the local network to function.

### Android

To use Android app you should first install AnyTask on device with a desktop Anytype installed.
Then connect to the same Private network (Wi-Fi) and enter the desktop ip in the app (example: 10.0.0.45:31010).
- Request 4-digit code
- Enter the code in the app


### Windows



## Developing

### Dependencies

- Client API code is generated with [openapi-generator](https://github.com/OpenAPITools/openapi-generator)
- [anytype-api](https://github.com/anyproto/anytype-api/)
- [dioxus-cli](https://github.com/DioxusLabs/dioxus)

#### Windows, Linux

```bash
cargo install --git https://github.com/DioxusLabs/dioxus dioxus-cli --locked
```

#### NixOS

```bash
devenv shell -v
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

### Client API generation

- copy `openapi-version.yaml` to `apis/`
- build the client
  ```bash
  (devenv) client-api-generate
  ```
#### 2025-11-08 Patches
- add view type enums (can return types outside the enum set)
  ```
    enum:
      - grid
      - table
      - list
      - galery
      - kanban
      - calendar
      - graph
  ```
- add property name mapping
  ```   
    PropertyWithValue:
      discriminator:
        propertyName: format
        mapping:
          text: "#/components/schemas/TextPropertyValue"
          number: "#/components/schemas/NumberPropertyValue"
          select: "#/components/schemas/SelectPropertyValue"
          multi_select: "#/components/schemas/MultiSelectPropertyValue"
          date: "#/components/schemas/DatePropertyValue"
          files: "#/components/schemas/FilesPropertyValue"
          checkbox: "#/components/schemas/CheckboxPropertyValue"
          url: "#/components/schemas/UrlPropertyValue"
          email: "#/components/schemas/EmailPropertyValue"
          phone: "#/components/schemas/PhonePropertyValue"
          objects: "#/components/schemas/ObjectsPropertyValue"
  ```
