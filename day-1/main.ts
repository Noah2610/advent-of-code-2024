async function main() {
    const data = await getInput();
    const result = calcTotalDistance(data);
    console.log(result);
}

function calcTotalDistance(data: string) {
    const [listA, listB] = data
        .split("\n")
        .reduce<[number[], number[]]>(
            (lists, line) => {
                const nums = line.split(/\s+/).map(Number);
                if (nums.length !== 2) {
                    return lists;
                }
                return lists.map((list, i) => [...list, nums[i]]) as [
                    number[],
                    number[],
                ];
            },
            [[], []],
        )
        .map((list) => list.sort((a, b) => a - b));

    const totalDist = listA
        .map((numA, i) => [numA, listB[i]] as [number, number])
        .reduce((sum, [a, b]) => sum + Math.abs(a - b), 0);

    return totalDist;
}

async function getInput() {
    return await Deno.readTextFile(`${import.meta.dirname}/input.txt`);
}

main();

function dbg<T>(val: T) {
    console.log(val);
    return val;
}
