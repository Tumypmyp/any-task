# AnyTask: Cross-Platform Client for Anytype

**A modern, lightweight client for Anytype built with Rust and Dioxus.**

[Anytype](https://github.com/anyproto/) has a great desktop, Android and iOS clients. This project focuses on cross-platform implementation of the client using [Dioxus](https://github.com/DioxusLabs/dioxus), a modern Rust framework. The goal is to raise phone and tablet experience to the level of the Anytype desktop app.

**⚠️ AnyTask is currently in early development.**

<div align="center">
  <video src="./notes/android_demo.webm" height="500" autoplay loop muted playsinline></video>
</div>

## Features

- [x] Standalone app (`anytype-heart` library is embedded)
- [x] Views: Spaces, Lists, Objects
- [x] Relations: Text, Checkbox, Date, Number, Status, etc.
- [x] Customize relations in Lists
- [x] List views
- [x] Live relation updates
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
devenv tasks run bundle:android         # bundle android apk
```

### Manual install for Windows, Linux, Mac

- [dioxus-cli](https://github.com/DioxusLabs/dioxus)

```bash
cargo install dioxus-cli --version 0.7.9 --locked
```

```bash
just serve-windows
just serve-android
```
