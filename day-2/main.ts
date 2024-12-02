import { getInput } from "@util";

async function main() {
    const input = await getInput("day-2");

    const safeReports = input
        .trim()
        .split("\n")
        .map((line) => line.split(/\s+/).map(Number))
        .filter(isReportSafe).length;

    console.log(safeReports);
}

function isReportSafe(report: number[]): boolean {
    const MAX_LEVEL_DIST = 3;

    if (report.length === 0) return false;
    if (report.length < 2) return true;

    const direction = Math.sign(report[1]! - report[0]!);
    if (direction === 0) return false;

    return report.every((level, i) => {
        const prevLevel = report[i - 1] ?? null;
        if (prevLevel === null) return true;

        const diff = level - prevLevel;
        return (
            direction === Math.sign(diff) && Math.abs(diff) <= MAX_LEVEL_DIST
        );
    });
}

main();
