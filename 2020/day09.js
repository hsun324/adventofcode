var input = document.body.innerText.trim().split('\n').map((s) => parseInt(s, 10));
var bad = input[input.findIndex((s, i) => i >= 25 && !input.slice(i - 25, i).find((x) => input.slice(i - 25, i).indexOf(s - x) != -1))];
console.log("Part 1:", bad);
var i = 0, j = 0, sum = input[0];
while (sum != bad)
    if (sum > bad) sum -= input[i++];
    else if (sum < bad) sum += input[++j];
var slice = input.slice(i, j + 1).sort((a, b) => a - b);
console.log("Part 2:", slice[0] + slice[slice.length - 1]);
