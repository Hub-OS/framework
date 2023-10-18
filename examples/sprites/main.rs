use framework::logging::*;
use framework::prelude::*;
use rand::prelude::*;

fn main() -> anyhow::Result<()> {
    default_logger::init!();

    let game = Game::<WinitGameLoop>::new("Sprites", (800, 600));

    game.run(MainScene::new)
}

struct MainScene {
    sprites: Vec<Sprite>,
    camera: OrthoCamera,
    next_scene: NextScene,
}

impl MainScene {
    fn new(game_io: &mut GameIO) -> MainScene {
        let mut camera = OrthoCamera::new(game_io, Vec2::new(800.0, 600.0));
        camera.invert_y(true);

        let texture = Texture::load_from_memory(game_io, include_bytes!("sprite.png")).unwrap();

        let mut sprites = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..500 {
            let mut sprite = Sprite::new(game_io, texture.clone());

            let camera_bounds = camera.bounds();

            sprite.set_position(Vec2::new(
                rng.gen_range(camera_bounds.horizontal_range()),
                rng.gen_range(camera_bounds.vertical_range()),
            ));

            sprite.set_rotation(rng.gen_range(0.0..std::f32::consts::PI * 2.0));

            sprite.set_origin(Vec2::new(0.5, 0.5));
            sprite.set_size(Vec2::new(60.0, 60.0));

            sprites.push(sprite);
        }

        MainScene {
            camera,
            sprites,
            next_scene: NextScene::None,
        }
    }
}

impl Scene for MainScene {
    fn next_scene(&mut self) -> &mut NextScene {
        &mut self.next_scene
    }

    fn update(&mut self, game_io: &mut GameIO) {
        let a = std::f32::consts::PI / 180.0 * 3.0;

        for sprite in &mut self.sprites {
            let rotation = sprite.rotation();
            sprite.set_rotation(rotation + a);
        }

        let mut camera_pos = self.camera.position();

        let input = game_io.input();

        camera_pos.x += input.controller_axis(0, AnalogAxis::LeftStickX)
            + input.keys_as_axis(Key::Left, Key::Right);

        camera_pos.y -= input.controller_axis(0, AnalogAxis::LeftStickY)
            + input.keys_as_axis(Key::Down, Key::Up);

        if input.was_button_just_pressed(0, Button::A) {
            input
                .controller(0)
                .unwrap()
                .rumble(1.0, 1.0, Duration::from_secs_f32(5.0));
        }

        self.camera.set_position(camera_pos);
    }

    fn draw(&mut self, game_io: &mut GameIO, render_pass: &mut RenderPass) {
        // self.camera.resize_to_window(window);
        self.camera.scale_to_window(game_io.window());

        let uniforms = [self.camera.as_binding()];
        let mut render_queue =
            SpriteQueue::new_with_default_pipeline(game_io, uniforms).with_inverted_y(true);

        for sprite in &self.sprites {
            render_queue.draw_sprite(sprite);
        }

        render_pass.consume_queue(render_queue);
    }
}
