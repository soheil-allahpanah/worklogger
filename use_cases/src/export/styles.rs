use rust_xlsxwriter::{Color, Format, FormatAlign, FormatBorder};

/// Same palette as the TUI — stable hash → color for tag labels.
const TAG_PALETTE: [Color; 10] = [
    Color::RGB(0x78_AE_EB), // blue
    Color::RGB(0x58_C4_8C), // emerald
    Color::RGB(0xEB_A3_78), // coral
    Color::RGB(0xB4_8C_EB), // lavender
    Color::RGB(0xEB_C4_58), // gold
    Color::RGB(0x78_D2_D2), // teal
    Color::RGB(0xEB_82_AA), // rose
    Color::RGB(0xA0_D2_78), // lime
    Color::RGB(0xD2_96_EB), // orchid
    Color::RGB(0xEB_D2_82), // sand
];

pub fn tag_color_index(tag: &str) -> usize {
    let hash = tag
        .bytes()
        .fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(u64::from(b)));
    (hash as usize) % TAG_PALETTE.len()
}

pub struct ExportStyles {
    pub title: Format,
    pub header: Format,
    pub row_even: Format,
    pub row_odd: Format,
    pub tag_separator: Format,
    pub tag_formats: [Format; 10],
}

impl ExportStyles {
    pub fn new() -> Self {
        let border = Format::new().set_border(FormatBorder::Thin).set_border_color(Color::RGB(0xC8_CC_D4));

        let title = Format::new()
            .set_bold()
            .set_font_size(14.0)
            .set_font_color(Color::White)
            .set_background_color(Color::RGB(0x26_26_2A))
            .set_align(FormatAlign::VerticalCenter);

        let header = Format::new()
            .set_bold()
            .set_font_color(Color::White)
            .set_background_color(Color::RGB(0x3C_78_D8))
            .set_border(FormatBorder::Thin)
            .set_border_color(Color::RGB(0x2F_5F_AE))
            .set_align(FormatAlign::Center)
            .set_align(FormatAlign::VerticalCenter);

        let row_even = border
            .clone()
            .set_background_color(Color::RGB(0xF4_F5_F7))
            .set_align(FormatAlign::VerticalCenter);

        let row_odd = border
            .clone()
            .set_background_color(Color::White)
            .set_align(FormatAlign::VerticalCenter);

        let tag_separator = Format::new().set_font_color(Color::RGB(0x9A_9E_A6));

        let tag_formats = std::array::from_fn(|i| {
            Format::new()
                .set_bold()
                .set_font_color(TAG_PALETTE[i])
        });

        Self {
            title,
            header,
            row_even,
            row_odd,
            tag_separator,
            tag_formats,
        }
    }

    pub fn row_format(&self, data_row_index: usize) -> &Format {
        if data_row_index % 2 == 0 {
            &self.row_even
        } else {
            &self.row_odd
        }
    }
}
