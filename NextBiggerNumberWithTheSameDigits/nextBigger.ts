export function nextBigger(n: number): number{
	const stringNumber = n.toString();
	
	if (stringNumber.length < 2) {
		return -1;
	}

	let returnValue = Infinity;
	let indexCap = 0;
	let largeDigitIndex = stringNumber.length - 2;
	let smallDigitIndex = stringNumber.length - 1;

	while (indexCap <= largeDigitIndex) {
		const swapValue = isFinite(returnValue) ? returnValue.toString() : stringNumber;
		const candidateNumber = swapDigitsValue(swapValue, largeDigitIndex, smallDigitIndex);

		if (candidateNumber > n && candidateNumber < returnValue) {
			indexCap = indexCap ? indexCap : largeDigitIndex; //Once Index cap is set to largeDigitIndex every combination up to indexCap will need to be checked.
			returnValue = candidateNumber;
			largeDigitIndex = stringNumber.length - 2;
			smallDigitIndex = stringNumber.length - 1;

		} else if (largeDigitIndex === smallDigitIndex -1) {
			largeDigitIndex--;
			smallDigitIndex = stringNumber.length -1;

		} else {
			smallDigitIndex--;
		}
	}

	return isFinite(returnValue) ? returnValue : -1;
}


const swapDigitsValue = (stringNumber:string, swapIndex1:number, swapIndex2:number): number => {
	let returnValue = "";

	for (let i=0; i<stringNumber.length; i++) {
		switch (i) {
			case swapIndex1:
				returnValue += stringNumber[swapIndex2];
				break;

			case swapIndex2:
				returnValue += stringNumber[swapIndex1];
				break;

			default:
				returnValue += stringNumber[i];
		}

	}

	return Number(returnValue);
}
