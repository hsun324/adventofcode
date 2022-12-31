var input = document.body.firstChild.innerText.trim().split('\n').map((s) => s.split(''));
var step = (state) => {
    var ydim = state.length;
    var xdim = state[0].length;

    var moved = 0;
    var east = state.map((row) => [...row]);
    for (var y = 0; y < ydim; y++) {
        for (var x = 0; x < xdim; x++) {
            if (state[y][x] == '>' && state[y][(x + 1) % xdim] == '.') {
                east[y][x] = '.';
                east[y][(x + 1) % xdim] = '>';
                moved++;
            }
        }
    }

    var south = east.map((row) => [...row]);
    for (var y = 0; y < ydim; y++) {
        for (var x = 0; x < xdim; x++) {
            if (east[y][x] == 'v' && east[(y + 1) % ydim][x] == '.') {
                south[y][x] = '.';
                south[(y + 1) % ydim][x] = 'v';
                moved++;
            }
        }
    }

    return [moved, south];
};
var moved, state = input;
for (var i = 1; [moved, state] = step(state), moved; i++);
console.log("Part 1:", i);
