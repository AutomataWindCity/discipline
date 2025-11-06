import { TEquality, TUnique, Unique } from "../mod.ts";

export const enum TOrdering {
  Less = -1,
  Equal = 0,
  Greater = 1,
}


export type TOrder<A, B = A> = TUnique<"Order", {
  compare(a: A, b: B): TOrdering;
  isEqualTo(a: A, b: B): boolean;
  isInequalTo(a: A, b: B): boolean;
  isLessThan(a: A, b: B): boolean;
  isGreaterThan(a: A, b: B): boolean;
  isLessThanOrEqualTo(a: A, b: B): boolean;
  isGreaterThanOrEqualTo(a: A, b: B): boolean;
  findMaximum(a: A, b: B): A | B;
  findMinimum(a: A, b: B): A | B;
  clamp(value: A, min: B, max: B): A | B;
  // {
    // if (this.cmp(min) === Order.Less) return min;
    // if (this.cmp(max) === Order.Greater) return max;
    // return this as unknown as T;
  // }
}>;

export const implement = <A, B = A>({
  equality, 
  compare,
  clamp,
  findMaximum,
  findMinimum,
  isGreaterThan,
  isGreaterThanOrEqualTo,
  isLessThan,
  isLessThanOrEqualTo,
}: {
  equality: TEquality<A, B>,
  compare(a: A, b: B): TOrdering;
  isLessThan?(a: A, b: B): boolean;
  isGreaterThan?(a: A, b: B): boolean;
  isLessThanOrEqualTo?(a: A, b: B): boolean;
  isGreaterThanOrEqualTo?(a: A, b: B): boolean;
  findMaximum?(a: A, b: B): A | B;
  findMinimum?(a: A, b: B): A | B;
  clamp?(value: A, min: A | B, max: A | B): A | B;
}): TOrder<A, B> => {
  return Unique.create({
    ...equality,
    compare,
    isLessThan: isLessThan ?? ((a, b) => {
      return compare(a, b) === TOrdering.Less;
    }),
    isLessThanOrEqualTo: isLessThanOrEqualTo ?? ((a, b) => {
      return compare(a, b) !== TOrdering.Greater;
    }),
    isGreaterThan: isGreaterThan ?? ((a, b) => {
      return compare(a, b) === TOrdering.Greater;
    }),
    isGreaterThanOrEqualTo: isGreaterThanOrEqualTo ?? ((a, b) => {
      return compare(a, b) !== TOrdering.Less;
    }),
    clamp: clamp ?? ((value, min, max) => {
      if ()
    }),
  });
};

// A base class to make implementation easier
abstract class BaseOrd<T> implements Ord<T> {
  abstract cmp(other: T): Order;

  lt(other: T): boolean {
    return this.cmp(other) === Order.Less;
  }

  gt(other: T): boolean {
    return this.cmp(other) === Order.Greater;
  }

  eq(other: T): boolean {
    return this.cmp(other) === Order.Equal;
  }

  le(other: T): boolean {
    return this.cmp(other) !== Order.Greater;
  }

  ge(other: T): boolean {
    return this.cmp(other) !== Order.Less;
  }

  max(other: T): T {
    return this.cmp(other) === Order.Greater ? (this as unknown as T) : other;
  }

  min(other: T): T {
    return this.cmp(other) === Order.Less ? (this as unknown as T) : other;
  }

  clamp(min: T, max: T): T {
    if (this.cmp(min) === Order.Less) return min;
    if (this.cmp(max) === Order.Greater) return max;
    return this as unknown as T;
  }
}

// Example: A tiny Number class for our smol automata boy!
class TinyNumber extends BaseOrd<TinyNumber> {
  constructor(public value: number) {
    super();
  }

  cmp(other: TinyNumber): Order {
    if (this.value < other.value) return Order.Less;
    if (this.value > other.value) return Order.Greater;
    return Order.Equal;
  }

  // For nice printing
  toString(): string {
    return `TinyNumber(${this.value})`;
  }
}

// Example: Even tinier String class!
class TinyString extends BaseOrd<TinyString> {
  constructor(public value: string) {
    super();
  }

  cmp(other: TinyString): Order {
    if (this.value < other.value) return Order.Less;
    if (this.value > other.value) return Order.Greater;
    return Order.Equal;
  }

  toString(): string {
    return `TinyString("${this.value}")`;
  }
}

// Utility functions for comparison
function partialCmp<T>(a: T, b: T): Order | null {
  if (a === b) return Order.Equal;
  if (a < b) return Order.Less;
  if (a > b) return Order.Greater;
  return null; // Incomparable
}

// Let's test it out, smol automata!
console.log("=== Testing Order System for Smol Automata Boy ===");

const num1 = new TinyNumber(5);
const num2 = new TinyNumber(10);
const num3 = new TinyNumber(5);

console.log(`${num1} cmp ${num2}:`, num1.cmp(num2)); // Less
console.log(`${num1} < ${num2}:`, num1.lt(num2));    // true
console.log(`${num1} > ${num2}:`, num1.gt(num2));    // false
console.log(`${num1} == ${num3}:`, num1.eq(num3));   // true

const str1 = new TinyString("apple");
const str2 = new TinyString("banana");

console.log(`\n${str1} cmp ${str2}:`, str1.cmp(str2)); // Less
console.log(`${str1}.max(${str2}):`, str1.max(str2));  // banana

// Clamp example
const middleNum = new TinyNumber(7);
console.log(`\n${middleNum}.clamp(${num1}, ${num2}):`, middleNum.clamp(num1, num2)); // TinyNumber(7)

// Sorting example
const numbers = [new TinyNumber(3), new TinyNumber(1), new TinyNumber(2)];
numbers.sort((a, b) => a.cmp(b));
console.log("\nSorted numbers:", numbers.map(n => n.value)); // [1, 2, 3]