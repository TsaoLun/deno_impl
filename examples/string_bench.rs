use std::rc::Rc;

use deno_core::{anyhow::Result, FsModuleLoader, JsRuntime, RuntimeOptions};
use deno_impl::{execute_main_module, ops::string_bulder};

#[tokio::main]
async fn main() -> Result<()> {
    let options = RuntimeOptions {
        module_loader: Some(Rc::new(FsModuleLoader)),
        extensions: vec![string_bulder::init()],
        ..Default::default()
    };
    let mut rt = JsRuntime::new(options);
    let js_file = format!("{}/examples/stringBench.js", env!("CARGO_MANIFEST_DIR"));
    execute_main_module(&mut rt, js_file).await?;
    Ok(())
}
