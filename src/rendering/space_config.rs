//! Space rendering config

use embedded_graphics::text::renderer::TextRenderer;

use crate::utils::str_width;

/// Retrieves size of space characters.
pub trait SpaceConfig: Copy {
    /// Look at the size of next n spaces, without advancing.
    fn peek_next_width(&self, n: u32) -> u32;

    /// Advance the internal state
    fn consume(&mut self, n: u32) -> u32;
}

/// Contains the fixed width of a space character.
#[derive(Copy, Clone, Debug)]
pub struct UniformSpaceConfig {
    /// Space width.
    pub space_width: u32,
}

impl UniformSpaceConfig {
    /// Creates a default space configuration object based on the current MonoFont.
    #[inline]
    #[must_use]
    pub fn new<F: TextRenderer>(renderer: &F) -> Self {
        Self {
            space_width: str_width(renderer, " "),
        }
    }
}

impl SpaceConfig for UniformSpaceConfig {
    #[inline]
    fn peek_next_width(&self, n: u32) -> u32 {
        n * self.space_width
    }

    #[inline]
    fn consume(&mut self, n: u32) -> u32 {
        self.peek_next_width(n)
    }
}
