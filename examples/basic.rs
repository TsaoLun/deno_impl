use std::rc::Rc;

use deno_core::{
    anyhow::Result, resolve_url_or_path, serde::de::DeserializeOwned, serde_v8, v8, JsRuntime,
    RuntimeOptions, FsModuleLoader,
};
#[tokio::main]
async fn main() -> Result<()> {
    // let options: RuntimeOptions = RuntimeOptions::default();
    let options = RuntimeOptions{module_loader: Some(Rc::new(FsModuleLoader)), ..Default::default()};
    let mut rt: JsRuntime = JsRuntime::new(options);
    // let code: &str = include_str!("basic.js");
    // let result: String = eval(&mut rt, code).await?;
    // print!("Rust Output: {:?}\n", result);
    let path: String = format!("{}/examples/basic_module.js", env!("CARGO_MANIFEST_DIR"));
    let url = resolve_url_or_path(&path)?;
    let id = rt.load_main_module(&url, None).await?;
    let mut receiver = rt.mod_evaluate(id);
    tokio::select! {
        resolved = &mut receiver => {
            resolved.expect("failed to evaluate module")?;
        }
        _ = rt.run_event_loop(false) => {
            receiver.await.expect("failed to evaluate module")?;
        }
    }
    rt.mod_evaluate(id).await??;
    rt.run_event_loop(false).await?;
    Ok(())
}

#[allow(dead_code)]
async fn eval<T>(rt: &mut JsRuntime, code: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let ret = rt.execute_script("<anon>", code)?;
    let result = rt.resolve_value(ret).await?;
    let scope = &mut rt.handle_scope();
    let result = v8::Local::new(scope, result);
    Ok(serde_v8::from_v8(scope, result)?)
}
