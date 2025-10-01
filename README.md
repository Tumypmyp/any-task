# AnyTasks

[Anytype](https://github.com/anyproto/) has great desktop, Android and iOS clients. This project focuses on cross-platform compatibility using Dioxus, a modern Rust framework.

The client is currently in active development and supports only the desktop version of Anytype. The Android version will become available after the release of Anytype API on Android.

<div align="center">
  <img src="./notes/ui.png" width="1000">
</div>

## Features

- [x] Login with API token
- [x] Save/Remove API token
- [x] View Spaces
- [x] View Objects
- [x] View Queries
- [x] Text property
- [x] Done property
- [x] Tag property
- [x] Date/Time property
- [x] Choose viewed properties
- [x] Configure viewed properties
- [ ] Choose views
- [ ] Auth with 4-digit code
- [ ] Timeline view
- [ ] Build for Android



## Installing Dependencies

- API code is generated with `openapi-generator-cli`.
- [dioxus-cli](https://github.com/DioxusLabs/dioxus)

### Windows, Linux

```bash
cargo install --git https://github.com/DioxusLabs/dioxus dioxus-cli --locked
```

### NixOS

```bash
nix develop
```

## Building

### Desktop

```bash
dx serve --platform desktop
```

### Android

- `adb pair 192.168.x.x:port` - connect to device with wireless debugging
- `adb devices` - get connected devices
 

```bash
dx serve --platform android --device
```

## Publishing

```bash
dx bundle --platform desktop
```
