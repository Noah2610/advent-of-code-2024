#!/bin/env -S deno --allow-read --allow-run

import { dirname } from "node:path";
import { expectNever } from "ts-expect";

type ProjectType = "ts" | "rs";

async function main(args: string[]) {
    if (args.length === 0) {
        throw new Error("Expected at least 1 argument for day, ex. `day-1`");
    }

    const dayArg = args[0]!;
    const dir = [dirname(import.meta.dirname ?? "."), dayArg].join("/");

    try {
        const stat = await Deno.stat(dir);
        if (!stat.isDirectory) {
            throw new Error();
        }
    } catch (_e: unknown) {
        throw new Error(`Day "${dayArg}" directory does not exist "${dir}"`);
    }

    const projectType = await determineProjectType(dir);
    console.warn(`Running ${dayArg}`);

    let cmd: Deno.Command;

    switch (projectType) {
        case "ts": {
            cmd = new Deno.Command("deno", {
                args: [
                    "run",
                    "--allow-read",
                    "--allow-env",
                    "main.ts",
                    ...args.slice(1),
                ],
                cwd: dir,
            });

            break;
        }

        case "rs": {
            cmd = new Deno.Command("cargo", {
                args: ["run", ...args.slice(1)],
                cwd: dir,
            });

            break;
        }

        default: {
            expectNever(projectType);
        }
    }

    const proc = cmd.spawn();
    const status = await proc.status;

    if (!status.success) {
        throw new Error(`Process exited with status code ${status.code}`);
    }
}

async function determineProjectType(dir: string): Promise<ProjectType> {
    const projectTypeFileNames: { file: string; type: ProjectType }[] = [
        {
            file: "main.ts",
            type: "ts",
        },
        {
            file: "Cargo.toml",
            type: "rs",
        },
    ];

    for (const { file, type } of projectTypeFileNames) {
        try {
            const stat = await Deno.stat([dir, file].join("/"));
            if (!stat.isFile) {
                throw new Error();
            }

            return type;
        } catch (_e: unknown) {
            // do nothing
        }
    }

    throw new Error(`Failed to determine project type in dir "${dir}"`);
}

try {
    await main(Deno.args);
} catch (e: unknown) {
    const msg =
        e instanceof Error
            ? e.message
            : typeof e === "string"
            ? e
            : JSON.stringify(e);
    console.error(
        `%c[Error]%c ${msg}`,
        "background-color: red; font-weight: bold",
        "color: red",
    );
    Deno.exit(1);
}
