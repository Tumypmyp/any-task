# Contributing to AnyTask

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
