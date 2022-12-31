var input = document.body.innerText.trim().split('\n').map((s) => parseInt(s, 10));
var val = input.find((x) => input.indexOf(2020 - x) != -1);
console.log("Part 1:", val * (2020 - val));
var twos = input.flatMap((x) => input.map((y) => x + y));
var val1 = input.find((x) => twos.indexOf(2020 - x) != -1);
var val2 = input.find((x) => input.indexOf(2020 - val1 - x) != -1);
console.log("Part 2:", val1 * val2 * (2020 - val1 - val2));
