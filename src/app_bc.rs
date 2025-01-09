use agb::{
    display::object::{OamManaged, Object, Tag},
    input::{ButtonController, Tri},
    interrupt::VBlank,
    Gba,
};

use crate::tiles::{CHAR_BACK, CHAR_FRONT, CHAR_LEFT, CHAR_RIGHT};

const INITIAL_COORDINATES: (i32, i32) = (104, 64);
const STEP: i32 = 32;

#[derive(Clone, Copy)]
enum MovementType {
    Up,
    Down,
    Left,
    Right,
}

struct Movement {
    tp: MovementType,
    step: i32,
}

impl Movement {
    fn new(tp: MovementType, step: i32) -> Self {
        Self { tp, step }
    }
}

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
    movement: Option<Movement>,
}

impl<'a> Entity<'a> for Char<'a> {
    fn new(tag: &'static Tag, gfx: &'a OamManaged) -> Self {
        let frame = 0;

        Self {
            tag,
            frame,
            sprite: gfx.object_sprite(tag.animation_sprite(frame)),
            movement: None,
        }
    }

    fn place(&mut self, (x, y): (i32, i32)) {
        self.sprite.set_position((x, y)).show();
    }

    fn process(&mut self, input: &ButtonController) {
        if let Some(_) = self.movement {
            return;
        }

        match input.x_tri() {
            Tri::Positive => self.movement = Some(Movement::new(MovementType::Right, STEP)),
            Tri::Negative => self.movement = Some(Movement::new(MovementType::Left, STEP)),
            _ => {}
        }

        match input.y_tri() {
            Tri::Positive => self.movement = Some(Movement::new(MovementType::Down, STEP)),
            Tri::Negative => self.movement = Some(Movement::new(MovementType::Up, STEP)),
            _ => {}
        }
    }

    fn tick(&mut self, gfx: &'a OamManaged) {
        match &self.movement {
            Some(movement) => {
                match movement.tp {
                    MovementType::Up => {
                        self.sprite.set_y(self.sprite.y() - 1);
                        self.tag = CHAR_BACK;
                    }
                    MovementType::Down => {
                        self.sprite.set_y(self.sprite.y() + 1);
                        self.tag = CHAR_FRONT
                    }
                    MovementType::Left => {
                        self.sprite.set_x(self.sprite.x() - 1);
                        self.tag = CHAR_LEFT;
                    }
                    MovementType::Right => {
                        self.sprite.set_x(self.sprite.x() + 1);
                        self.tag = CHAR_RIGHT;
                    }
                };

                self.sprite
                    .set_sprite(gfx.sprite(self.tag.animation_sprite(self.frame)));

                self.frame = if self.frame > self.tag.sprites().len() {
                    0
                } else {
                    self.frame + 1
                };

                if movement.step == 0 {
                    self.movement = None;
                    return;
                }

                self.movement = Some(Movement::new(movement.tp, movement.step - 1));
            }
            _ => {}
        }
    }
}

pub fn run(mut gba: Gba) -> ! {
    let vblank = VBlank::get();
    let mut input = ButtonController::new();

    let gfx = gba.display.object.get_managed();

    let mut char = Char::new(CHAR_FRONT, &gfx);
    char.place(INITIAL_COORDINATES);

    let mut count = 0;

    loop {
        vblank.wait_for_vblank();
        input.update();
        char.tick(&gfx);

        if count % 5 == 0 {
            char.process(&input);
            char.tick(&gfx);

            count = 0
        }

        count += 1;

        gfx.commit();
    }
}
