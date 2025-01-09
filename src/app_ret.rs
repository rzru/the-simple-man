use core::usize;

use agb::{
    display::{
        object::{OamManaged, Object, Tag},
        tiled::{RegularBackgroundSize, TiledMap},
        Priority,
    },
    include_background_gfx,
    input::{Button, ButtonController, Tri},
    interrupt::VBlank,
    mgba::Mgba,
    Gba,
};

use crate::{
    background::LEVEL_MAP,
    tiles::{CHAR_BACK, CHAR_FRONT, CHAR_LEFT, CHAR_RIGHT},
};

include_background_gfx!(test_bg, tiles => 256 "gfx/test-bg.png");

trait Entity<'a> {
    fn new(tag: &'static Tag, gfx: &'a OamManaged, coords: (i32, i32)) -> Self;
    fn tick(&mut self, input: &ButtonController, gfx: &'a OamManaged);
}

struct Char<'a> {
    sprite: Object<'a>,
    current: &'static Tag,
    current_idx: usize,
}

impl<'a> Entity<'a> for Char<'a> {
    fn new(tag: &'static Tag, gfx: &'a OamManaged, (x, y): (i32, i32)) -> Self {
        let sprite = tag.animation_sprite(0);

        let mut sprite = gfx.object_sprite(sprite);
        sprite.set_position((x, y)).show();

        Self {
            sprite,
            current: tag,
            current_idx: 0,
        }
    }

    fn tick(&mut self, input: &ButtonController, gfx: &'a OamManaged) {
        let mut animate = false;

        if input.is_pressed(Button::DOWN) {
            self.current = CHAR_FRONT;
            animate = true
        }

        if input.is_pressed(Button::UP) {
            self.current = CHAR_BACK;
            animate = true
        }

        if input.is_pressed(Button::RIGHT) {
            self.current = CHAR_RIGHT;
            animate = true
        }

        if input.is_pressed(Button::LEFT) {
            self.current = CHAR_LEFT;
            animate = true
        }

        if animate {
            self.current_idx = if self.current_idx > self.current.sprites().len() {
                0
            } else {
                self.current_idx + 1
            };

            self.sprite
                .set_sprite(gfx.sprite(self.current.animation_sprite(self.current_idx)));
        }
    }
}

struct StaticObj<'a> {
    sprite: Object<'a>,
    current: &'static Tag,
    current_idx: usize,
    coords: (i32, i32),
}

impl<'a> Entity<'a> for StaticObj<'a> {
    fn new(tag: &'static Tag, gfx: &'a OamManaged, (x, y): (i32, i32)) -> Self {
        let sprite = tag.animation_sprite(0);

        let mut sprite = gfx.object_sprite(sprite);
        sprite.set_position((x, y)).show();

        Self {
            sprite,
            current: tag,
            current_idx: 0,
            coords: (x, y),
        }
    }

    fn tick(&mut self, input: &ButtonController, gfx: &'a OamManaged) {
        match input.x_tri() {
            Tri::Negative => self.coords.0 += 2,
            Tri::Positive => self.coords.0 -= 2,
            Tri::Zero => match input.y_tri() {
                Tri::Negative => self.coords.1 += 2,
                Tri::Positive => self.coords.1 -= 2,
                Tri::Zero => {}
            },
        }

        self.current_idx = if self.current_idx > self.current.sprites().len() {
            0
        } else {
            self.current_idx + 1
        };

        self.sprite
            .set_sprite(gfx.sprite(self.current.animation_sprite(self.current_idx)));
        self.sprite.set_position(self.coords);
    }
}

pub fn run(mut gba: Gba) -> ! {
    let mut logger = Mgba::new();

    let gfx = gba.display.object.get_managed();

    let (bg_gfx, mut vram) = gba.display.video.tiled0();

    let tileset = &test_bg::tiles.tiles;
    vram.set_background_palettes(test_bg::PALETTES);

    let mut bg = bg_gfx.background(
        Priority::P0,
        RegularBackgroundSize::Background64x32,
        tileset.format(),
    );

    for y in 0..32u16 {
        for x in 0..64u16 {
            bg.set_tile(
                &mut vram,
                (x, y),
                &tileset,
                test_bg::tiles.tile_settings[LEVEL_MAP[y as usize][x as usize]],
            );
        }
    }

    bg.commit(&mut vram);
    bg.set_visible(true);

    let mut main_character = Char::new(CHAR_FRONT, &gfx, (104, 64));

    let vblank = VBlank::get();
    let mut input = ButtonController::new();
    let mut count = 0;

    let mut scroll_pos = (0, 0);

    loop {
        let _logger = logger.as_mut().unwrap();

        vblank.wait_for_vblank();
        input.update();

        if count % 5 == 0 {
            main_character.tick(&input, &gfx);

            bg.set_scroll_pos(scroll_pos);
            bg.commit(&mut vram);

            count = 0
        }

        let new_scroll_pos = (
            scroll_pos.0 + input.x_tri() as i16,
            scroll_pos.1 + input.y_tri() as i16,
        );

        let movement_blocked = check_boundary(new_scroll_pos, LEVEL_MAP);

        if !movement_blocked {
            scroll_pos = new_scroll_pos;
        }

        count += 1;

        gfx.commit();
    }
}

fn check_boundary((scroll_x, scroll_y): (i16, i16), level: [[usize; 64]; 32]) -> bool {
    let x_offset = scroll_x / 8;
    let y_offset = scroll_y / 8;

    let right = level[(10 + y_offset) as usize][(15 + x_offset) as usize];
    let left = level[(10 + y_offset) as usize][(14 + x_offset) as usize];

    let top = level[(12 + y_offset) as usize][(15 + x_offset) as usize];
    let bottom = level[(8 + y_offset) as usize][(15 + x_offset) as usize];

    return right == 1 || left == 1 || top == 1 || bottom == 1;
}
