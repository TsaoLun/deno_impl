async function hello() {
  return new Promise((res, _) => {
    Deno.core.print("Hello world!\n");
    res("PRINT_SUCCESS");
  })
}

hello();