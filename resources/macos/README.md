# macOS Icon Generation

The app bundle icon is built from `resources/icons/lintel.svg` on macOS CI. A valid `lintel.icns` must be generated with multiple resolutions (16, 32, 64, 128, 256, 512) via ImageMagick + icnsutil on a macOS runner.

Until then, the window icon is embedded via egui's `with_icon()`.
