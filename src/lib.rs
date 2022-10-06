pub mod ops;

use std::{ops::Deref, rc::Rc, sync::Arc};

use deno_core::{
    anyhow::{Ok, Result},
    resolve_url_or_path,
    serde::de::DeserializeOwned,
    serde_v8, v8, JsRuntime, FsModuleLoader, error::AnyError,
};
use deno_runtime::{worker::WorkerOptions, BootstrapOptions, deno_web::BlobStore, deno_broadcast_channel::{BroadcastChannel, InMemoryBroadcastChannel}, ops::io::Stdio};

pub async fn eval<T>(rt: &mut JsRuntime, code: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let ret = rt.execute_script("<anon>", code)?;
    let result = rt.resolve_value(ret).await?;
    let scope = &mut rt.handle_scope();
    let result = v8::Local::new(scope, result);
    Ok(serde_v8::from_v8(scope, result)?)
}

pub async fn execute_main_module(rt: &mut JsRuntime, path: impl AsRef<str>) -> Result<()> {
    let url = resolve_url_or_path(path.as_ref())?;
    let id = rt.load_main_module(&url, None).await?;
    let mut receiver = rt.mod_evaluate(id);
    tokio::select! {
        resolved = &mut receiver => {
            resolved.expect("failed to evaluate module")
        }
        _ = rt.run_event_loop(false) => {
            receiver.await.expect("failed to evaluate module")
        }
    }
}

pub struct MainWorkerOptions(WorkerOptions);

impl Deref for MainWorkerOptions {
    type Target = WorkerOptions;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for MainWorkerOptions {
    fn default() -> Self {
        let bootstrap = BootstrapOptions{
            args: vec![],
            cpu_count: 1,
            debug_flag: false,
            enable_testing_features: false,
            location: None,
            no_color: false,
            is_tty: false,
            runtime_version: "x".to_string(),
            ts_version: "x".to_string(),
            unstable: false,
            user_agent: "my_runtime".to_string(),
            inspect: false
        };
        let create_web_worker_cb = Arc::new(|_|{
            todo!("not supported yet")
        });
        let web_worker_preload_module_cb = Arc::new(|_|{
            todo!("not supported yet")
        });
        let web_worker_pre_execute_module_cb = Arc::new(|_|{
            todo!("not supported yet")
        });
        Self(WorkerOptions {
            bootstrap,
            extensions: vec![],
            unsafely_ignore_certificate_errors: None,
            root_cert_store: None,
            seed: None,
            module_loader: Rc::new(FsModuleLoader),
            npm_resolver: None,
            create_web_worker_cb,
            web_worker_preload_module_cb,
            web_worker_pre_execute_module_cb,
            format_js_error_fn: None,
            source_map_getter: None,
            maybe_inspector_server: None,
            should_break_on_first_statement: false,
            get_error_class_fn: Some(&get_error_class_fn),
            cache_storage_dir: None,
            origin_storage_dir: None,
            blob_store: BlobStore::default(),
            broadcast_channel: InMemoryBroadcastChannel::default(),
            shared_array_buffer_store: None,
            compiled_wasm_module_store: None,
            stdio: Stdio::default(),
        })
    }
}

fn get_error_class_fn(e: &AnyError) -> &'static str {
    deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}

impl MainWorkerOptions {
    pub fn into_inner(self) -> WorkerOptions {
        self.0
    }
}