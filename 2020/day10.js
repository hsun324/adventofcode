var input = document.body.innerText.trim().split('\n').map((s) => parseInt(s, 10));
var sorted = [...input].sort((a, b) => a - b);
var diff = [...sorted.map((x, i) => i == 0 ? x : x - sorted[i - 1]), 3];
var hist = diff.reduce((a, x) => { a[x] = (a[x] || 0) + 1; return a; }, {});
console.log("Part 1:", hist[1] * hist[3]);
var data = Array(sorted.length).fill(0);
data[data.length - 1] = 1;
for (var i = data.length - 2; i >= 0; i--)
    for (var j = i + 1; j < sorted.length && sorted[j] - sorted[i] <= 3; j++)
        data[i] += data[j];
console.log("Part 2:", data[0] + data[1] + data[2]);
