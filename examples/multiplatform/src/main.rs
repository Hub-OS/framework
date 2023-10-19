#[cfg(not(target_os = "android"))]
fn main() -> anyhow::Result<()> {
    multiplatform::shared_main(multiplatform::PlatformApp::default())
}
