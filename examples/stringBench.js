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

const builder = stringBuilder

const rsStringBuilder = (str, values) => {
  return builder(str, values);
};

const start = new Date().getTime();

genStr(STR, VALUES).exec(rsStringBuilder);

Deno.core.print((new Date().getTime()-start).toString()+"\n");