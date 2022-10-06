const res = await fetch("https://jsonplaceholder.typicode.com/todos/1");
console.log(await res.json())