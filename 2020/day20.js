var input = document.body.innerText.trim().split('\n\n').map((s) => {
    var sp = s.split('\n');
    var tile = parseInt(sp[0].slice(5, 9));
    var grid = sp.slice(1).map((s) => s.split('').map((c) => c == '#' ? '1' : '0'));
    return [tile, grid];
}).reduce((a, p) => { a[p[0]] = p[1]; return a; }, []);
var num = input.reduce((c) => c + 1, 0);
var dim = Math.sqrt(num);
var sdim = input.at(-1).length;
var perms = [
    (grid, y, x) => grid[y][x],                                     // normal      ( y,  x)
    (grid, y, x) => grid[grid.length - 1 - x][y],                   // rotate 90   (-x,  y)
    (grid, y, x) => grid[grid.length - 1 - y][grid.length - 1 - x], // rotate 180  (-y, -x)
    (grid, y, x) => grid[x][grid.length - 1 - y],                   // rotate 270  ( x, -y)
    (grid, y, x) => grid[grid.length - 1 - y][x],                   // flip y      (-y,  x)
    (grid, y, x) => grid[y][grid.length - 1 - x],                   // flip x      ( y, -x)
    (grid, y, x) => grid[x][y],                                     // flip y = x  ( x,  y)
    (grid, y, x) => grid[grid.length - 1 - x][grid.length - 1 - y]  // flip y = -x (-x, -y)
];
var edges = [
    (grid, perm) => grid[0].map(($, x) => perm(grid, 0, x)),               // top
    (grid, perm) => grid[0].map(($, x) => perm(grid, grid.length - 1, x)), // bottom
    (grid, perm) => grid[0].map(($, y) => perm(grid, y, 0)),               // left
    (grid, perm) => grid[0].map(($, y) => perm(grid, y, grid.length - 1))  // right
];
var all = input.flatMap(($, id) => perms.map(($, perm) => [id, perm]));
var bordersIndex = input.map((grid) => perms.map((perm) => edges.map((border) => parseInt(border(grid, perm).join('')))));
var bordersEdge = bordersIndex.reduce((acc, perms, id) => perms.reduce((acc, borders, perm) => borders.reduce((acc, border, edge) => {
    var key = [edge, border];
    acc[edge] = (acc[edge] || []);
    acc[edge][border] = (acc[edge][border] || []);
    acc[edge][border].push([id, perm]);
    return acc;
}, acc), acc), []);
var search = () => {
    var grid = Array(dim).fill().map(() => Array(dim).fill());
    var searchImpl = (x, y, grid, selected) => {
        var candidates = all;
        if (y > 0) {
            var neighbor = grid[y - 1][x];
            var border = bordersIndex[neighbor[0]][neighbor[1]][1];
            candidates = bordersEdge[0][border].filter((l) => candidates.findIndex((r) => l[0] == r[0] && l[1] == r[1]) >= 0);
        }
        if (x > 0) {
            var neighbor = grid[y][x - 1];
            var border = bordersIndex[neighbor[0]][neighbor[1]][3];
            candidates = bordersEdge[2][border].filter((l) => candidates.findIndex((r) => l[0] == r[0] && l[1] == r[1]) >= 0);
        }
        candidates = candidates.filter((candidate) => !selected.has(candidate[0]));
        candidates.forEach((candidate) => {
            grid[y][x] = candidate;
            selected.add(candidate[0]);

            var nx = x + 1, ny = y;
            if (nx >= dim) { nx = 0; ny++; }
            if (ny >= dim) throw "done";

            searchImpl(nx, ny, grid, selected);

            selected.delete(candidate[0]);
            grid[y][x] = undefined;
        });
    };
    try {
        searchImpl(0, 0, grid, new Set());
    } catch ($) {
        return grid;
    }
        
};
var match = search();
var result = match[0][0][0] * match[0].at(-1)[0] * match.at(-1)[0][0] * match.at(-1).at(-1)[0];
console.log("Part 1:", result);
var snake = "                  # " + "\n" +
            "#    ##    ##    ###" + "\n" +
            " #  #  #  #  #  #   ";
var ph = snake.split('\n').length, pw = snake.split('\n')[0].length;
var pattern = snake.split('\n').flatMap((s, y) => s.split('').flatMap((s, x) => s == '#' ? [[y, x]] : []));
var parts = match.map((row) => row.map((ip) => input[ip[0]].map((row, y) => row.map(($, x) => perms[ip[1]](input[ip[0]], y, x)).slice(1, -1)).slice(1, -1)));
var image = parts.flatMap((row) => row[0].map(($, y) => row.flatMap((part) => part[y])));
var points = {};
for (var y = 0; y < image.length - ph; y++)
    for (var x = 0; x < image[y].length - pw; x++)
        if (perms.some((perm) => pattern.every((p) => perm(image, y + p[0], x + p[1]) == '1')))
            pattern.forEach((p) => points[[y + p[0], x + p[1]]] = true);
var result = image.flat().filter((c) => c == '1').length - Object.keys(points).length;
console.log("Part 2:", result);
