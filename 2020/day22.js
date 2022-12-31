var input = document.body.innerText.trim().split('\n\n').map((s) => s.split('\n').slice(1).map((s) => parseInt(s)));
var simulate = (game) => {
    while (game[0].length && game[1].length) {
        var a = game[0].shift(), b = game[1].shift();
        if (a > b) { game[0].push(a, b); }
        if (b > a) { game[1].push(b, a); }
    }
    return game;
}
var outcome = simulate(input.map((deck) => [...deck]));
var result = [...(outcome[0].length ? outcome[0] : outcome[1])].reverse().reduce((a, b, i) => a + b * (i + 1), 0);
console.log("Part 1:", result);
var simulate = (game) => {
    var seen = {};
    while (game[0].length && game[1].length) {
        if (seen[[game[0],"$",game[1]]]) return [[1], []];
        seen[[game[0],"$",game[1]]] = true;
        var a = game[0].shift(), b = game[1].shift();
        if (game[0].length >= a && game[1].length >= b) {
            var outcome = simulate([game[0].slice(0, a), game[1].slice(0, b)]);
            if (outcome[0].length) game[0].push(a, b);
            if (outcome[1].length) game[1].push(b, a);
        } else {
            if (a > b) game[0].push(a, b);
            if (b > a) game[1].push(b, a);
        }
    }
    return game;
}
var outcome = simulate(input.map((deck) => [...deck]));
var result = [...(outcome[0].length ? outcome[0] : outcome[1])].reverse().reduce((a, b, i) => a + b * (i + 1), 0);
console.log("Part 2:", result);
