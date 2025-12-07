bundle-all: bundle-windows-nsis bundle-windows-msi

bundle-windows-nsis:
  @echo "Building windows nsis bundle..."
  dx bundle --package-types "nsis" --release --verbose --out-dir ./dist/windows

bundle-windows-msi:
  @echo "Building windows msi bundle..."
  dx bundle --package-types "msi" --release --verbose --out-dir ./dist/windows
