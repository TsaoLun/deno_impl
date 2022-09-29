use deno_core::{anyhow::Result, serde_v8, v8, JsRuntime, RuntimeOptions};
#[tokio::main]
async fn main() -> Result<()> {
    let options: RuntimeOptions = RuntimeOptions::default();
    let mut rt: JsRuntime = JsRuntime::new(options);
    let code: &str = include_str!("../examples/hello.js");
    let ret = rt.execute_script("<hello>", code)?;
    let result = rt.resolve_value(ret).await?;
    let scope = &mut rt.handle_scope();
    let result = v8::Local::new(scope, result);
    let result: String = serde_v8::from_v8(scope, result)?;
    print!("Rust Output: {:?}\n", result);
    Ok(())
}
