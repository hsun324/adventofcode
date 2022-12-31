var input = document.body.innerText.trim().split('\n').map((s) => {
    var p = s.split('');
    var cons = [];
    while (p.length) {
        var c = p.shift();
        if (c == 'n' || c == 's') cons.push(c + p.shift());
        if (c == 'w' || c == 'e') cons.push(c);
    }
    return cons;
});
var axial = {
    'nw': [-1, -1],
    'ne': [-1, 0],
    'w': [0, -1],
    'e': [0, 1],
    'sw': [1, 0],
    'se': [1, 1]
};
var flips = input.map((dirs) => dirs.reduce((p, dir) => [p[0] + axial[dir][0], p[1] + axial[dir][1]], [0, 0]));
var state = Object.values(flips.reduce((a, p) => { a[p] = (a[p] || [p, 0]); a[p][1]++; return a; }, {})).filter((tile) => tile[1] % 2 == 1).map((tile) => tile[0]);
var result = state.length;
console.log("Part 1:", result);
var step = (state) => {
    var adj = {};
    state.forEach((tile) => Object.values(axial).forEach((off) => {
        var key = [tile[0] + off[0], tile[1] + off[1]];
        adj[key] = (adj[key] || [key, 0]);
        adj[key][1]++;
    }));
    var sel = Object.fromEntries(state.map((tile) => [tile, true]));
    var keep = state.filter((tile) => adj[tile] && (adj[tile][1] == 1 || adj[tile][1] == 2));
    var enable = Object.values(adj).filter((ops) => !sel[ops[0]] && ops[1] == 2).map((ops) => ops[0]);
    return [...keep, ...enable];
};
var final = Array(100).fill().reduce(step, state);
console.log("Part 2:", final.length);
