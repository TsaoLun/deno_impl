import { print } from "./base.js";

async function hello() {
  return new Promise((res, _) => {
    print("Hello world!\n");
    res("Hello");
  });
}

await hello();