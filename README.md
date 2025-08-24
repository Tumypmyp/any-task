# Any tasks

[Anytype](https://github.com/anyproto/) has great desktop, Android and iOS clients. This project focuses on cross-platform compatibility using Dioxus, a modern Rust framework.

The client is currently in active development and supports only the desktop version of Anytype. The Android version will become available after the release of Anytype API on Android.

<div align="center">
  <img src="./notes/ui.jpg" width="1000">
</div>

## Features

- [x] Auth with API token
- [x] Space view
- [x] Tasks view
- [x] Done property
- [x] Save app state after closing
- [x] Tag property
- [ ] Choose query
- [ ] Choose properties
- [ ] Date/Time property
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

```bash
dx serve
```

## Publishing

```bash
dx bundle
```