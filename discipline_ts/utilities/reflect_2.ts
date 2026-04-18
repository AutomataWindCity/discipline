// import {} from "./async.ts" with { type: "comptime" };

// export const enum Type {
//   Null,
//   String,
//   Number,
//   Boolean,
//   Array,
//   Object,
//   Unknown,
// }

// class Null {
//   readonly type = Type.Null;

//   private constructor() {}

//   static create() {
//     return new Null();
//   }
// }

// class String {

// }

// type Reflection<Value> = {
//   readonly type: "Null"
//   readonly access: (value: Value) => Value extends null ? null : never,
// } | {
//   readonly type: "Undefined"
//   readonly access: (value: Value) => Value extends undefined ? undefined : never,
// } | {
//   readonly type: "String"
//   readonly access: (value: Value) => Value extends string ? string : never,
// } | {
//   readonly type: "Number"
//   readonly access: (value: Value) => Value extends number ? number : never,
// } | {
//   readonly type: "Boolean"
//   readonly access: (value: Value) => Value extends boolean ? boolean : never,
// } | {
//   readonly type: "Array"
//   readonly access: (value: Value) => unknown[],
//   readonly itemReflection: Reflection<unknown>,
// } | {
//   readonly type: "Object",
//   readonly properties: {
//     readonly name: string,
//     readonly getter: (it: Value) => unknown,
//     readonly reflection: Reflection<unknown>,
//   }[],
// } | {
//   readonly type: "NewType",
//   readonly getter: (it: Value) => unknown,
//   readonly reflection: Reflection<unknown>,
// }



// const x = <Value>(value: Value, reflection: Reflection<Value>) => {
//   switch (reflection.type) {
//     case "Null": {
//       let it = reflection.access(value);
//       break;
//     }
//     case "Undefined": {
//       let it = reflection.access(value);
//       break;
//     }
//     case "String": {
//       let it = reflection.access(value);
//       it.localeCompare("");
//       break;
//     }
//     case "Number": {
//       let it = reflection.access(value);
//       break;
//     }
//     case "Boolean": {
//       let it = reflection.access(value);
//       break;
//     }
//     case "Array": {
//       let it = reflection.access(value);
//       break;
//     }
//     case "Object": {
//       let str = "";

//       for (const property of reflection.properties) {
//         str += property.name;
//         str += ": ";
//         x(property.getter(value), property.reflection);
//       }
//       break;
//     }
//     case "NewType": {
//       let it = reflection.access(value);
//       break;
//     }
//   }
// };


// // export interface NullReflectionFactory {
// //   readonly variant: ReflectionVariant.Null,
// // }
// // export interface UndefinedReflectionFactory {
// //   readonly variant: ReflectionVariant.Undefined,
// // }
// // export interface NumberReflectionFactory {
// //   readonly variant: ReflectionVariant.Number,
// // }
// // export interface StringReflectionFactory {
// //   readonly variant: ReflectionVariant.String,
// // }
// // export interface BooleanReflectionFactory {
// //   readonly variant: ReflectionVariant.Boolean,
// // }
// // export interface ArrayReflectionFactory {
// //   readonly variant: ReflectionVariant.Array,
// //   readonly itemReflectionFactory: AnyReflectionFactory,
// // }
// // export interface ObjectPropertyReflectionFactory {
// //   readonly name: string,
// // }
// // export interface ObjectReflectionFactory {
// //   readonly variant: ReflectionVariant.Object,
// //   readonly properties: ObjectPropertyReflectionFactory[],
// // }
// // export interface ClassReflectionFactory {
// //   readonly variant: ReflectionVariant.Class,
// //   readonly properties: ObjectPropertyReflectionFactory[],
// // }
// // export interface NewTypeReflectionFactory {
// //   readonly variant: ReflectionVariant.NewType,
// //   readonly innerTypeReflectionFactory: AnyReflectionFactory[],
// // }

// // export type AnyReflectionFactory = (
// //   | NullReflectionFactory
// //   | UndefinedReflectionFactory
// //   | NumberReflectionFactory
// //   | StringReflectionFactory
// //   | BooleanReflectionFactory
// //   | ArrayReflectionFactory
// // );

// // export interface Visitor<It> {
// //   readonly onNull: () => void,
// //   readonly onUndefined: () => void,
// //   readonly onString: (value: string) => void,
// //   readonly onNumber: (value: number) => void,
// //   readonly onBoolean: (value: boolean) => void,
// //   readonly onOpenArray: () => void,
// //   readonly onArrayItem: () => void,
// //   readonly onCloseArray: () => void,
// // }

// // export const macro = () => {
// //   return `
    
// //     {
// //       write: (it, context) => {
// //         context.buffer.push(it);
// //       },

// //       serialize: (it) => {
// //         let context = Context.create();
        
// //         let jsonText;

// //         try {
// //           jsonText = JSON.stringify();
// //         }
// //       },
// //     };
// //   `
// // };

// export const enum ReflectionVariant {
//   Null,
//   Undefined,
//   String,
//   Number,
//   Boolean,
//   Array,
//   Class,
//   Object,
//   NewType,
// }

// export interface IsReflection<Reflected> {
//   readonly variant: ReflectionVariant,
//   readonly reflected: Reflected,
// }

// export interface NullReflection {
//   readonly variant: ReflectionVariant.Null,
//   readonly reflected: null,
// }

// export interface UndefinedReflection {
//   readonly variant: ReflectionVariant.Undefined,
//   readonly reflected: undefined,
// }

// export interface StringReflection {
//   readonly variant: ReflectionVariant.String,
//   readonly reflected: string,
// }

// export interface NumberReflection {
//   readonly variant: ReflectionVariant.Number,
//   readonly reflected: number,
// }

// export interface BooleanReflection {
//   readonly variant: ReflectionVariant.Boolean,
//   readonly reflected: boolean,
// }

// export interface ArrayReflection<Item> {
//   readonly variant: ReflectionVariant.Array,
//   // readonly reflection: 
//   readonly reflected: Item[],
// }

// export interface ObjectPropertyReflection<
//   Object, 
//   Name,
//   Value,
// > {
//   readonly valueGetter: (object: Object) => Value,
// }

// export type GenericPropertyList = ObjectPropertyReflection<unknown, unknown, unknown>[];

// export interface ObjectReflection<
//   Object,
//   DataProperties extends GenericPropertyList,
// > {
//   readonly variant: ReflectionVariant.Object,
//   readonly reflected: Object,
//   readonly properties: DataProperties,
// }

// export interface ClassReflection<
//   Class,
//   DataProperties extends GenericPropertyList,
// > {
//   readonly variant: ReflectionVariant.Class,
//   readonly reflected: Class,
//   readonly properties: DataProperties,
// }

// export interface WrapperReflection<
//   Outer,
//   Inner,
// > {
//   readonly variant: ReflectionVariant.NewType,
//   readonly reflected: Outer,
//   readonly innerGetter: (outer: Outer) => Inner,
//   // readonly innerReflection: 
// }

// export type AnyReflection = (
//   | NullReflection
//   | UndefinedReflection
//   | StringReflection
//   | NumberReflection
//   | BooleanReflection
//   | ObjectReflection<unknown, GenericPropertyList>
//   | ArrayReflection<unknown>
//   | ClassReflection<unknown, GenericPropertyList>
//   | WrapperReflection<unknown, unknown>
// );

// type Return  = ( null | string | boolean | number | Return[])

// const toJson = (
//   value: unknown, 
//   reflection: AnyReflection,
// ): Return => {
//   switch (reflection.variant) {
//     case ReflectionVariant.Null: {
//       return value as null;
//     }
//     case ReflectionVariant.Undefined: {
//       throw new Error();
//     }
//     case ReflectionVariant.Number: {
//       return value as number;
//     }
//     case ReflectionVariant.Boolean: {
//       return value as boolean;
//     }
//     case ReflectionVariant.String: {
//       return value as string;
//     }
//     case ReflectionVariant.Object: {
//       return reflection
//         .properties
//         .map(property => toJson(
//           property.valueGetter(value), 
//           property.valueReflection,
//         ));
//     }
//     case ReflectionVariant.Class: return reflection
//       .properties
//       .map(property => toJson(
//         property.valueGetter(value),
//         property.valueReflection,
//       ));
//     case ReflectionVariant.NewType: return toJson(
//       reflection.innerGetter(value),
//       reflection.innerReflection,
//     );
//   }
// };

import { Nominal } from "../x.ts"

export const NONE_BRAND = Symbol();
export const SOME_BRAND = Symbol();

export type NullableNone = null;
export type NullableSome<T> = T;
export type Nullable<T> = NullableNone | NullableSome<T>;

// export const Option = {
//   None: (): None => {
//     return Nominal.create(NONE_BRAND, null);
//   },

//   Some: <T>(value: T): Some<T> => {
//     return Nominal.create(SOME_BRAND, value);
//   },

//   isSome: <T>(me: Option<T>): me is Some<T> => {
//     return me !== null;
//   },

//   isNone: <T>(me: Option<T>): me is None => {
//     return me === null;
//   },

//   value: <T>(me: Some<T>): T => {
//     return Nominal.get(me);
//   },

//   unwrap: <T>(me: Option<T>): T => {
//     if (Option.isSome(me)) {
//       return me.value;
//     }

//     throw new Error("Called `unwrap` on a `None` value");
//   },

//   unwrapOr: <T>(me: Option<T>, defaultValue: T): T => {
//     return Option.isSome(me) ? me.value : defaultValue;
//   },

//   unwrapOrElse: <T>(me: Option<T>, fn: () => T): T => {
//     return Option.isSome(me) ? me.value : fn();
//   },

//   expect: <T>(me: Option<T>, msg: string): T => {
//     if (Option.isSome(me)) {
//       return me.value;
//     }

//     throw new Error(msg);
//   },

//   map: <T, U>(me: Option<T>, fn: (value: T) => U): Option<U> => {
//     return Option.isSome(me) ? Option.Some(fn(me.value)) : Option.None();
//   },

//   mapOr: <T, U>(me: Option<T>, defaultValue: U, fn: (value: T) => U): U => {
//     return Option.isSome(me) ? fn(me.value) : defaultValue;
//   },

//   mapOrElse: <T, U>(me: Option<T>, defaultFn: () => U, fn: (value: T) => U): U => {
//     return Option.isSome(me) ? fn(me.value) : defaultFn();
//   },

//   and: <T, U>(me: Option<T>, other: Option<U>): Option<U> => {
//     return Option.isSome(me) ? other : Option.None();
//   },

//   andThen: <T, U>(me: Option<T>, fn: (value: T) => Option<U>): Option<U> => {
//     return Option.isSome(me) ? fn(me.value) : Option.None();
//   },

//   or: <T>(me: Option<T>, other: Option<T>): Option<T> => {
//     return Option.isSome(me) ? me : other;
//   },

//   orElse: <T>(me: Option<T>, fn: () => Option<T>): Option<T> => {
//     return Option.isSome(me) ? me : fn();
//   },

//   filter: <T>(me: Option<T>, predicate: (value: T) => boolean): Option<T> => {
//     return Option.isSome(me) && predicate(me.value) 
//       ? me 
//       : Option.None();
//   },

//   zip: <T, U>(me: Option<T>, other: Option<U>): Option<[T, U]> => {
//     return Option.isSome(me) && Option.isSome(other) 
//       ? Option.Some([me.value, other.value]) 
//       : Option.None();
//   },

//   zipWith: <T, U, V>(
//     me: Option<T>,
//     other: Option<U>,
//     fn: (a: T, b: U) => V
//   ): Option<V> => {
//     return Option.isSome(me) && Option.isSome(other) 
//       ? Option.Some(fn(me.value, other.value)) 
//       : Option.None();
//   },

//   unzip: <T, U>(me: Option<[T, U]>): [Option<T>, Option<U>] => {
//     return Option.isSome(me) 
//       ? [Option.Some(me.value[0]), Option.Some(me.value[1])] 
//       : [Option.None(), Option.None()];
//   },

//   // Boolean operations
//   contains: <T>(me: Option<T>, value: T): boolean => {
//     return Option.isSome(me) && me.value === value;
//   },

//   xor: <T>(me: Option<T>, other: Option<T>): Option<T> => {
//     if (Option.isSome(me) && Option.isNone(other)) {
//       return me;
//     }

//     if (Option.isNone(me) && Option.isSome(other)) {
//       return other;
//     }

//     return Option.None();
//   },

//   // Conversion methods
//   // export const okOr = <T, E>(me: Option<T>, err: E): Result<T, E> => {
//   //   return isSome(me) ? Ok(me.value) : Err(err);
//   // };

//   // export const okOrElse = <T, E>(me: Option<T>, errFn: () => E): Result<T, E> => {
//   //   return isSome(me) ? Ok(me.value) : Err(errFn());
//   // };

//   // Utility methods
//   flatten: <T>(me: Option<Option<T>>): Option<T> => {
//     return Option.isSome(me) ? me.value : Option.None();
//   },

//   getOrInsert: <T>(me: Option<T>, value: T): T => {
//     if (Option.isNone(me)) {
//       // This would require mutable options to be fully implemented
//       // For immutable version, we return the value but cannot modify the original
//       return value;
//     }
//     return me.value;
//   },

//   getOrInsertWith: <T>(me: Option<T>, fn: () => T): T => {
//     if (Option.isNone(me)) {
//       return fn();
//     }
//     return me.value;
//   },
// };

export const NullableNone = (): NullableNone => {
  return null;
};

export const NullableSome = <T>(value: T): NullableSome<T> => {
  return value;
};

export const isSome = <T>(me: Nullable<T>): me is NullableSome<T> => {
  return me !== null;
};

export const isNone = <T>(me: Nullable<T>): me is NullableNone => {
  return me === null;
};

export const value = <T>(me: NullableSome<T>): T => {
  return me;
};

export const unwrap = <T>(me: Nullable<T>): T => {
  if (isSome(me)) {
    return value(me);
  }

  throw new Error("Called `unwrap` on a `None` value");
};

export const unwrapOr = <T>(me: Nullable<T>, defaultValue: T): T => {
  return isSome(me) ? value(me) : defaultValue;
};

export const unwrapOrElse = <T>(me: Nullable<T>, fn: () => T): T => {
  return isSome(me) ? value(me) : fn();
};

export const expect = <T>(me: Nullable<T>, msg: string): T => {
  if (isSome(me)) {
    return value(me);
  }

  throw new Error(msg);
};

export const map = <T, U>(me: Nullable<T>, fn: (value: T) => U): Nullable<U> => {
  return isSome(me) ? NullableSome(fn(value(me))) : NullableNone();
};

export const mapOr = <T, U>(me: Nullable<T>, defaultValue: U, fn: (value: T) => U): U => {
  return isSome(me) ? fn(value(me)) : defaultValue;
};

export const mapOrElse = <T, U>(me: Nullable<T>, defaultFn: () => U, fn: (value: T) => U): U => {
  return isSome(me) ? fn(value(me)) : defaultFn();
};

export const and = <T, U>(me: Nullable<T>, other: Nullable<U>): Nullable<U> => {
  return isSome(me) ? other : NullableNone();
};

export const andThen = <T, U>(me: Nullable<T>, fn: (value: T) => Nullable<U>): Nullable<U> => {
  return isSome(me) ? fn(value(me)) : NullableNone();
};

export const or = <T>(me: Nullable<T>, other: Nullable<T>): Nullable<T> => {
  return isSome(me) ? me : other;
};

export const orElse = <T>(me: Nullable<T>, fn: () => Nullable<T>): Nullable<T> => {
  return isSome(me) ? me : fn();
};

export const filter = <T>(me: Nullable<T>, predicate: (value: T) => boolean): Nullable<T> => {
  return isSome(me) && predicate(value(me)) 
    ? me 
    : NullableNone();
};

export const zip = <T, U>(me: Nullable<T>, other: Nullable<U>): Nullable<[T, U]> => {
  return isSome(me) && isSome(other) 
    ? NullableSome([value(me), value(other)]) 
    : NullableNone();
};

export const zipWith = <T, U, V>(
  me: Nullable<T>,
  other: Nullable<U>,
  fn: (a: T, b: U) => V
): Nullable<V> => {
  return isSome(me) && isSome(other) 
    ? NullableSome(fn(value(me), value(other))) 
    : NullableNone();
};

export const unzip = <T, U>(me: Nullable<[T, U]>): [Nullable<T>, Nullable<U>] => {
  return isSome(me) 
    ? [NullableSome(value(me)[0]), NullableSome(value(me)[1])] 
    : [NullableNone(), NullableNone()];
};

export const contains = <T>(me: Nullable<T>, val: T): boolean => {
  return isSome(me) && value(me) === val;
};

export const xor = <T>(me: Nullable<T>, other: Nullable<T>): Nullable<T> => {
  if (isSome(me) && isNone(other)) {
    return me;
  }

  if (isNone(me) && isSome(other)) {
    return other;
  }

  return NullableNone();
};

export const flatten = <T>(me: Nullable<Nullable<T>>): Nullable<T> => {
  return isSome(me) ? value(me) : NullableNone();
};

export const getOrInsert = <T>(me: Nullable<T>, val: T): T => {
  if (isNone(me)) {
    return val;
  }
  return value(me);
};

export const getOrInsertWith = <T>(me: Nullable<T>, fn: () => T): T => {
  if (isNone(me)) {
    return fn();
  }
  return value(me);
};

// Individual exports
export const Nullable = { 
  None: NullableNone,
  Some: NullableSome,
  isSome,
  isNone,
  value,
  unwrap,
  unwrapOr,
  unwrapOrElse,
  expect,
  map,
  mapOr,
  mapOrElse,
  and,
  andThen,
  or,
  orElse,
  filter,
  zip,
  zipWith,
  unzip,
  contains,
  xor,
  flatten,
  getOrInsert,
  getOrInsertWith
};