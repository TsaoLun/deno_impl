const STR = "benchmark";
const VALUES = Array.from({ length: 100000 }, (_) => STR);
function genStr(str, values) {
  const executor = {
    str,
    values,
    exec: (handler) => {
      return handler(executor.str, executor.values);
    },
  };
  return executor;
}

const rsStringBuilder = (str, values) => {
  return stringBuilder(str, values);
};

const start = new Date().getTime();

for (let i = 0; i < 100; i++){
  genStr(STR, VALUES).exec(rsStringBuilder);
}

Deno.core.print(`${(new Date().getTime()-start).toString()} ms\n`);