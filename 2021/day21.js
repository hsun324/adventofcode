var input = document.body.innerText.trim().split('\n').map((s) => parseInt(s.slice(-2)));
var start = performance.now();
var rolled = 0;
var roll = () => { return ++rolled; }
var p1s = 0, p2s = 0;
var p1p = input[0], p2p = input[1];
while (p1s < 1000 && p2s < 1000) {
    var p1r = roll() + roll() + roll();
    var p1p = ((p1p - 1) + p1r) % 10 + 1;
    p1s += p1p;
    if (p1s >= 1000) break;
    var p2r = roll() + roll() + roll();
    var p2p = ((p2p - 1) + p2r) % 10 + 1;
    p2s += p2p;
}
var result = rolled * Math.min(p1s, p2s)
console.log("Part 1:", result, "-", (performance.now() - start).toFixed(2) + "ms");
var step = (state, win) => {
    var next = {};
    Object.entries(state).forEach((s) => {
        var [key, [[p1p, p2p, p1s, p2s], f]] = s;
        var freq = [[3, 1], [4, 3], [5, 6], [6, 7], [7, 6], [8, 3], [9, 1]];
        freq.forEach((p1roll) => {
            var [p1r, p1rollf] = p1roll;
            var p1np = ((p1p - 1 + p1r) % 10) + 1;
            var p1ns = p1s + p1np;
            if (p1ns >= 21) win[0] = (win[0] || 0) + f * p1rollf;
            else {
                freq.forEach((p2roll) => {
                    var [p2r, p2rollf] = p2roll;
                    var p2np = ((p2p - 1 + p2r) % 10) + 1;
                    var p2ns = p2s + p2np;
                    if (p2ns >= 21) win[1] = (win[1] || 1) + f * p1rollf * p2rollf;
                    else {
                        // console.log("add state p1 rolls " + p1roll + " and p2 rolls " + p2roll + " | p1np: " + p1np + ", p1ns: " + p1s + " -> " + p1ns + ", p2np: " + p2np + ", p2ns: " + p2s + " -> " + p2ns);
                        var nkey = [p1np, p2np, p1ns, p2ns];
                        next[nkey] = (next[nkey] || [nkey, 0]);
                        next[nkey][1] += f * p1rollf * p2rollf;
                    }
                });
            }
        });
    });
    return next;
};
var start = performance.now();
var ikey = [input[0], input[1], 0, 0]
var initial = {};
initial[ikey] = [ikey, 1];
var win = [];
var state = JSON.parse(JSON.stringify(initial));
while (Object.keys(state).length)
    state = step(state, win);
var result = Math.max(...win);
console.log("Part 2:", result, "-", (performance.now() - start).toFixed(2) + "ms");
