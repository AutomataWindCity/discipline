import { withVirtualKey } from "../mod.ts";

const BRAND = Symbol();

const TYPE_NONE = 0;
const TYPE_SOME = 1;

export type None = {
  readonly type: typeof TYPE_NONE,
  readonly [BRAND]: "Option.None",
};

export type Some<T> = {
  readonly type: typeof TYPE_SOME,
  readonly [BRAND]: "Option.Some",
  readonly value: T,
};

export const None = (): None => {
  return withVirtualKey(BRAND, {
    type: TYPE_NONE,
  });
};

export const Some = <T>(value: T): Some<T> => {
  return withVirtualKey(BRAND, {
    type: TYPE_SOME,
    value,
  });
};

export type Option<T> = None | Some<T>;

export const isSome = <T>(me: Option<T>): me is Some<T> => {
  return me.type === TYPE_SOME;
};

export const isNone = <T>(me: Option<T>): me is None => {
  return me.type === TYPE_NONE;
};

export const value = <T>(me: Some<T>): T => {
  return me.value;
};

// Core methods
export const unwrap = <T>(me: Option<T>): T => {
  if (isSome(me)) {
    return me.value;
  }

  throw new Error("Called `unwrap` on a `None` value");
};

export const unwrapOr = <T>(me: Option<T>, defaultValue: T): T => {
  return isSome(me) ? me.value : defaultValue;
};

export const unwrapOrElse = <T>(me: Option<T>, fn: () => T): T => {
  return isSome(me) ? me.value : fn();
};

export const expect = <T>(me: Option<T>, msg: string): T => {
  if (isSome(me)) {
    return me.value;
  }
  throw new Error(msg);
};

export const map = <T, U>(me: Option<T>, fn: (value: T) => U): Option<U> => {
  return isSome(me) ? Some(fn(me.value)) : None();
};

export const mapOr = <T, U>(me: Option<T>, defaultValue: U, fn: (value: T) => U): U => {
  return isSome(me) ? fn(me.value) : defaultValue;
};

export const mapOrElse = <T, U>(me: Option<T>, defaultFn: () => U, fn: (value: T) => U): U => {
  return isSome(me) ? fn(me.value) : defaultFn();
};

export const and = <T, U>(me: Option<T>, other: Option<U>): Option<U> => {
  return isSome(me) ? other : None();
};

export const andThen = <T, U>(me: Option<T>, fn: (value: T) => Option<U>): Option<U> => {
  return isSome(me) ? fn(me.value) : None();
};

export const or = <T>(me: Option<T>, other: Option<T>): Option<T> => {
  return isSome(me) ? me : other;
};

export const orElse = <T>(me: Option<T>, fn: () => Option<T>): Option<T> => {
  return isSome(me) ? me : fn();
};

export const filter = <T>(me: Option<T>, predicate: (value: T) => boolean): Option<T> => {
  return isSome(me) && predicate(me.value) ? me : None();
};

export const zip = <T, U>(me: Option<T>, other: Option<U>): Option<[T, U]> => {
  return isSome(me) && isSome(other) ? Some([me.value, other.value]) : None();
};

export const zipWith = <T, U, V>(
  me: Option<T>,
  other: Option<U>,
  fn: (a: T, b: U) => V
): Option<V> => {
  return isSome(me) && isSome(other) ? Some(fn(me.value, other.value)) : None();
};

export const unzip = <T, U>(me: Option<[T, U]>): [Option<T>, Option<U>] => {
  return isSome(me) ? [Some(me.value[0]), Some(me.value[1])] : [None(), None()];
};

// Boolean operations
export const contains = <T>(me: Option<T>, value: T): boolean => {
  return isSome(me) && me.value === value;
};

export const xor = <T>(me: Option<T>, other: Option<T>): Option<T> => {
  if (isSome(me) && isNone(other)) return me;
  if (isNone(me) && isSome(other)) return other;
  return None();
};

// Conversion methods
// export const okOr = <T, E>(me: Option<T>, err: E): Result<T, E> => {
//   return isSome(me) ? Ok(me.value) : Err(err);
// };

// export const okOrElse = <T, E>(me: Option<T>, errFn: () => E): Result<T, E> => {
//   return isSome(me) ? Ok(me.value) : Err(errFn());
// };

// Utility methods
export const flatten = <T>(me: Option<Option<T>>): Option<T> => {
  return isSome(me) ? me.value : None();
};

export const getOrInsert = <T>(me: Option<T>, value: T): T => {
  if (isNone(me)) {
    // This would require mutable options to be fully implemented
    // For immutable version, we return the value but cannot modify the original
    return value;
  }
  return me.value;
};

export const getOrInsertWith = <T>(me: Option<T>, fn: () => T): T => {
  if (isNone(me)) {
    return fn();
  }
  return me.value;
};