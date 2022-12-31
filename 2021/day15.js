var input = document.body.innerText.trim().split('\n').map((s) => s.split('').map((s) => parseInt(s)));
var solve = (board) => {
    var queue = [{x: 0, y: 0, risk: 0}];
    var visited = {};
    while (queue.length) {
        var front = queue[0];
        queue = queue.slice(1);
        if (visited[[front.x, front.y]]) continue;
        if (front.x == board.length - 1 && front.y == board.length - 1) return front.risk;
        visited[[front.x, front.y]] = true;
        [[-1, 0], [1, 0], [0, -1], [0, 1]].forEach((d) => {
            var nx = front.x + d[0], ny = front.y + d[1];
            if (nx >= 0 && nx < board.length && ny >= 0 && ny < board[nx].length)
                queue.push({x: nx, y: ny, risk: front.risk + board[nx][ny]});
        });
        queue.sort((a, b) => a.risk - b.risk);
    }
}
console.log("Part 1:", solve(input));
var five = [0, 1, 2, 3, 4];
var wrap = (n) => (n - 1) %  9 + 1;
var extended = five.flatMap((cx) => input.map((row) => five.flatMap((cy) => row.map((v) => wrap(v + cx + cy)))));
console.log("Part 2:", solve(extended));
