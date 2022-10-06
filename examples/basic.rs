use std::rc::Rc;
use deno_impl::execute_main_module;
use deno_core::{
    anyhow::{Result, Ok}, JsRuntime,
    RuntimeOptions, FsModuleLoader,
};
#[tokio::main]
async fn main() -> Result<()> {
    // let options: RuntimeOptions = RuntimeOptions::default();
    let options = RuntimeOptions{module_loader: Some(Rc::new(FsModuleLoader)), ..Default::default()};
    let mut rt = JsRuntime::new(options);
    // let code: &str = include_str!("basic.js");
    // let result: String = eval(&mut rt, code).await?;
    // print!("Rust Output: {:?}\n", result);
    let path: String = format!("{}/examples/basic_module.js", env!("CARGO_MANIFEST_DIR"));
    execute_main_module(&mut rt, &path).await?;
    Ok(())
}