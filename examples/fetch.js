Deno.core.print("starting to fetch...");
const res = await fetch("https://www.baidu.com");
Deno.core.print(`status: ${res.status}\n`);
Deno.core.print(`statusText: ${res.statusText}\n`);
Deno.core.print(`headers: ${res.headers}\n`);
