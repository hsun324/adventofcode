var input = document.body.innerText.trim().split('\n\n');
var lookup = input[0].split('\n').join('');
var grid = input[1].split('\n').map((s) => s.split(''));
var dirs = [-1, 0, 1].flatMap((x) => [-1, 0, 1].map((y) => [x, y]));
var outer = '.';
var step = (grid, outer) => {
    var newGrid = Array(grid.length + 2).fill().map(($, y) => Array(grid[0].length + 2).fill().map(($, x) => {
        var ox = x - 1, oy = y - 1;
        var get = (x, y) => x >= 0 && y >= 0 && y < grid.length && x < grid[y].length ? grid[y][x] : outer;
        var idx = parseInt(dirs.map((p) => get(ox + p[1], oy + p[0]) == '#' ? '1' : '0').join(''), 2);
        var next = lookup[idx];
        if (!lookup[idx]) throw "fail";
        return next;
    }));
    var newOuter = lookup[parseInt((outer == '#' ? '1' : '0').repeat(9), 2)];
    return [newGrid, newOuter];
};
var start = performance.now();
var [grid1, outer1] = step(grid, outer);
var [grid2, outer2] = step(grid1, outer1);
var result = grid2.flat().filter((x) => x == '#').length;
console.log("Part 1:", result, "-", (performance.now() - start).toFixed(2) + "ms");
var [gx, os] = [grid, outer];
for (var i = 0; i < 50; i++)
    [gx, os] = step(gx, os);
var result = gx.flat().filter((x) => x == '#').length;
console.log("Part 2:", result, "-", (performance.now() - start).toFixed(2) + "ms");
