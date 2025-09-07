# chromashift

A minimal library for converting between various color formats and color spaces.

📖 **[Full Documentation](https://csskit.rs/docs/internal/chromashift/)**

## Purpose

Chromashift is a focused color manipulation library designed to seamlessly convert color formats between one and other.
It provides comprehensive color space conversions while maintaining a minimal dependency footprint. Developed following
CSS specifications, for processing colors in [csskit](https://csskit.rs/).

### Color Format Support

- **RGB/sRGB**: Standard RGB with optional alpha channel
- **HSL**: Hue, Saturation, Lightness with intuitive manipulation
- **HSV/HSB**: Hue, Saturation, Value/Brightness for color pickers
- **HWB**: Hue, Whiteness, Blackness as specified in CSS Color Level 4
- **LAB**: Perceptually uniform CIE L*a*b\* color space
- **LCH**: Lightness, Chroma, Hue cylindrical representation of LAB
- **XYZ**: CIE XYZ tristimulus values for device-independent color
