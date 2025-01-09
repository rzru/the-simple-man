use agb::{
    display::object::{Graphics, Tag},
    include_aseprite,
};

pub static GRAPHICS: &Graphics = include_aseprite!("gfx/char-front.aseprite");

pub static BALL_GRAPHICS: &Graphics = include_aseprite!("gfx/ball.aseprite");

pub static HOUSES: &Graphics = include_aseprite!("gfx/houses.aseprite");

pub static CHAR_FRONT: &Tag = GRAPHICS.tags().get("Char Front");
pub static CHAR_BACK: &Tag = GRAPHICS.tags().get("Char Back");
pub static CHAR_LEFT: &Tag = GRAPHICS.tags().get("Char Left");
pub static CHAR_RIGHT: &Tag = GRAPHICS.tags().get("Char Right");

pub static BALL_TAG: &Tag = BALL_GRAPHICS.tags().get("Ball");

pub static HOUSE_1: &Tag = HOUSES.tags().get("House-1");
