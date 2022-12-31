var input = document.body.innerText.trim().split('').flatMap((s) => parseInt(s, 16).toString(2).padStart(4, '0')).join('');
var makeStream = (stream) => (n, raw) => {
    if (stream.length < n)
        throw "buffer underrun";
    var res = stream.slice(0, n);
    stream = stream.slice(n);
    return raw ? res : parseInt(res, 2);
};
var parse = (stream) => {
    try {
        var version = stream(3);
        var type = stream(3);
        if (type === 4) {
            var value = 0;
            while (true) {
                var part = stream(5);
                value = value * 16 + part % 16;
                if (part < 16) break;
            }
            return {version: version, type: type, value: value};
        } else if (stream(1) == 0) {
            var bits = stream(15);
            var segment = stream(bits, true);
            var substream = makeStream(segment);
            var subpackets = [];
            for (var cur; cur = parse(substream);) subpackets.push(cur);
            return {version: version, type: type, subpackets: subpackets};
        } else {
            var packets = stream(11);
            var subpackets = Array(packets).fill().map(($) => parse(stream));
            return {version: version, type: type, subpackets: subpackets};
        }
    } catch ($) {
        return undefined;
    }
};
var packet = parse(makeStream(input));
var total = (packet) => (packet.subpackets || []).map(total).reduce((a, b) => a + b, packet.version);
console.log("Part 1:", total(packet));
var compute = (packet) => {
    if (packet.type == 0) return packet.subpackets.map(compute).reduce((a, b) => a + b, 0);
    if (packet.type == 1) return packet.subpackets.map(compute).reduce((a, b) => a * b, 1);
    if (packet.type == 2) return Math.min(...packet.subpackets.map(compute));
    if (packet.type == 3) return Math.max(...packet.subpackets.map(compute));
    if (packet.type == 4) return packet.value;
    if (packet.type == 5) return compute(packet.subpackets[0]) > compute(packet.subpackets[1]) ? 1 : 0;
    if (packet.type == 6) return compute(packet.subpackets[0]) < compute(packet.subpackets[1]) ? 1 : 0;
    if (packet.type == 7) return compute(packet.subpackets[0]) == compute(packet.subpackets[1]) ? 1 : 0;
};
console.log("Part 2:", compute(packet));
