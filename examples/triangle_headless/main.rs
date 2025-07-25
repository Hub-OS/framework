mod triangle;

use framework::logging::*;
use framework::prelude::*;
use framework_core::runtime::HeadlessGameLoop;
use triangle::{Triangle, TriangleInstanceData, TriangleVertex};

fn main() -> anyhow::Result<()> {
    default_logger::init!();

    let game = Game::<HeadlessGameLoop>::new("Triangle Headless Example", (800, 600));

    game.run(MainScene::new)
}

struct MainScene {
    render_pipeline: RenderPipeline<TriangleVertex, TriangleInstanceData>,
    triangle: Triangle,
    next_scene: NextScene,
}

impl MainScene {
    fn new(game_io: &mut GameIO) -> MainScene {
        let graphics = game_io.graphics();

        let shader = graphics
            .load_shader_from_descriptor(include_wgsl!("triangle.wgsl"))
            .unwrap();

        let render_pipeline = RenderPipelineBuilder::new(game_io)
            .with_vertex_shader(&shader, "vs_main")
            .with_fragment_shader(&shader, "fs_main")
            .build::<TriangleVertex, TriangleInstanceData>()
            .unwrap();

        MainScene {
            render_pipeline,
            triangle: Triangle::new(game_io),
            next_scene: NextScene::None,
        }
    }
}

impl Scene for MainScene {
    fn next_scene(&mut self) -> &mut NextScene {
        &mut self.next_scene
    }

    fn update(&mut self, _game_io: &mut GameIO) {
        log::info!("updating");
    }

    fn draw(&mut self, game_io: &mut GameIO, render_pass: &mut RenderPass) {
        let mut render_queue = RenderQueue::new(game_io, &self.render_pipeline, []);
        render_queue.draw_model(&self.triangle);
        render_pass.consume_queue(render_queue);

        log::info!("drawing");
    }
}
