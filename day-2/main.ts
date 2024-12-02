import { getInput } from "@util";

async function main() {
    const input = await getInput("day-2", { dev: false });

    const safeReports = input
        .trim()
        .split("\n")
        .map((line) => line.split(/\s+/).map(Number))
        .filter((report, i) => {
            const isSafe = isReportSafe(report);

            console.log(`${i}. ${report} => ${isSafe ? "SAFE" : "NOT SAFE"}`);

            return isSafe;
        }).length;

    console.log(safeReports);
}

function isReportSafe(report: number[]): boolean {
    if (isExactReportSafe(report)) return true;

    for (let i = 0; i < report.length; i++) {
        const partialReport = [...report.slice(0, i), ...report.slice(i + 1)];
        if (isExactReportSafe(partialReport)) {
            return true;
        }
    }

    return false;
}

function isExactReportSafe(report: number[]): boolean {
    if (report.length === 0) return false;
    if (report.length < 2) return true;

    const direction = Math.sign(report[1]! - report[0]!);
    if (direction === 0) return false;

    return report.every((level, i) => {
        const prevLevel = report[i - 1] ?? null;
        if (prevLevel === null) return true;
        return isLevelSafe(prevLevel, level, direction);
    });
}

// function isReportSafe(report: number[]): boolean {
//     if (report.length === 0) return false;
//     if (report.length < 2) return true;

//     const direction = report
//         .map((level, i) => {
//             const prevLevel = report[i - 1];
//             if (!prevLevel) return null;
//             return Math.sign(level - prevLevel);
//         })
//         .filter((direction): direction is number => direction !== null)
//         .reduce<Map<number, number>>(
//             (directionCounts, direction) =>
//                 directionCounts.set(
//                     direction,
//                     (directionCounts.get(direction) ?? 0) + 1,
//                 ),
//             new Map(),
//         )
//         .entries()
//         .reduce<[number, number]>(
//             (highest, directionCount) =>
//                 directionCount[1] > highest[1] ? directionCount : highest,
//             [0, 0],
//         )[0];

//     // const direction = Math.sign(report[1]! - report[0]!);

//     if (direction === 0) return false;

//     let hasSkipped = false;
//     let prevIdx = -1;

//     for (let i = 0; i < report.length; i++) {
//         const level = report[i]!;
//         const prevLevel = report[prevIdx] ?? null;

//         if (prevLevel === null) {
//             prevIdx = i;
//             continue;
//         }

//         const isSafe = isLevelSafe(prevLevel, level, direction);

//         if (isSafe) {
//             prevIdx = i;
//             continue;
//         }

//         if (!hasSkipped) {
//             hasSkipped = true;

//             const nextLevel = report[i + 1] ?? null;
//             if (nextLevel === null) continue;

//             const isNextSafe = isLevelSafe(level, nextLevel, direction);

//             if (isNextSafe) {
//                 prevIdx = i;
//             }

//             continue;
//         }

//         return false;
//     }

//     return true;
// }

function isLevelSafe(
    prevLevel: number,
    nextLevel: number,
    direction: number,
): boolean {
    const MAX_LEVEL_DIST = 3;

    const diff = nextLevel - prevLevel;
    return direction === Math.sign(diff) && Math.abs(diff) <= MAX_LEVEL_DIST;
}

main();
