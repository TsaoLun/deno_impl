use deno_core::{
    op, Extension,
    include_js_files
};

#[op]
fn op_string_builder(str: String, args: Vec<String>) -> String {
    str + &args.join("")
}

pub fn init() -> Extension {
    Extension::builder()
        .js(include_js_files!(
            prefix "stringBuilder",
            "stringBuilder.js",
        ))
        .ops(vec![op_string_builder::decl()])
        .build()
}