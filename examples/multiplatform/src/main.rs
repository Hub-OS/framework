use framework::prelude::PlatformApp;

fn main() -> anyhow::Result<()> {
    multiplatform::shared_main(PlatformApp::default())
}
