mod main_scene;
use main_scene::MainScene;

use framework::logging::*;
use framework::prelude::*;

pub fn shared_main(platform_app: WinitPlatformApp) -> anyhow::Result<()> {
    default_logger::init!();

    Game::<WinitGameLoop>::new("Multiplatform", (800, 600))
        .with_platform_app(platform_app)
        .run(MainScene::new)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn sync_main() {
    shared_main(WinitPlatformApp::default()).unwrap();
}

#[cfg(target_os = "android")]
#[no_mangle]
pub fn android_main(app: WinitPlatformApp) {
    shared_main(app).unwrap();
}
