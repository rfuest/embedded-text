//! Top vertical text alignment.
use crate::{
    alignment::{HorizontalTextAlignment, VerticalTextAlignment},
    rendering::cursor::Cursor,
    style::height_mode::HeightMode,
    TextBox,
};
use embedded_graphics::text::renderer::TextRenderer;

/// Align text to the top of the TextBox.
#[derive(Copy, Clone, Debug)]
pub struct TopAligned;

impl VerticalTextAlignment for TopAligned {
    #[inline]
    fn apply_vertical_alignment<'a, S, A, H>(
        _cursor: &mut Cursor,
        _styled_text_box: &'a TextBox<'a, S, A, Self, H>,
    ) where
        S: TextRenderer,
        A: HorizontalTextAlignment,
        H: HeightMode,
    {
        // nothing to do here
    }
}

#[cfg(test)]
mod test {
    use embedded_graphics::{
        mock_display::MockDisplay,
        mono_font::{ascii::FONT_6X9, MonoTextStyleBuilder},
        pixelcolor::BinaryColor,
        prelude::*,
        primitives::Rectangle,
    };

    use crate::{alignment::TopAligned, style::TextBoxStyleBuilder, TextBox};

    #[test]
    fn test_top_alignment() {
        let mut display = MockDisplay::new();

        let character_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X9)
            .text_color(BinaryColor::On)
            .background_color(BinaryColor::Off)
            .build();

        let style = TextBoxStyleBuilder::new()
            .vertical_alignment(TopAligned)
            .build();

        TextBox::with_textbox_style(
            "word",
            Rectangle::new(Point::zero(), Size::new(55, 16)),
            character_style,
            style,
        )
        .draw(&mut display)
        .unwrap();

        display.assert_pattern(&[
            "........................",
            "......................#.",
            "......................#.",
            "#...#...##...#.#....###.",
            "#.#.#..#..#..##.#..#..#.",
            "#.#.#..#..#..#.....#..#.",
            ".#.#....##...#......###.",
            "........................",
            "........................",
            "                        ",
            "                        ",
            "                        ",
            "                        ",
            "                        ",
            "                        ",
            "                        ",
            "                        ",
        ]);
    }
}
