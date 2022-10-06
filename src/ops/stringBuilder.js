((window) => {
  function stringBuilder(str, args) {
    return Deno.core.opSync("op_string_builder", str, args);
  }
  window.stringBuilder = stringBuilder;
  Deno.core.print("binded stringBuilder\n")
})(this);