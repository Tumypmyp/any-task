# Contributing to AnyTask

```
.
├── build.rs                      # build script to add compiled engine inside the app
├── go-engine/                    # Go layer on top of anytype-heart (compiled to lib)
├── src/                          # Dioxus app
│   ├── anytype_cli.rs
│   ├── components/
│   ├── engine.rs
│   ├── helpers
│   ├── main.rs
│   ├── persistent_history.rs
│   ├── proxy.rs
│   └── views/
```

## Anytype CLI

### Useful commands

```
cd anytype-binaries
./anytype-x86_64-pc-windows-msvc auth create test-bot
./anytype-x86_64-pc-windows-msvc auth login
./anytype-x86_64-pc-windows-msvc space list
./anytype-x86_64-pc-windows-msvc space join [invite link]
./anytype-x86_64-pc-windows-msvc auth apikey create test-key


```

## Client API generation

- copy `openapi-version.yaml` to `apis/`
- build the client
  ```bash
  (devenv) client-api-generate
  ```

### 2025-11-08 Patches

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
