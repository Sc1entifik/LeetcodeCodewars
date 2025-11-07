function findUniq(arr: string[]):string {
	const sortedStrings = arr.map((x:string, y:number) => {
		let cleanString = x.toLowerCase().split("");
		cleanString.sort();
		const sortedUniqueString = [...new Set(cleanString)];

		return [sortedUniqueString.join(""), y];
	});

	const sortedStringMap = new Map();
	sortedStrings.forEach(x => {
		const key = x[0];

		if (sortedStringMap.get(key)) {
			sortedStringMap.set(key, sortedStringMap.get(key) + 1);

		} else {
			sortedStringMap.set(key, 1);
		}
	});

	const returnIndex = sortedStrings.filter(x => sortedStringMap.get(x[0]) === 1)[0][1];

	return arr[returnIndex];
}
