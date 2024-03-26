use agb::{
    display::object::{Graphics, Tag},
    include_aseprite,
};

pub const GRAPHICS: &Graphics = include_aseprite!("gfx/char-front.aseprite");

pub const CHAR_FRONT: &Tag = GRAPHICS.tags().get("Char Front");
pub const CHAR_BACK: &Tag = GRAPHICS.tags().get("Char Back");
pub const CHAR_LEFT: &Tag = GRAPHICS.tags().get("Char Left");
pub const CHAR_RIGHT: &Tag = GRAPHICS.tags().get("Char Right");
