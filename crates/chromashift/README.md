# chromashift

A library for converting between various color formats and color spaces.

📖 **[Full Documentation](https://csskit.rs/docs/internal/chromashift/)**

## Features

- **RGB/sRGB**: Standard RGB with optional alpha channel
- **HSL**: Hue, Saturation, Lightness with intuitive manipulation
- **HSV/HSB**: Hue, Saturation, Value/Brightness for color pickers
- **HWB**: Hue, Whiteness, Blackness as specified in CSS Color Level 4
- **LAB**: Perceptually uniform CIE L*a*b\* color space
- **LCH**: Lightness, Chroma, Hue cylindrical representation of LAB
- **XYZ**: CIE XYZ tristimulus values for device-independent color
- **RgbSpace**: White point, RGB<->XYZ matrices and transfer function for each CSS Color 4 predefined RGB space (sRGB, Display P3, A98 RGB, ProPhoto RGB, Rec. 2020)
- **Chromaticity / Matrix3 / Transfer**: The primitives those descriptors are built from - CIE xy chromaticities (with Bradford adaptation), 3x3 conversion matrices, and gamma encode/decode defined over the whole real line
- **InterpolationSpace**: Each CSS `<color-interpolation-method>`, with its hue direction for polar spaces, and colour interpolation in it: missing and powerless channels, hue interpolation, and premultiplied alpha

## Optional Features

- `anstyle` - Enables converting to [anstyle](https://crates.io/crates/anstyle).
- `rec2020-bt1886` - Decodes `rec2020` with the BT.1886 2.4 gamma that CSS Color 4 now specifies, instead of the BT.2020 OETF browsers currently render.

## Part of csskit

This crate is part of the csskit project, a comprehensive CSS toolchain.

For more information, visit [csskit.rs](https://csskit.rs/).

## License

MIT
