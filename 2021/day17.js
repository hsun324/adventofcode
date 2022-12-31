var input = Array.from(document.body.innerText.match(/target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)/)).slice(1).map((s) => parseInt(s));
var xmin = input[0], xmax = input[1], ymin = input[2], ymax = input[3];
var check = (dx, dy) => {
    for (var x = 0, y = 0, peak = 0; dy >= 0 || y >= ymin; x += dx, y += dy, dx -= Math.sign(dx), dy--) {
        if (y > peak) peak = y;
        if (x >= xmin && x <= xmax && y >= ymin && y <= ymax) return peak;
    }
};
var best = 0, count = 0;
for (var dx = 0; dx <= xmax; dx++) {
    for (var dy = ymin; dy <= 500; dy++) {
        var result = check(dx, dy);
        if (result !== undefined) count++;
        if (result !== undefined && result > best) best = result;
    }
}
console.log("Part 1:", best);
console.log("Part 2:", count);
