import { describe, expect, it } from "@jest/globals";
import { highlightMultiLine, highlightSingleLine } from "./functions";

describe("highlight single line", () => {
    it.each([
        ["", 0, 0, {suffix: ""}],
        ["", 1, 2, {suffix: ""}],
        ["foo bar", 0, 0, {suffix: "foo bar"}],
        [
            "foo bar", 2, 4, {prefix: "fo",
                marked: "o ",
                suffix: "bar"},
        ],
        [
            "foo bar", 0, 7, {prefix: "",
                marked: "foo bar",
                suffix: ""},
        ],
        ["foo bar", 3, 3, {suffix: "foo bar"}],
        ["foo bar", 4, 1, {suffix: "foo bar"}],
        [
            "foo bar", 10, 20, {prefix: "foo bar",
                marked: "",
                suffix: ""},
        ],
        ["foo bar", 20, 10, {suffix: "foo bar"}],
        ["foo bar", -1, 2, {suffix: "foo bar"}],
        ["foo bar", 1, -2, {suffix: "foo bar"}],
    ])("highlightSingleLine(%s, %i, %i) == %s", (str, symbolStart, symbolEnd, expected) => {
        const actual = highlightSingleLine(str, symbolStart, symbolEnd);

        expect(actual).toEqual(expected);
    });
});

describe("highlightMultiLine", () => {
    it.each([
        [
            "", 0, 0, 0, 0, 0, {prefix: "",
                marked: ""},
        ],
        ["", 1, 0, 0, 0, 0, {marked: ""}],
        ["foo bar", -1, 0, 5, 2, 6, {marked: "foo bar"}],
        ["foo bar", 1, 0, 5, 2, 6, {marked: "foo bar"}],
        [
            "foo bar", 1, 1, 5, 2, 6, {prefix: "fo",
                marked: "o bar"},
        ],
        [
            "foo bar", 1, 1, 5, -2, 6, {prefix: "",
                marked: "foo bar"},
        ],
        [
            "foo bar", 1, 1, 5, 2, -6, {prefix: "fo",
                marked: "o bar"},
        ],
        [
            "foo bar", 1, 1, 5, 0, 0, {prefix: "",
                marked: "foo bar"},
        ],
        [
            "foo bar", 5, 1, 5, 2, 6, {marked: "foo ba",
                suffix: "r"},
        ],
        [
            "foo bar", 5, 1, 5, -2, 6, {marked: "foo ba",
                suffix: "r"},
        ],
        [
            "foo bar", 5, 1, 5, 2, -6, {marked: "foo bar",
                suffix: ""},
        ],
        [
            "foo bar", 5, 1, 5, 0, 0, {marked: "",
                suffix: "foo bar"},
        ],
    ])("highlightMultiLine(%s, %i, %i, %i, %i, %i) == %s", (
        str,
        lineCurr,
        lineStart,
        lineEnd,
        symbolStart,
        symbolEnd,
        expected,
    ) => {
        const actual = highlightMultiLine(
            str,
            lineCurr,
            lineStart,
            lineEnd,
            symbolStart,
            symbolEnd,
        );

        expect(actual).toEqual(expected);
    });
});
