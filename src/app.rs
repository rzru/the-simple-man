use core::fmt::Arguments;

use agb::{
    display::{
        object::{OamManaged, Object, Tag},
        tiled::{MapLoan, RegularBackgroundSize, RegularMap, Tiled0, TiledMap, VRamManager},
        Priority,
    },
    include_background_gfx,
    input::{ButtonController, Tri},
    interrupt::VBlank,
    mgba::{DebugLevel, Mgba},
    Gba,
};

use mapgen::generate_background_map;

use crate::tiles::BALL_TAG;

include_background_gfx!(test_bg, tiles => 256 "gfx/bg-tiles.png");

const INITIAL_COORDINATES: (i32, i32) = (104, 64);
const STEP: i32 = 24;

trait Entity<'a> {
    fn new(tag: &'static Tag, gfx: &'a OamManaged) -> Self;
    fn place(&mut self, coords: (i32, i32));
    fn process(&mut self, input: &ButtonController);
    fn tick(&mut self, gfx: &'a OamManaged);
}

struct Char<'a> {
    tag: &'a Tag,
    frame: usize,
    sprite: Object<'a>,
}

impl<'a> Entity<'a> for Char<'a> {
    fn new(tag: &'static Tag, gfx: &'a OamManaged) -> Self {
        let frame = 0;

        Self {
            tag,
            frame,
            sprite: gfx.object_sprite(tag.animation_sprite(frame)),
        }
    }

    fn place(&mut self, (x, y): (i32, i32)) {
        self.sprite.set_position((x, y)).show();
    }

    fn process(&mut self, input: &ButtonController) {
        todo!()
    }

    fn tick(&mut self, gfx: &'a OamManaged) {
        todo!()
    }
}

struct Background<'a> {
    bg: MapLoan<'a, RegularMap>,
    tilemap: [[usize; 32]; 32],
}

impl<'a> Background<'a> {
    fn new(bg_gfx: &'a Tiled0, mut vram: &mut VRamManager) -> Self {
        vram.set_background_palettes(test_bg::PALETTES);

        let tileset = &test_bg::tiles.tiles;
        let tilemap = generate_background_map!("gfx/bg.png");

        let mut bg = bg_gfx.background(
            Priority::P0,
            RegularBackgroundSize::Background32x32,
            tileset.format(),
        );

        for y in 0..32u16 {
            for x in 0..32u16 {
                bg.set_tile(
                    &mut vram,
                    (x, y),
                    &tileset,
                    test_bg::tiles.tile_settings[tilemap[y as usize][x as usize]],
                );
            }
        }

        bg.commit(&mut vram);
        bg.set_visible(true);

        Self { bg, tilemap }
    }
}

pub fn run(mut gba: Gba) -> ! {
    let vblank = VBlank::get();
    let mut input = ButtonController::new();

    let gfx = gba.display.object.get_managed();
    let (bg_gfx, mut vram) = gba.display.video.tiled0();

    let mut background = Background::new(&bg_gfx, &mut vram);

    let mut char = Char::new(BALL_TAG, &gfx);
    char.place(INITIAL_COORDINATES);

    let mut scroll_x = 0i16;
    let mut scroll_y = 0i16;

    loop {
        vblank.wait_for_vblank();
        input.update();

        match input.x_tri() {
            Tri::Positive => {
                if !path_right_blocked((scroll_x + 1, scroll_y), background.tilemap) {
                    scroll_x += 1
                }
            }
            Tri::Negative => {
                if !path_left_blocked((scroll_x - 1, scroll_y), background.tilemap) {
                    scroll_x -= 1
                }
            }
            _ => {}
        };

        match input.y_tri() {
            Tri::Positive => {
                if !path_down_blocked((scroll_x, scroll_y + 1), background.tilemap) {
                    scroll_y += 1
                }
            }
            Tri::Negative => {
                if !path_up_blocked((scroll_x, scroll_y - 1), background.tilemap) {
                    scroll_y -= 1
                }
            }
            _ => {}
        };
        // char.process(&input);
        // char.tick(&gfx);

        background.bg.set_scroll_pos((scroll_x, scroll_y));
        background.bg.commit(&mut vram);

        gfx.commit();
    }
}

fn path_right_blocked((scroll_x, scroll_y): (i16, i16), tilemap: [[usize; 32]; 32]) -> bool {
    let init_pos_x = INITIAL_COORDINATES.0 as i16;
    let init_pos_y = INITIAL_COORDINATES.1 as i16;

    let cur_offset_x = init_pos_x + scroll_x;

    if cur_offset_x % 8 != 0 {
        return false;
    }

    let cur_tile = cur_offset_x / 8;
    let last_tile = cur_tile;
    let next_tile = cur_tile + 3;

    let cur_offset_y = init_pos_y + scroll_y;
    let cur_tile_y = cur_offset_y / 8;

    let (tile1_y, tile2_y, tile3_y) = (cur_tile_y + 1, cur_tile_y + 2, cur_tile_y + 3);

    tilemap[tile1_y as usize][next_tile as usize] != 5
        || tilemap[tile2_y as usize][next_tile as usize] != 5
        || tilemap[tile3_y as usize][next_tile as usize] != 5
}

fn path_left_blocked((pos_x, pos_y): (i16, i16), tilemap: [[usize; 32]; 32]) -> bool {
    let init_pos_x = INITIAL_COORDINATES.0 as i16;
    let init_pos_y = INITIAL_COORDINATES.1 as i16;

    let cur_offset_x = init_pos_x + pos_x;

    if cur_offset_x % 8 != 0 {
        return false;
    }

    let cur_tile = cur_offset_x / 8;
    let last_tile = cur_tile;
    let next_tile = cur_tile + 4;

    let cur_offset_y = init_pos_y + pos_y;
    let cur_tile_y = cur_offset_y / 8;

    let (tile1_y, tile2_y, tile3_y) = (cur_tile_y + 1, cur_tile_y + 2, cur_tile_y + 3);

    tilemap[tile1_y as usize][last_tile as usize] != 5
        || tilemap[tile2_y as usize][last_tile as usize] != 5
        || tilemap[tile3_y as usize][last_tile as usize] != 5
}

fn path_up_blocked((pos_x, pos_y): (i16, i16), tilemap: [[usize; 32]; 32]) -> bool {
    let init_pos_x = INITIAL_COORDINATES.0 as i16;
    let init_pos_y = INITIAL_COORDINATES.1 as i16;

    let cur_offset_y = init_pos_y + pos_y;

    if cur_offset_y % 8 != 0 {
        return false;
    }

    let cur_tile = cur_offset_y / 8;
    let last_tile = cur_tile;
    let next_tile = cur_tile + 4;

    let cur_offset_x = init_pos_x + pos_x;
    let cur_tile_x = cur_offset_x / 8;

    let (tile1_x, tile2_x, tile3_x) = (cur_tile_x + 1, cur_tile_x + 2, cur_tile_x + 3);

    tilemap[last_tile as usize][tile1_x as usize] != 5
        || tilemap[last_tile as usize][tile2_x as usize] != 5
        || tilemap[last_tile as usize][tile3_x as usize] != 5
}

fn path_down_blocked((pos_x, pos_y): (i16, i16), tilemap: [[usize; 32]; 32]) -> bool {
    let init_pos_x = INITIAL_COORDINATES.0 as i16;
    let init_pos_y = INITIAL_COORDINATES.1 as i16;

    let cur_offset_y = init_pos_y + pos_y;

    if cur_offset_y % 8 != 0 {
        return false;
    }

    let cur_tile = cur_offset_y / 8;
    let last_tile = cur_tile;
    let next_tile = cur_tile + 3;

    let cur_offset_x = init_pos_x + pos_x;
    let cur_tile_x = cur_offset_x / 8;

    let (tile1_x, tile2_x, tile3_x) = (cur_tile_x + 1, cur_tile_x + 2, cur_tile_x + 3);

    tilemap[next_tile as usize][tile1_x as usize] != 5
        || tilemap[next_tile as usize][tile2_x as usize] != 5
        || tilemap[next_tile as usize][tile3_x as usize] != 5
}
