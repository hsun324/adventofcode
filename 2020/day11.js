var input = document.body.innerText.trim().split('\n').map((s) => s.split(''));
var wrapped = (grid, x, y) => x >= 0 && x < grid.length && y >= 0 && y < grid[x].length ? grid[x][y] : 'L';
var ec = [[-1, -1], [-1, 0], [-1, 1], [0, -1], [0, 1], [1, -1], [1, 0], [1, 1]];
var step = (state) => {
    return state.map((row, x) => row.map((v, y) => {
        var adj = ec.reduce((a, c) => a + (wrapped(state, x + c[0], y + c[1]) == '#' ? 1 : 0), 0);
        if (v == '.') return '.';
        if (v == '#') return adj >= 4 ? 'L' : '#';
        if (v == 'L') return adj == 0 ? '#' : 'L';
    }));
}
for (var cur = input, next = step(input); JSON.stringify(cur) !== JSON.stringify(next); cur = next, next = step(next));
console.log("Part 1:", cur.flat().filter((x) => x == '#').length);
var ns = input.map((row, x) => row.map((v, y) => ec.map((c) => {
    for (var d = 1; wrapped(input, x + d * c[0], y + d * c[1]) == '.'; d++);
    return [x + d * c[0], y + d * c[1]];
})));
var step = (state) => {
    return state.map((row, x) => row.map((v, y) => {
        var adj = ns[x][y].reduce((a, c) => a + (wrapped(state, c[0], c[1]) == '#' ? 1 : 0), 0);
        if (v == '.') return '.';
        if (v == '#') return adj >= 5 ? 'L' : '#';
        if (v == 'L') return adj == 0 ? '#' : 'L';
    }));
}
for (var cur = input, next = step(input); JSON.stringify(cur) !== JSON.stringify(next); cur = next, next = step(next));
console.log("Part 2:", cur.flat().filter((x) => x == '#').length);
