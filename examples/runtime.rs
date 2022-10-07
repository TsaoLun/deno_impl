use deno_core::resolve_url_or_path;
use deno_impl::MainWorkerOptions;
use deno_runtime::{deno_core::anyhow::Result, permissions::Permissions, worker::MainWorker};

#[tokio::main]
async fn main() -> Result<()> {
    let options = MainWorkerOptions::default();
    let js_file = format!("{}/examples/runtime.js", env!("CARGO_MANIFEST_DIR"));
    let url = resolve_url_or_path(&js_file)?;
    let permission = Permissions {
        net: Permissions::new_net(&Some(vec![]), false)?,
        ..Default::default()
    };
    let mut worker =
        MainWorker::bootstrap_from_options(url.clone(), permission, options.into_inner());
    worker.execute_main_module(&url).await?;
    Ok(())
}
