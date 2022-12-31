var input = document.body.innerText.trim().replaceAll(' ', '').split('\n').map((s) => s.split(''));
var parse = (input, prec) => {
    var output = [];
    var stack = [];
    input.forEach((s) => {
        if (prec[s]) {
            while ((prec[stack.at(-1)] || 0) >= prec[s]) output.push(stack.pop());
            stack.push(s);
        } else if (s == '(') {
            stack.push('(');
        } else if (s == ')') {
            while (stack.at(-1) != '(') output.push(stack.pop());
            stack.pop();
        } else {
            output.push(parseInt(s));
        }
    });
    return [...output, ...stack.reverse()];
};
var reduce = (postfix) => {
    var stack = [];
    postfix.forEach((t) => {
        if (t == '+') stack.push(stack.pop() + stack.pop());
        else if (t == '*') stack.push(stack.pop() * stack.pop());
        else stack.push(t);
    });
    return stack[0];
};
console.log("Part 1:", input.map((c) => reduce(parse(c, {'+': 1, '*': 1}))).reduce((a, b) => a + b));
console.log("Part 2:", input.map((c) => reduce(parse(c, {'+': 2, '*': 1}))).reduce((a, b) => a + b));
