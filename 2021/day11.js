var input = document.body.innerText.trim().split('\n').map((s) => s.split('').map((s) => parseInt(s)));
function step(state) {
    var flashes = 0;
    var newState = state.map((row) => [...row]);
    var flashed = {};
    var flash = (x, y) => {
        if (x >= 0 && x < newState.length && y >= 0 && y < newState[x].length && !flashed[[x, y]] && ++newState[x][y] == 10) {
            newState[x][y] = 0;
            flashed[[x, y]] = true;
            flashes++;
            [[-1, -1], [-1, 0], [-1, 1], [0, -1], [0, 1], [1, -1], [1, 0], [1, 1]].forEach((d) => flash(x + d[0], y + d[1]));
        }
    };
    state.forEach((row, x) => row.forEach(($, y) => flash(x, y)));
    return {next: newState, flashes: flashes};
};
var next = inputRaw, flashes;
var flashed = 0;
for (var i = 0; i < 100; i++) {
    ({next, flashes} = step(next));
    flashed += flashes;
};
console.log("Part 1:", flashed);
var next = inputRaw, flashes;
for (var i = 0; ; i++) {
    ({next, flashes} = step(next));
    if (flashes == 100) break;
}
console.log("Part 2:", i + 1);
