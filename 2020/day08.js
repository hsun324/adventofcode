var input = document.body.innerText.trim().split('\n').map((s) => [s[0], (s[4] == '+' ? 1 : -1) * parseInt(s.slice(5))]);
function run(prog) {
    var exec = [];
    for (var acc = 0, ip = 0; ip < prog.length && !exec[ip]; ip++) {
        exec[ip] = true;
        if (prog[ip][0] == 'a') acc += prog[ip][1];
        if (prog[ip][0] == 'j') ip += prog[ip][1] - 1;
    }
    return [ip == prog.length, acc];
}
console.log("Part 1:", run(input)[1]);
console.log("Part 2:", input.map((x, i) => {
    var prog = [...input.map((x) => [...x])];
    prog[i][0] = prog[i][0] == 'n' ? 'j' : 'n';
    return run(prog);
}).find((x) => x[0])[1]);
