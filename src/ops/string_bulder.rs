use deno_core::{
    op, Extension,
    include_js_files
    //serde::Serialize
};

// #[derive(Serialize)]
// pub struct Res (String);


#[op]
fn op_string_builder(str: String, args: Vec<String>) -> String {
    print!("x");
    let res = str + &args.join("");
    res
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