import { dirname } from "node:path";

export async function getInput(
    day: `day-${number}`,
    options?: { dev?: boolean },
): Promise<string> {
    const dev = options?.dev ?? false;
    const inputFile = dev ? "dev-input.txt" : "input.txt";

    return await Deno.readTextFile(
        [dirname(import.meta.dirname ?? "."), day, inputFile].join("/"),
    );
}

export function dbg<T>(val: T) {
    console.log(val);
    return val;
}
