#[cfg(not(target_os = "android"))]
fn main() -> anyhow::Result<()> {
    use framework::prelude::WinitPlatformApp;

    multiplatform::shared_main(WinitPlatformApp::default())
}
