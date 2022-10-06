use std::rc::Rc;

use deno_core::{anyhow::Result, JsRuntime, RuntimeOptions, FsModuleLoader};
use deno_impl::{ops::fetch, execute_main_module};

#[tokio::main]
async fn main() -> Result<()> {
    let options = RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        extensions: vec![fetch::init()],
        ..Default::default()
    };
    let mut rt = JsRuntime::new(options);
    let js_file = format!("{}/examples/fetch.js", env!("CARGO_MANIFEST_DIR"));
    execute_main_module(&mut rt, js_file).await?;
    Ok(())
}