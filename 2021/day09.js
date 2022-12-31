var inputRaw = document.body.innerText.trim().split('\n').map((s) => s.split('').map((s2) => parseInt(s2, 10)))
var input = (x, y) => x >= 0 && x < inputRaw.length && y >= 0 && y < inputRaw[0].length ? inputRaw[x][y] : 9;
console.log("Part 1:", inputRaw.flatMap((row, x) => row.filter((v, y) => v < Math.min(input(x + 1, y), input(x - 1, y), input(x, y + 1), input(x, y - 1)))).reduce((a, b) => a + b + 1, 0));
var basin = inputRaw.map((v) => v.map((x) => -1));
var index = 0;
function recurse(x, y) {
    if (input(x, y) == 9) return -1;
    if (basin[x][y] != -1) return basin[x][y];
    if (input(x - 1, y) < input(x, y)) return basin[x][y] = recurse(x - 1, y);
    if (input(x + 1, y) < input(x, y)) return basin[x][y] = recurse(x + 1, y);
    if (input(x, y - 1) < input(x, y)) return basin[x][y] = recurse(x, y - 1);
    if (input(x, y + 1) < input(x, y)) return basin[x][y] = recurse(x, y + 1);
    return basin[x][y] = index++;
}
inputRaw.map((row, x) => row.map((v, y) => recurse(x, y)));
var freq = basin.flat().reduce((a, b) => { if (b >= 0) a[b] = (a[b] || 0) + 1; return a }, []).sort((a, b) => a - b).slice(-3);
console.log("Part 2:", freq[0] * freq[1] * freq[2]);
