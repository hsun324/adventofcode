var inputRaw = document.body.innerText.trim().split('\n\n');
var initial = inputRaw[0].split('');
var rules = Object.fromEntries(inputRaw[1].split('\n').map((s) => [s[0] + s[1], s[6]]));
var compute = (s, n) => Array(n).fill().reduce((state) => {
    var next = {};
    var freq = Object.assign({}, state.freq);
    Object.entries(state.pairs).forEach((pair) => {
        var key = pair[0], value = pair[1], insert = rules[key];
        next[key[0] + insert] = (next[key[0] + insert] || 0) + value;
        next[insert + key[1]] = (next[insert + key[1]] || 0) + value;
        freq[insert] = (freq[insert] || 0) + value;
    });
    return {pairs: next, freq: freq};
}, {
    pairs: s.slice(1).map((v, i) => s[i] + v).reduce((a, s) => { a[s] = (a[s] || 0) + 1; return a; }, {}),
    freq: s.reduce((a, c) => { a[c] = (a[c] || 0) + 1; return a; }, {})
});
var freq = Object.values(compute(initial, 10).freq).sort((a, b) => a - b);
console.log("Part 1:", freq[freq.length - 1] - freq[0]);
var freq = Object.values(compute(initial, 40).freq).sort((a, b) => a - b);
console.log("Part 2:", freq[freq.length - 1] - freq[0]);
