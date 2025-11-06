// import { TypeId, Unique } from "../mod.ts";

// export type None = Unique<TypeId.OptionNone>;

// export type Some<T> = Unique<TypeId.OptionSome> & {
//   readonly value: T
// };

// export type Option<T> = None | Some<T>;

// export const None = (): None => {
//   return {
//     typeId: TypeId.OptionNone,
//   };
// };

// export const Some = <T>(value: T): Some<T> => {
//   return {
//     typeId: TypeId.OptionSome,
//     value,
//   }
// };

// export const isSome = <T>(me: Option<T>): me is Some<T> => {
//   return me.typeId === TypeId.OptionSome;
// };

// export const isNone = <T>(me: Option<T>): me is None => {
//   return me.typeId === TypeId.OptionNone;
// };

import { TypeId, Unique } from "../mod.ts";

export class None implements Unique {
  readonly typeId = TypeId.OptionNone;

  private constructor() {}

  static new() {
    return new None();
  }
}

export class Some<T> implements Unique {
  readonly typeId = TypeId.OptionSome;

  private constructor(private readonly _value: T) {}

  static new<T>(value: T): Some<T> {
    return new Some(value);
  }

  value(): T {
    return this._value
  }
}

export type Option<T> = None | Some<T>;

export const isSome = <T>(me: Option<T>): me is Some<T> => {
  return me.typeId === TypeId.OptionSome;
};

export const isNone = <T>(me: Option<T>): me is None => {
  return me.typeId === TypeId.OptionNone;
};