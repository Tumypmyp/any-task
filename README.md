# AnyTask: Cross-Platform Client for Anytype

**A modern, lightweight client for Anytype built with Rust and Dioxus.**

[Anytype](https://github.com/anyproto/) has a great desktop, Android and iOS clients. This project focuses on cross-platform implementation of the client using [Dioxus](https://github.com/DioxusLabs/dioxus), a modern Rust framework. The goal is to raise phone and tablet experience to the level of the Anytype desktop app.

**⚠️ AnyTask is currently in early development.**

## Real-time Property Management

<div align="center">
  <img src="./notes/windows_demo.webp" width="700">
</div>

## Customizable Object Views

<div align="center">
  <img src="./notes/android_demo.webp" height="500">
</div>

## Features

- [x] Standalone app (`anytype-heart` library is embedded)
- [x] Views: Spaces, Lists, Objects
- [ ] Properties: Text, Checkbox, Select, Date, etc.
- [x] Customize properties appearence in Lists
- [ ] Live property update on value change
- [ ] Use different List views
- [x] Open last visited view after app reload
<!--- [ ] Timeline/Calendar view-->

## Developing

### NixOS

You can install development environment with all the dependencies using [devenv](https://devenv.sh/).

```bash
devenv shell -v
```

```bash
devenv tasks run deps:download          # download anytype-heart dependencies
devenv tasks run engine:build           # build anytype-heart library
devenv tasks run proto:download         # download anytype-heart proto files for communication
```

### Manual install for Windows, Linux, Mac

- [dioxus-cli](https://github.com/DioxusLabs/dioxus)

```bash
cargo install --git https://github.com/DioxusLabs/dioxus dioxus-cli --locked
```
