((window) => {
  async function stringBuilder(str, ...args) {
    Deno.core.print(str)
    return await Deno.core.opSync("op_string_builder", str, args);
  }
  window.stringBuilder = stringBuilder;
  Deno.core.print("binded stringBuilder\n")
})(this);