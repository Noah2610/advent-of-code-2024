import { getInput } from "@util";

async function main() {
    const data = await getInput("day-1");
    // const result = calcTotalDistance(data);
    const result = calcSimilarityScore(data);
    console.log(result);
}

function calcSimilarityScore(data: string) {
    const [listA, listB] = getLists(data);

    return listA.reduce(
        (score, num) =>
            score +
            num * listB.reduce((sum, b) => sum + (num === b ? 1 : 0), 0),
        0,
    );
}

function calcTotalDistance(data: string) {
    const [listA, listB] = getLists(data).map((list) =>
        list.sort((a, b) => a - b),
    );

    const totalDist = listA
        .map((numA, i) => [numA, listB[i]] as [number, number])
        .reduce((sum, [a, b]) => sum + Math.abs(a - b), 0);

    return totalDist;
}

function getLists(data: string) {
    return data.split("\n").reduce<[number[], number[]]>(
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
    );
}

main();
