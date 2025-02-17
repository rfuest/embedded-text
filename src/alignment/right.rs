//! Right aligned text.
use crate::{
    alignment::HorizontalTextAlignment, rendering::space_config::UniformSpaceConfig,
    style::LineMeasurement,
};
use embedded_graphics::text::renderer::TextRenderer;

/// Marks text to be rendered right aligned.
#[derive(Copy, Clone, Debug)]
pub struct RightAligned;
impl HorizontalTextAlignment for RightAligned {
    type SpaceConfig = UniformSpaceConfig;

    #[inline]
    fn place_line(
        _line: &str,
        renderer: &impl TextRenderer,
        measurement: LineMeasurement,
    ) -> (u32, Self::SpaceConfig) {
        (
            measurement.max_line_width - measurement.width,
            UniformSpaceConfig::new(renderer),
        )
    }
}

#[cfg(test)]
mod test {
    use embedded_graphics::{
        geometry::Point,
        mock_display::MockDisplay,
        mono_font::{ascii::FONT_6X9, MonoTextStyleBuilder},
        pixelcolor::BinaryColor,
        primitives::Rectangle,
        Drawable,
    };

    use crate::{
        alignment::RightAligned, rendering::test::assert_rendered, style::TextBoxStyleBuilder,
        utils::test::size_for, TextBox,
    };

    #[test]
    fn simple_render() {
        assert_rendered(
            RightAligned,
            "word",
            size_for(&FONT_6X9, 6, 1),
            &[
                "            ........................",
                "            ......................#.",
                "            ......................#.",
                "            #...#...##...#.#....###.",
                "            #.#.#..#..#..##.#..#..#.",
                "            #.#.#..#..#..#.....#..#.",
                "            .#.#....##...#......###.",
                "            ........................",
                "            ........................",
            ],
        );
    }

    #[test]
    fn simple_render_cr() {
        let mut display = MockDisplay::new();
        display.set_allow_overdraw(true);

        let character_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X9)
            .text_color(BinaryColor::On)
            .build();

        let style = TextBoxStyleBuilder::new().alignment(RightAligned).build();

        TextBox::with_textbox_style(
            "O\rX",
            Rectangle::new(Point::zero(), size_for(&FONT_6X9, 3, 1)),
            character_style,
            style,
        )
        .draw(&mut display)
        .unwrap();

        display.assert_pattern(&[
            "                  ",
            "            ##### ",
            "            ## ## ",
            "            # # # ",
            "            # # # ",
            "            ## ## ",
            "            ##### ",
        ]);
    }

    #[test]
    fn simple_word_wrapping() {
        assert_rendered(
            RightAligned,
            "word wrapping",
            size_for(&FONT_6X9, 9, 2),
            &[
                "                              ........................",
                "                              ......................#.",
                "                              ......................#.",
                "                              #...#...##...#.#....###.",
                "                              #.#.#..#..#..##.#..#..#.",
                "                              #.#.#..#..#..#.....#..#.",
                "                              .#.#....##...#......###.",
                "                              ........................",
                "                              ........................",
                "      ................................................",
                "      ................................#...............",
                "      ................................................",
                "      #...#..#.#....###..###...###...##....###....##..",
                "      #.#.#..##.#..#..#..#..#..#..#...#....#..#..#..#.",
                "      #.#.#..#.....#..#..#..#..#..#...#....#..#..#..#.",
                "      .#.#...#......###..###...###...###...#..#...###.",
                "      ...................#.....#....................#.",
                "      ...................#.....#..................##..",
            ],
        );
    }

    #[test]
    fn word_longer_than_line_wraps_word() {
        assert_rendered(
            RightAligned,
            "word  somereallylongword",
            size_for(&FONT_6X9, 9, 3),
            &[
                "                              ........................",
                "                              ......................#.",
                "                              ......................#.",
                "                              #...#...##...#.#....###.",
                "                              #.#.#..#..#..##.#..#..#.",
                "                              #.#.#..#..#..#.....#..#.",
                "                              .#.#....##...#......###.",
                "                              ........................",
                "                              ........................",
                "......................................................",
                "...........................................##....##...",
                "............................................#.....#...",
                "..###...##..##.#....##...#.#....##....###...#.....#...",
                ".##....#..#.#.#.#..#.##..##.#..#.##..#..#...#.....#...",
                "...##..#..#.#.#.#..##....#.....##....#..#...#.....#...",
                ".###....##..#...#...###..#......###...###..###...###..",
                "......................................................",
                "......................................................",
                "......................................................",
                ".......##...........................................#.",
                "........#...........................................#.",
                ".#..#...#.....##...###....##..#...#...##...#.#....###.",
                ".#..#...#....#..#..#..#..#..#.#.#.#..#..#..##.#..#..#.",
                ".#..#...#....#..#..#..#..#..#.#.#.#..#..#..#.....#..#.",
                "..###..###....##...#..#...###..#.#....##...#......###.",
                ".#..#.......................#.........................",
                "..##......................##..........................",
            ],
        );
    }

    #[test]
    fn first_word_longer_than_line_wraps_word() {
        assert_rendered(
            RightAligned,
            "somereallylongword",
            size_for(&FONT_6X9, 9, 2),
            &[
                "......................................................",
                "...........................................##....##...",
                "............................................#.....#...",
                "..###...##..##.#....##...#.#....##....###...#.....#...",
                ".##....#..#.#.#.#..#.##..##.#..#.##..#..#...#.....#...",
                "...##..#..#.#.#.#..##....#.....##....#..#...#.....#...",
                ".###....##..#...#...###..#......###...###..###...###..",
                "......................................................",
                "......................................................",
                "......................................................",
                ".......##...........................................#.",
                "........#...........................................#.",
                ".#..#...#.....##...###....##..#...#...##...#.#....###.",
                ".#..#...#....#..#..#..#..#..#.#.#.#..#..#..##.#..#..#.",
                ".#..#...#....#..#..#..#..#..#.#.#.#..#..#..#.....#..#.",
                "..###..###....##...#..#...###..#.#....##...#......###.",
                ".#..#.......................#.........................",
                "..##......................##..........................",
            ],
        );
    }

    #[test]
    fn soft_hyphen_rendering() {
        assert_rendered(
            RightAligned,
            "soft\u{AD}hyphen",
            size_for(&FONT_6X9, 6, 2),
            &[
                "      ..............................",
                "      ...............#....#.........",
                "      ..............#.#...#.........",
                "      ..###...##....#....###........",
                "      .##....#..#..###....#...#####.",
                "      ...##..#..#...#.....#.#.......",
                "      .###....##....#......#........",
                "      ..............................",
                "      ..............................",
                "....................................",
                ".#.................#................",
                ".#.................#................",
                ".###...#..#..###...###....##...###..",
                ".#..#..#..#..#..#..#..#..#.##..#..#.",
                ".#..#..#..#..#..#..#..#..##....#..#.",
                ".#..#...###..###...#..#...###..#..#.",
                ".......#..#..#......................",
                "........##...#......................",
            ],
        );
    }
}
