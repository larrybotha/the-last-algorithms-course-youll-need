export default function bs_list(haystack: number[], needle: number): boolean {
    let result = false;
    let minIndex = 0
    let maxIndex = haystack.length; // max-exclusive [min, max)

    do {
        const midIndex = Math.floor(minIndex + (maxIndex - minIndex) / 2)
        const midValue = haystack[midIndex]

        if (midValue === needle) {
            result = true
            break;
        } else if (midValue < needle) {
            // exclude the midpoint from subsequent iterations
            minIndex = midIndex + 1;
        } else {
            maxIndex = midIndex
        }
    } while (minIndex < maxIndex);

    return result
}
