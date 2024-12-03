import { getInput } from "@util";
import {
    extractNumber,
    extractToken,
    extractUntil,
    extractWhile,
    type ExtractResult,
} from "./extractors.ts";

type Instruction = {
    a: number;
    b: number;
};

async function main() {
    const input = await getInput("day-3");
    const instructions = parseInstructions(input);

    if (!instructions || instructions.length === 0) {
        throw new Error("Failed to parse instructions");
    }

    let sum = 0;
    for (const instruction of instructions) {
        const product = instruction.a * instruction.b;
        sum += product;
    }

    console.log(sum);
}

function parseInstructions(input: string): Instruction[] | null {
    const { parsed: instructions } = extractWhile(input, (s) =>
        extractUntil(s, extractInstruction),
    );
    return instructions;
}

function extractInstruction(input: string): ExtractResult<Instruction> {
    const { parsed: parsedToken, rest: restToken } = extractToken(input, "mul");
    if (!parsedToken) {
        return { parsed: null, rest: input };
    }

    const { parsed: parsedLparen, rest: restLparen } = extractToken(
        restToken,
        "(",
    );
    if (!parsedLparen) {
        return { parsed: null, rest: input };
    }

    const { parsed: parsedA, rest: restA } = extractNumber(restLparen);
    if (!parsedA) {
        return { parsed: null, rest: input };
    }

    const { parsed: parsedComma, rest: restComma } = extractToken(restA, ",");
    if (!parsedComma) {
        return { parsed: null, rest: input };
    }

    const { parsed: parsedB, rest: restB } = extractNumber(restComma);
    if (!parsedB) {
        return { parsed: null, rest: input };
    }

    const { parsed: parsedRparen, rest: restRparen } = extractToken(restB, ")");
    if (!parsedRparen) {
        return { parsed: null, rest: input };
    }

    return {
        parsed: {
            a: parsedA,
            b: parsedB,
        },
        rest: restRparen,
    };
}

main();
