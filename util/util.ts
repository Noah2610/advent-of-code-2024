import { dirname } from "node:path";

export async function getInput(day: `day-${number}`): Promise<string> {
    return await Deno.readTextFile(
        `${dirname(import.meta.dirname ?? ".")}/${day}/input.txt`,
    );
}

export function dbg<T>(val: T) {
    console.log(val);
    return val;
}
