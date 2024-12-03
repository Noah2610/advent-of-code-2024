import { expectNever } from "ts-expect";
import { getInput } from "@util";
import {
    extractNumber,
    extractFirst,
    extractToken,
    extractUntil,
    extractWhile,
    type ExtractResult,
} from "./extractors.ts";

type Instruction =
    | { type: "do" }
    | { type: "dont" }
    | {
          type: "mul";
          a: number;
          b: number;
      };

type InstructionOfType<T extends Instruction["type"]> = Instruction & {
    type: T;
};

async function main() {
    const input = await getInput("day-3");
    const instructions = parseInstructions(input);

    if (!instructions || instructions.length === 0) {
        throw new Error("Failed to parse instructions");
    }

    let sum = 0;
    let isEnabled = true;

    for (const instruction of instructions) {
        if (instruction.type === "do") {
            isEnabled = true;
            continue;
        }

        if (instruction.type === "dont") {
            isEnabled = false;
            continue;
        }

        if (instruction.type === "mul") {
            if (!isEnabled) continue;
            sum += instruction.a * instruction.b;
            continue;
        }

        expectNever(instruction);
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
    return extractFirst<Instruction>(input, [
        extractInstructionDo,
        extractInstructionDont,
        extractInstructionMul,
    ]);
}

function extractInstructionDo(
    input: string,
): ExtractResult<InstructionOfType<"do">> {
    const { parsed: parsedToken, rest: restToken } = extractToken(input, "do");
    if (!parsedToken) {
        return { parsed: null, rest: input };
    }

    const { parsed: parsedParens, rest: restParens } = extractToken(
        restToken,
        "()",
    );
    if (!parsedParens) {
        return { parsed: null, rest: input };
    }

    return {
        parsed: {
            type: "do",
        },
        rest: restParens,
    };
}

function extractInstructionDont(
    input: string,
): ExtractResult<InstructionOfType<"dont">> {
    const { parsed: parsedToken, rest: restToken } = extractToken(
        input,
        "don't",
    );
    if (!parsedToken) {
        return { parsed: null, rest: input };
    }

    const { parsed: parsedParens, rest: restParens } = extractToken(
        restToken,
        "()",
    );
    if (!parsedParens) {
        return { parsed: null, rest: input };
    }

    return {
        parsed: {
            type: "dont",
        },
        rest: restParens,
    };
}

function extractInstructionMul(
    input: string,
): ExtractResult<InstructionOfType<"mul">> {
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
            type: "mul",
            a: parsedA,
            b: parsedB,
        },
        rest: restRparen,
    };
}

main();
