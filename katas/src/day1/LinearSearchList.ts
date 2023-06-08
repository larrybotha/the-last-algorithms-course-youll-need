export default function linear_search(haystack: number[], needle: number): boolean {
    let result = false;
    const num_values =  haystack.length;

 for (let i = 0; i < num_values; i++) {
    if (needle == haystack[i]) {
        result = true
        break;
    }
 }

 return result
}
