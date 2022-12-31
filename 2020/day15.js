var input = document.body.innerText.trim().split(',').map((s) => parseInt(s, 10));
var simulate = (rounds) => {
    var spoken = input[input.length - 1];
    var hists = Array(rounds).fill([-1, -1]);
    input.forEach((n, i) => hists[n] = [-1, i + 1]);
    for (var t = input.length + 1; t <= rounds; t++) {
        var last = hists[spoken];
        var say = last[0] != -1 ? last[1] - last[0] : 0;
        spoken = say;
        hists[say] = [(hists[say] || [-1, -1])[1], t];
    }
    return spoken;
};
console.log("Part 1:", simulate(2020));
console.log("Part 1:", simulate(30000000));
