var input = document.body.innerText.trim().split('\n').map((s) => parseInt(s));
var prime = 20201227;
var inverse = Array(prime);
for (var v = 1, i = 0; i < prime; i++, v = (v * 7) % prime)
    inverse[v] = i;
for (var v = 1, i = 0; i < inverse[input[1]]; i++, v = (v * input[0]) % prime);
console.log("Part 1:", v);
