// SPDX-License-Identifier: GPL-3.0-only

/// Parse a hex color string (e.g., "#62a0ea") to an iced Color
pub fn parse_hex_color(hex: &str) -> Option<cosmic::iced::Color> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(cosmic::iced::Color::from_rgb8(r, g, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_color_with_hash() {
        let color = parse_hex_color("#62a0ea").unwrap();
        assert_eq!(color.r, 0x62 as f32 / 255.0);
        assert_eq!(color.g, 0xa0 as f32 / 255.0);
        assert_eq!(color.b, 0xea as f32 / 255.0);
    }

    #[test]
    fn test_parse_hex_color_without_hash() {
        let color = parse_hex_color("ff0000").unwrap();
        assert_eq!(color.r, 1.0);
        assert_eq!(color.g, 0.0);
        assert_eq!(color.b, 0.0);
    }

    #[test]
    fn test_parse_hex_color_invalid_length() {
        assert!(parse_hex_color("#fff").is_none());
        assert!(parse_hex_color("#fffffff").is_none());
        assert!(parse_hex_color("").is_none());
    }

    #[test]
    fn test_parse_hex_color_invalid_chars() {
        assert!(parse_hex_color("#gggggg").is_none());
        assert!(parse_hex_color("#zzzzzz").is_none());
    }
}
