var fs = require('fs');
var readline = require('readline');

var lineReader = readline.createInterface({
	input: fs.createReadStream('input.txt')
});

var freq = 0;
var changes = [];
var seenNums = [];

lineReader.on('line', (line) => {
	var num = parseInt(line);
	if (!isNaN(num)) changes.push(num);
});

function applyChanges(callback) {
	var num;
	for (num of changes) {
		freq += num;
		callback(freq);
	}
}

var runCount = 0;
var duplicate = null;

lineReader.on('close', () => {
	while (duplicate === null) {
		applyChanges((f) => {
			if (duplicate === null && seenNums[f]) {
				// found duplicate
				console.log(f)
				duplicate = f;
			};

			seenNums[f] = true;
		});

		runCount++;
	}
});
