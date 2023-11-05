import type { JestConfigWithTsJest } from "ts-jest";

export default (): JestConfigWithTsJest => {
    return {
        rootDir: "src",
        preset: "ts-jest",
        testEnvironment: "node",
        extensionsToTreatAsEsm: [".ts"],
        transform: {
            "^.+\\.(ts|tsx)?$": [
                "ts-jest",
                {
                    tsconfig: "tsconfig.test.json",
                    useESM: true,
                },
            ],
        },
    };
}
