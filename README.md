# Any tasks

Anytype has great desktop, Android and iOS clients. This project focuses on cross-platform compatibility using Dioxus, a modern Rust framework.

The client is currently in active development and supports only the desktop version of Anytype. The Android version will become available upon the release of the Anytype API on Android.

<div align="center">
  <img src="./notes/ui.jpg" width="1000">
</div>

## Features

- [x] Auth with API token
- [x] Choose space
- [x] View tasks
- [x] Check/uncheck Done property
- [x] Remember state of the app after closing
- [x] Change Tag property
- [x] View Date/Time
- [ ] Change Date/Time
- [ ] Auth with 4-digit code
- [ ] Choose query
- [ ] Timeline view
- [ ] Build for Android


## Development

The project includes `assets` folder and a `views` folder. API code is generated with `openapi-generator-cli` to `api`.


## Installing Dependencies

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