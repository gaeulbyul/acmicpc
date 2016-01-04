function singi (a, b) {
	return a.toString(b).split('').map(function (x) {
		return parseInt(x, b);
	}).reduce(function (x, y) {
		return x + y;
	});
}
for(var i=2992; i<=9999; i++) {
	var n10 = singi(i, 10);
	var n12 = singi(i, 12);
	var n16 = singi(i, 16);
	if (n10 === n12 && n12 === n16) {
		print(i);
	}
}
