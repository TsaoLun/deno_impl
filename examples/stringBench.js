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
Deno.core.print(genStr(STR, VALUES).exec(rsStringBuilder))