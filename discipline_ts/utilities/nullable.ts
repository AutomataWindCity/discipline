
// export const NONE_BRAND = Symbol();
// export const SOME_BRAND = Symbol();

export type NullableNone = null;
export type NullableSome<T> = T;
export type Nullable<T> = NullableNone | NullableSome<T>;

const map = <Value, NewValue = Value>(it: Nullable<Value>, fn: (value: Value) => NewValue): Nullable<NewValue> => {
  return it === null
    ? null
    : fn(it)
};

export const Nullable = {
  map,
};