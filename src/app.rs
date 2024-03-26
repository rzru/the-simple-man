use core::usize;

use agb::{
    display::object::{OamManaged, Object, Tag},
    input::{Button, ButtonController},
    interrupt::VBlank,
    Gba,
};

use crate::tiles::{CHAR_BACK, CHAR_FRONT, CHAR_LEFT, CHAR_RIGHT};

struct Char<'a> {
    sprite: Option<Object<'a>>,
    current: (&'static Tag, usize),
    current_idx: usize,
}

impl<'a> Char<'a> {
    fn new() -> Self {
        Self {
            sprite: None,
            current: (CHAR_FRONT, 8),
            current_idx: 0,
        }
    }

    fn init(&mut self, gfx: &'a OamManaged) {
        let (tag, _) = self.current;
        let sprite = tag.animation_sprite(self.current_idx);

        let mut sprite = gfx.object_sprite(sprite);
        sprite.set_x(104).set_y(64).show();

        self.sprite = Some(sprite);
    }

    fn tick(&mut self, input: &ButtonController, gfx: &'a OamManaged) {
        let mut animate = false;

        if input.is_pressed(Button::DOWN) {
            self.current = (CHAR_FRONT, 8);
            animate = true
        }

        if input.is_pressed(Button::UP) {
            self.current = (CHAR_BACK, 8);
            animate = true
        }

        if input.is_pressed(Button::RIGHT) {
            self.current = (CHAR_RIGHT, 4);
            animate = true
        }

        if input.is_pressed(Button::LEFT) {
            self.current = (CHAR_LEFT, 4);
            animate = true
        }

        if animate {
            let (tag, frames) = self.current;

            self.current_idx = if self.current_idx > frames {
                0
            } else {
                self.current_idx + 1
            };

            self.sprite
                .as_mut()
                .unwrap()
                .set_sprite(gfx.sprite(tag.animation_sprite(self.current_idx)));
        }
    }
}

pub fn run(mut gba: Gba) -> ! {
    let gfx = gba.display.object.get_managed();

    let mut main_character = Char::new();
    main_character.init(&gfx);

    let vblank = VBlank::get();
    let mut input = ButtonController::new();
    let mut count = 0;

    loop {
        vblank.wait_for_vblank();
        input.update();

        count += 1;

        if count % 5 == 0 {
            main_character.tick(&input, &gfx);
            count = 0
        }

        gfx.commit();
    }
}
