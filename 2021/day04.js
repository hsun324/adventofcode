var rawInput = document.body.innerText.trim().split('\n\n');
var calls = rawInput[0].split(',').map((s) => parseInt(s));
var callsT = Object.fromEntries(calls.map((c, i) => [c, i]));
var boards = rawInput.slice(1).map((s) => s.replaceAll('\n', ' ').replaceAll('  ', ' ').trim().split(' ').map((s2) => parseInt(s2, 10)));
var lines = Array(5).fill(0).flatMap((x, i) => [[i, i + 5, i + 10, i + 15, i + 20], [5 * i, 5 * i + 1, 5 * i + 2, 5 * i + 3, 5 * i + 4]]);
function occur(board) {
    var min = Math.min(...lines.map((l) => Math.max(...l.map((x) => callsT[board[x]]))));
    var value = board.filter((v) => callsT[v] > min).reduce((a, b) => a + b, 0) * calls[min];
    return [min, value];
}
var results = boards.map((b, i) => [i, occur(b)]).sort((l, r) => l[1][0] - r[1][0]);
console.log("Part 1:", results[0][1][1]);
console.log("Part 2:", results[results.length - 1][1][1]);
