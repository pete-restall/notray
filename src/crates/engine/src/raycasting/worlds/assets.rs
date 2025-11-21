use crate::{Colour, WellKnownColours};
use crate::raycasting::*;

pub struct Palette;

impl Palette {
    pub const BLACK: Colour = Colour::new(0);
    pub const TRANSPARENT: Colour = Colour::new(0);
    pub const GRASS_LIGHTEST: Colour = Colour::new(48);
    pub const SKY_LIGHTEST: Colour = Colour::new(144);

    pub fn rgb_for(colour: Colour) -> (u8, u8, u8) {
        static PALETTE: &[u8; 256 * 3] = include_bytes!("palette.rgb");
        let index = colour.as_index() as usize * 3;
        let entry = &PALETTE[index..index + 3];
        (entry[0], entry[1], entry[2])
    }
}

impl WellKnownColours for Palette {
    const BLACK: Colour = Palette::BLACK;
    const TRANSPARENT: Colour = Palette::TRANSPARENT;
}

type NonTransparentTexture<'t, const W: u8, const H: u8> = StretchedStaticTexture<'t, W, H, 0xff00>;

type Brick1Texture<'t> = NonTransparentTexture<'t, 64, 64>;
type Brick1TextureColumnRenderer<'c> = TextureMappedColumnRenderer<'c, Brick1Texture<'c>>;

type Stone1Texture<'t> = NonTransparentTexture<'t, 64, 64>;
type Stone1TextureColumnRenderer<'c> = TextureMappedColumnRenderer<'c, Stone1Texture<'c>>;

pub struct Textures<'c> {
    brick1: Brick1Texture<'c>,
    stone1: Stone1Texture<'c>
}

pub enum TextureRenderer<'c> {
    Unknown(SolidColourColumnRenderer<'c>),
    Brick1(Brick1TextureColumnRenderer<'c>),
    Stone1(Stone1TextureColumnRenderer<'c>)
}

impl<'c> Textures<'c> {
    pub const fn new() -> Self {
        Self {
            brick1: Brick1Texture::new(include_bytes!("brick1-64x64.raw")),
            stone1: Brick1Texture::new(include_bytes!("stone1-64x64.raw"))
        }
    }

    pub fn new_renderer_for(&'c self, cell_tag: Option<CellTag>, column: &'c mut RenderingColumn) -> TextureRenderer<'c> {
        match cell_tag.map(|x| x.world_cell_id()).unwrap_or(255) {
            1 => TextureRenderer::Brick1(Brick1TextureColumnRenderer::new(&self.brick1, column)),
            2 => TextureRenderer::Stone1(Stone1TextureColumnRenderer::new(&self.stone1, column)),
            _ => TextureRenderer::Unknown(SolidColourColumnRenderer::new(Palette::BLACK, column))
        }
    }
}
