var convert = (snail) => {
    var result = [];
    var inner = (x, depth) => {
        if (typeof x === 'number') return result.push([depth, x]);
        x.forEach((s) => inner(s, depth + 1));
    };
    inner(snail, -1);
    return result;
};
var input = document.body.innerText.trim().split('\n').map(JSON.parse).map(convert);
var add = (a, b) => {
    var explode = (snail) => {
        var index = snail.findIndex((s) => s[0] == 4);
        if (index >= 0) {
            var [depth, a] = snail[index];
            var [[_, b]] = snail.splice(index + 1, 1);
            snail[index] = [3, 0];
            if (index >= 1) snail[index - 1][1] += a;
            if (index + 1 < snail.length) snail[index + 1][1] += b;
        }
        return index >= 0;
    };
    var split = (snail) => {
        var index = snail.findIndex((s) => s[1] >= 10);
        if (index >= 0) {
            var [depth, a] = snail[index];
            snail.splice(index, 1, [depth + 1, Math.floor(a / 2)], [depth + 1, Math.ceil(a / 2)]);
        }
        return index >= 0;
    };
    var snail = [...a.map((e) => [e[0] + 1, e[1]]), ...b.map((e) => [e[0] + 1, e[1]])];
    while (explode(snail) || split(snail));
    return snail;
};
var sum = input.reduce(add);
var magnitude = (snail) => {
    for (var s = 0; s < 4; s++) {
        var next = [];
        for (var i = 0; i < snail.length; i++) {
            if (i + 1 < snail.length && snail[i][0] == snail[i + 1][0]) {
                next.push([snail[i][0] - 1, 3 * snail[i][1] + 2 * snail[i + 1][1]]);
                i++;
            } else {
                next.push(snail[i]);
            }
        }
        snail = next;
    }
    return snail[0][1];
};
var result = magnitude(sum);
console.log("Part 1:", result);
var result = Math.max(...input.flatMap((a, i) => input.filter(($, j) => i != j).map((b) => magnitude(add(a, b)))));
console.log("Part 2:", result);
