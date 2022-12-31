var input = document.body.innerText.trim().split('\n');
var invalid = input.flatMap((line) => {
    try {
        line.split('').reduce((state, c) => {
            if (c == '(') state.push(')');
            else if (c == '[') state.push(']');
            else if (c == '{') state.push('}');
            else if (c == '<') state.push('>');
            else if (state.pop() != c) throw c;
            return state;
        }, []);
        return [];
    } catch (e) {
        return [e];
    }
});
var scores = {')': 3, ']': 57, '}': 1197, '>': 25137};
console.log("Part 1:", invalid.map((x) => scores[x]).reduce((a, b) => a + b));
var finish = input.flatMap((line) => {
    try {
        return [line.split('').reduce((state, c) => {
            if (c == '(') state.push(')');
            else if (c == '[') state.push(']');
            else if (c == '{') state.push('}');
            else if (c == '<') state.push('>');
            else if (state.pop() != c) throw c;
            return state;
        }, [])];
    } catch (e) {
        return [];
    }
});
var scores = {')': 1, ']': 2, '}': 3, '>': 4};
var all = finish.map((x) => parseInt(x.map((x) => scores[x]).reverse().join(''), 5)).sort((a, b) => a - b);
console.log("Part 2:", all[(all.length - 1) / 2]);
