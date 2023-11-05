/**
 * highlightText
 * Highlights given text.
 */
export const highlightText = (
    str: string,
    highlightIndex: number,
    lineCurr: number,
    lineStart: number,
    lineEnd: number,
    symbolStart: number,
    symbolEnd: number,
): string => {
    // We should check that current line is inside highlight.
    if (lineStart < lineCurr && lineCurr > lineEnd) {
        return str;
    }

    let highlight: HighlightResult;

    // Single line highlight.
    if (lineStart === lineEnd) {
        highlight = highlightSingleLine(str, symbolStart, symbolEnd);
    } else {
        // Multiline highlight.
        highlight = highlightMultiLine(str, lineCurr, lineStart, lineEnd, symbolStart, symbolEnd);
    }

    const { prefix, marked, suffix } = highlight;

    // Check if current highlight is multiline.
    return `${prefix}<mark data-index="${highlightIndex}">${marked}</mark>${suffix}`;
};

export interface HighlightResult {
    prefix?: string,
    marked?: string,
    suffix?: string,
}

/**
 * highlightSingleLine
 * Simply mark text between symbolStart and symbolEnd.
 */
export const highlightSingleLine = (
    str: string,
    symbolStart: number,
    symbolEnd: number,
): HighlightResult => {
    if (str === "") {
        return {
            suffix: "",
        };
    }

    if (symbolStart < 0 || symbolEnd < 0) {
        return {
            suffix: str,
        };
    }

    if (symbolStart >= symbolEnd) {
        return {
            suffix: str,
        };
    }

    return {
        prefix: str.substring(0, symbolStart),
        marked: str.substring(symbolStart, symbolEnd),
        suffix: str.substring(symbolEnd),
    };
};

/**
 * highlightMultiLine
 *  Handles three different scenarios:
 *
 *  1. Current line is the first line of highlight;
 *  2. Current line is inside of highlight;
 *  3. Current line is the last line of highlights.
 */
export const highlightMultiLine = (
    str: string,
    lineCurr: number,
    lineStart: number,
    lineEnd: number,
    symbolStart: number,
    symbolEnd: number,
): HighlightResult => {
    const res: HighlightResult = {};

    switch (lineCurr) {
        // First scenario.
        // Mark whole text from the symbolStart to the end of the line.
        case lineStart:
            symbolStart = symbolStart < 0 ? 0 : symbolStart;

            res.prefix = str.substring(0, symbolStart);
            res.marked = str.substring(symbolStart);
            break;

        // Third scenario.
        // Mark whole text from the beginning of the line to symbolEnd.
        case lineEnd:
            symbolEnd = symbolEnd < 0 ? str.length : symbolEnd;

            res.marked = str.substring(0, symbolEnd);
            res.suffix = str.substring(symbolEnd);
            break;

        // Second scenario.
        // Mark whole line.
        default:
            res.marked = str;
    }

    return res;
};
