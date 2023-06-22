export default function bubble_sort(arr: number[]): void {
    const len = arr.length;

    for (let i = 0; i < len; i++) {
        // only iterate up to (len - 1 - i) because:
        //  - the last item of the array is always sorted 1 iteration before
        //      we reach it - we don't have to evaluate the last item during each
        //      evaluation
        //  - on each iteration, we know that the value at (len - i) is sorted,
        //      because with bubble sort every iteration moves the largest unsorted
        //      value into its final position. Thus, we don't need to evaluate
        //      anything beyond (len - i)
        for (let j = 0; j < len - 1 - i; j++) {
            // use j and j + 1 - easier to understand than trying to understand
            // what arr[i] means at any given point
            if (arr[j] > arr[j + 1]) {
                const tmp = arr[j + 1];
                arr[j + 1] = arr[j];
                arr[j] = tmp;
            }
        }
    }
}
