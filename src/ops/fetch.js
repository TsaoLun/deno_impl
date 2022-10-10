((window) => {
  async function fetch(args) {
    const type = typeof args;
    if ( type === "string") {
      args = { url: args, method: "GET", headers: [], body: null}
    } else if ( type === "object" ){
      if (args.url) {
        args.method = args.method || "GET";
        args.headers = args.headers || [];
        args.body = args.body || null;
      } else {
        throw new Error("Invalid arguments");
      }
    } else {
      throw new Error("Invalid fetch args");
    }
    const res = await Deno.core.opAsync("op_fetch", args);
    res.text = () => {
      if (!res.body) {
        return null;
      }
      return Deno.core.opSync("op_decode_utf8", res.body);
    };
    res.json = () => {
      if (!res.text()) {
        return null;
      }
      return JSON.parse(res.text());
    };
  }
  window.fetch = fetch;
  Deno.core.print("binded op_fetch\n")
})(this);
