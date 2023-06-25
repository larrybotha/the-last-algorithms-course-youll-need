interface Node<T> {
    value?: T;
    next?: Node<T>;
}

export default class Queue<T> {
    public length: number;
    private head: Node<T>;
    private tail: Node<T>;

    constructor() {
        this.length = 0;
        this.tail = {};
        this.head = this.tail;
    }

    enqueue(item: T): void {
        const node = { value: item };

        this.head = this.length == 0 ? node : this.head;
        this.tail.next = node;
        this.tail = node;

        this.length++;
    }

    deque(): T | undefined {
        const node = this.head;

        this.length = Math.max(0, this.length - 1);
        this.tail = this.length == 0 ? {} : this.tail;
        this.head =
            this.length > 0 && this.head.next ? this.head.next : this.tail;

        return node.value;
    }

    peek(): T | undefined {
        return this.head.value;
    }
}
