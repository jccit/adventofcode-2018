var fs = require('fs');
var readline = require('readline');

var lineReader = readline.createInterface({
	input: fs.createReadStream('input.txt')
});

var freq = 0;

lineReader.on('line', (line) => {
	var num = parseInt(line);
	if (!isNaN(num)) freq += num;
});

lineReader.on('close', () => {
	console.log(freq);
});
