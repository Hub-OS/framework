mod main_scene;
use framework::runtime::GameWindowLoop;
use main_scene::MainScene;

use framework::logging::*;
use framework::prelude::*;

pub type GameLoop = WinitGameLoop;
pub type PlatformApp = <GameLoop as GameWindowLoop>::PlatformApp;

pub fn shared_main(platform_app: PlatformApp) -> anyhow::Result<()> {
    std::panic::set_hook(panic_hook());
    default_logger::init!();

    Game::<GameLoop>::new("Multiplatform", (800, 600))
        .with_platform_app(platform_app)
        .run(MainScene::new)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn sync_main() {
    shared_main(PlatformApp::default()).unwrap();
}

#[cfg(target_os = "android")]
#[no_mangle]
pub fn android_main(app: PlatformApp) {
    shared_main(app).unwrap();
}
