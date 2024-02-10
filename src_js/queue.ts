export class Queue<T> {
  readonly #elements: (T | undefined)[] = [];
  #headIndex: number = 0;

  isEmpty() {
    return this.#headIndex === this.#elements.length;
  }

  enqueue(...items: T[]) {
    this.#elements.push(...items);
  }

  dequeue(): T {
    if (this.isEmpty()) {
      throw new Error('Queue is empty');
    }

    const result = this.#elements[this.#headIndex] as T;
    this.#elements[this.#headIndex] = undefined;
    this.#headIndex++;

    if (this.#headIndex > 100 && this.#headIndex > this.#elements.length >> 1) {
      const newLength = (this.#elements.length = this.#headIndex);
      this.#elements.copyWithin(0, this.#headIndex);
      this.#elements.length = newLength;
      this.#headIndex = 0;
    }

    return result;
  }
}
