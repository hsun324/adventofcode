var input = document.body.innerText.trim().split('').map((s) => parseInt(s));
var state = [...input];
for (var i = 0; i < 100; i++) {
    var current = state.shift();
    var held = [state.shift(), state.shift(), state.shift()];
    var destination = current == 1 ? input.length : current - 1;
    while (held.indexOf(destination) >= 0)
        destination = destination == 1 ? input.length : destination - 1;
    state.splice(state.indexOf(destination) + 1, 0, ...held);
    state.push(current);
}
console.log("Part 1:", state.join('').split('1').reverse().join(''));
var max = 1000000;
var next = Array(max + 1).fill().map(($, i) => i % max + 1);
next[max] = input[0];
input.slice(0, -1).forEach((l, i) => next[l] = input[i + 1]);
next[input.at(-1)] = input.length + 1;
for (var i = 0, current = input[0]; i < 10000000; i++) {
    var n1 = next[current], n2 = next[n1], n3 = next[n2], after = next[n3];
    var destination = current == 1 ? max : current - 1;
    while (destination == n1 || destination == n2 || destination == n3)
        destination = destination == 1 ? max : destination - 1;
    var post = next[destination];
    next[destination] = n1;
    next[n3] = post;
    next[current] = after;
    current = after;
}
console.log("Part 2:", next[1] * next[next[1]]);
