export type ExtractResult<T> = {
    parsed: T | null;
    rest: string;
};

export type Extractor<T> = (input: string) => ExtractResult<T>;

export function extractToken<T extends string>(
    input: string,
    token: T,
): ExtractResult<T> {
    const part = input.slice(0, token.length);
    if (part === token) {
        return {
            parsed: token,
            rest: input.slice(token.length),
        };
    }

    return {
        parsed: null,
        rest: input,
    };
}

export function extractFirst<T>(
    input: string,
    extractors: Extractor<T>[],
): ExtractResult<T> {
    for (const extractor of extractors) {
        const result = extractor(input);
        if (result.parsed) {
            return result;
        }
    }

    return {
        parsed: null,
        rest: input,
    };
}

export function extractNumber(input: string): ExtractResult<number> {
    const DIGITS = new Set([..."0123456789"]);

    const { parsed: parsedNumsS, rest } = extractWhile(input, (s) => {
        const c = s[0];
        if (!c || !DIGITS.has(c)) {
            return { parsed: null, rest: s };
        }

        return { parsed: c, rest: s.slice(1) };
    });

    if (!parsedNumsS || parsedNumsS.length === 0) {
        return {
            parsed: null,
            rest,
        };
    }

    return {
        parsed: Number(parsedNumsS.join("")),
        rest,
    };
}

export function extractWhile<T>(
    input: string,
    extractor: Extractor<T>,
): ExtractResult<T[]> {
    let rest = input;
    const parsed: T[] = [];

    let result = extractor(rest);
    while (result.parsed) {
        parsed.push(result.parsed);
        rest = result.rest;
        result = extractor(rest);
    }

    return {
        parsed,
        rest: result.rest,
    };
}

export function extractUntil<T>(
    input: string,
    extractor: Extractor<T>,
): ExtractResult<T> {
    let parsedFinal: T | null = null;

    const { rest } = extractWhile(input, (s) => {
        const { parsed, rest } = extractor(s);
        if (parsed) {
            parsedFinal = parsed;
            return {
                parsed: null,
                rest: rest,
            };
        } else {
            return {
                parsed: rest.slice(0, 1),
                rest: rest.slice(1),
            };
        }
    });

    return {
        parsed: parsedFinal,
        rest,
    };
}
