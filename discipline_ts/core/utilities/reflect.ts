interface NullReflection {
  readonly variant: ReflectionVariant.Null,
  readonly reflectedType: null,
}

interface UndefinedReflection {
  readonly variant: ReflectionVariant.Undefined,
  readonly reflectedType: undefined,
}

interface NumberReflection {
  readonly variant: ReflectionVariant.Number,
  readonly reflectedType: number,
}

interface IntegerReflection {
  readonly variant: ReflectionVariant.Integer,
  readonly reflectedType: number,
}

interface BooleanReflection<ReflectedType extends boolean = boolean> {
  readonly variant: ReflectionVariant.Boolean,
  readonly reflectedType: ReflectedType,
}

interface StringReflection<ReflectedType extends string = string> {
  readonly variant: ReflectionVariant.String,
  readonly reflectedType: ReflectedType,
}

interface ArrayReflection<ReflectedItemType> {
  readonly variant: ReflectionVariant.Array,
  readonly reflectedType: ReflectedItemType[],
  readonly itemReflection: It<ReflectedItemType>,
}

interface ObjectPropertyReflection<Object, Name, Value> {
  readonly name: Name, 
  readonly value: (it: Object) => Value,
  readonly reflection: It<Value>
}

interface ObjectReflection<
  ReflectedType,
  Properties extends ObjectPropertyReflection<ReflectedType, unknown, unknown>[] = ObjectPropertyReflection<ReflectedType, unknown, unknown>[],
> {
  readonly variant: ReflectionVariant.Object,
  readonly properties: Properties,
}

interface ClassReflection<
  ReflectedType,
  DataProperties extends ObjectPropertyReflection<ReflectedType, unknown, unknown>[] = ObjectPropertyReflection<ReflectedType, unknown, unknown>[],
> {
  readonly variant: ReflectionVariant.Object,
  readonly name: string,
  readonly dataProperties: DataProperties,
}

interface WrapperReflection<ReflectedType, InnerType> {
  readonly variant: ReflectionVariant.Wrapper,
  readonly getInner: (outer: ReflectedType) => InnerType,
  // readonly innerReflection: 
}

type It<Reflection> = any;


export type AnyReflection2 = (
  | NullReflection
  | UndefinedReflection
  | NumberReflection
  | IntegerReflection
  | StringReflection
  | BooleanReflection
  | ArrayReflection<unknown>
  | ObjectReflection<unknown> 
  | ClassReflection<ObjectReflection<unknown>> 
  | WrapperReflection<unknown, AnyReflection>
);


const BRAND = Symbol();

export type Reflection<Value> = {
  match: <R1, R2, R3, R4, R5, R6, R7, R8, R9, R10>(
    reflection,
    {

    }: {
      Null: (reflection: Null) => R1,
      Undefined: (reflection: Undefined) => R2,
      Number: (reflection: Number) => R3,
      Integer: (reflection: Integer) => R4,
      String: (reflection: String) => R5,
      Boolean: (reflection: Boolean) => R6,
      Array: (reflection: Array<unknown>) => R7,
      Object: (reflection: Object<unknown>) => R8,
      Class: (reflection: Class<Object<unknown>>) => R9,
      Wrapper: (reflection: Wrapper<unknown, unknown>) => R10,
    },
  ) => {

  } 
};

export interface IsReflection<T> {
  readonly virtualPropertyForReflectedDataType: T,
  readonly brand: ReflectionVariant,
}

const brand = Symbol();

export interface IsReflection2 {
  [brand]: "reflection";
}

const withIt = <T>(it: T): T & IsReflection2 => {
  return it as any;
};

export type AnyReflection = (
  | Null 
  | Undefined
  | Number 
  | Integer
  | String 
  | Boolean 
  | Array<AnyReflection>
  | Object<unknown> 
  | Class<Object<unknown>> 
  | Wrapper<unknown, AnyReflection>
);

export type GetReflectedType<Reflection extends AnyReflection> = (
  Reflection extends Null 
    ? null
    : Reflection extends Undefined
      ? undefined
      : Reflection extends Number | Integer
        ? number
        : Reflection extends Boolean
          ? boolean
          : Reflection extends String
            ? string
            : Reflection extends Array<infer ItemReflection>
              ? GetReflectedType<ItemReflection>[]
              : Reflection extends Object<infer It>
                ? It
                : Reflection extends Class<Object<infer It>>
                  ? It
                  : Reflection extends Wrapper<infer Outer, AnyReflection>
                    ? Outer
                    : never
);

export type Reflect<T> = (
  T extends null 
      ? Null 
      : T extends undefined
        ? Undefined
        : T extends number 
          ? Number | Integer
          : T extends string 
            ? String 
            : T extends boolean 
              ? Boolean 
              : [T] extends [(infer Item)[]] 
                ? Array<Item>
                : T extends Record<infer Key, infer Value>
                  ? Object<T> | Class<Object<T>> | Wrapper<T, unknown>
                  : Null | Undefined | Number | Integer | String | Array<unknown> | Object<unknown> | Class<Object<unknown>>
);

export type ReflectMatchCases<Return> = {
  Null: (reflection: Null) => Return,
  Undefined: (reflection: Undefined) => Return,
  Integer: (reflection: Integer) => Return,
  Number: (reflection: Number) => Return,
  Boolean: (reflection: Boolean) => Return,
  String: (reflection: String) => Return,
  Array: (reflection: Array<unknown>) => Return,
  Object: (reflection: Object<unknown>) => Return,
  Class: (reflection: Class<Object<unknown>>) => Return,
};

export type MatchCases2<Reflection extends AnyReflection, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10> = ({
  Null: (
    Reflection extends Null
      ? (reflection: Null, value: null) => R1
      : (reflection: Null, value: never) => R1
  ),
  Undefined: (
    Reflection extends Undefined
      ? (reflection: Undefined, value: undefined) => R2
      : (reflection: Undefined, value: never) => R2
  ),
  Integer: (
    Reflection extends Integer
      ? (reflection: Integer, value: number) => R3
      : (reflection: Integer, value: never) => R3
  ),
  Number: (
    Reflection extends Number
      ? (reflection: Number, value: number) => R4
      : (reflection: Number, value: never) => R4
  ),
  Boolean: (
    Reflection extends Boolean
      ? (reflection: String, value: boolean) => R5
      : (reflection: String, value: never) => R5
  ),
  String: (
    Reflection extends String
      ? (reflection: String, value: string) => R6
      : (reflection: String, value: never) => R6
  ),
  Array: (
    Reflection extends Array<infer Item>
      ? (reflection: Array<Item>, value: Item[]) => R7
      : (reflection: Array<never>, value: never[]) => R7
  ),
  Object: (
    Reflection extends Object<infer It>
      ? (reflection: Object<It>, value: It) => R8
      : (reflection: Object<never>, value: never) => R8
  ),
  Class: (
    Reflection extends Object<infer It>
      ? (reflection: Class<Object<It>>, value: It) => R9
      : (reflection: Class<Object<never>>, value: never) => R9 
  ),
  Wrapper: (
    Reflection extends Wrapper<infer Outer, infer Inner>
      ? (reflection: Wrapper<Outer, Inner>, value: Outer) => R10
      : (reflection: Wrapper<never, never>, value: never) => R10
  )
});

export type X<Return, ReflectedObject, ReflectedArrayItem> = ({
  Null: (value: null, reflection: Null) => Return,
  Undefined: (value: undefined, reflection: Undefined) => Return,
  Integer: (value: number, reflection: Integer) => Return,
  Number: (value: number, reflection: Number) => Return,
  Boolean: (value: boolean, reflection: Boolean) => Return,
  String: (value: string, reflection: String) => Return,
  Array: (value: ReflectedArrayItem[], reflection: Array<ReflectedArrayItem>) => Return,
  Object: (value: ReflectedObject, reflection: Object<ReflectedObject>) => Return,
  Class: (value: ReflectedObject, reflection: Class<Object<ReflectedObject>>) => Return,
});

type NullMatchCase<Return> = {
  readonly Null: (value: null, reflection: Null) => Return,
};
type UndefinedMatchCase<Return> = {
  readonly Undefined: (value: undefined, reflection: Undefined) => Return,
};
type IntegerMatchCase<Return> = {
  readonly Integer: (value: number, reflection: Integer) => Return,
};
type NumberMatchCase<Return> = {
  readonly Number: (value: number, reflection: Number) => Return,
};
type BooleanMatchCase<Return> = {
  readonly Boolean: (value: boolean, reflection: Boolean) => Return,
};
type StringMatchCase<Return> = {
  readonly String: (value: string, reflection: String) => Return,
};
type ArrayMatchCase<Item, Return> = {
  readonly Array: (value: Item[], reflection: Array<Item>) => Return,
};
type ObjectMatchCase<Reflected, Return> = {
  readonly Object: (value: Reflected, reflection: Object<Reflected>) => Return,
};
type ClassMatchCase<Reflected, Return> = {
  readonly Class: (value: Reflected, reflection: Class<Object<Reflected>>) => Return,
};
type WrapperMatchCase<ReflectedOuter, RefletedInner, Return> = {
  readonly Wrapper: (value: ReflectedOuter, reflection: Wrapper<ReflectedOuter, RefletedInner>) => Return,
};

type AnyMatchCase<Return> = (
  | NullMatchCase<Return>
  | UndefinedMatchCase<Return>
  | NumberMatchCase<Return>
  | IntegerMatchCase<Return>
  | BooleanMatchCase<Return>
  | StringMatchCase<Return>
  | ArrayMatchCase<unknown, Return>
  | ObjectMatchCase<unknown, Return>
  | ClassMatchCase<unknown, Return>
  | WrapperMatchCase<unknown, unknown, Return>
);

export type ReflectMatchCasesWithValue<Value, Return> = (
  Value extends null 
      ? NullMatchCase<Return>
      : Value extends undefined
        ? UndefinedMatchCase<Return>
        : Value extends number 
          ? NumberMatchCase<Return> | IntegerMatchCase<Return>
          : Value extends string 
            ? StringMatchCase<Return>
            : Value extends boolean 
              ? BooleanMatchCase<Return>
              : [Value] extends [(infer Item)[]] 
                ? ArrayMatchCase<Item, Return>
                : Value extends Record<infer Key, infer Property> 
                  ? WrapperMatchCase<Value, unknown, Return> | ObjectMatchCase<Value, Return> | ClassMatchCase<Value, Return>
                  : AnyMatchCase<Return>
);

const x: ReflectMatchCasesWithValue<null, "dark"> = {
  Null: () => "dark"
};

export const Reflect = {
  match: <Return>(
    me: Reflect<unknown>,
    cases: ReflectMatchCases<Return>,
  ): Return => {
    switch (me.brand) {
      case ReflectionVariant.Null: return cases.Null(me);
      case ReflectionVariant.Undefined: return cases.Undefined(me);
      case ReflectionVariant.Number: return cases.Number(me);
      case ReflectionVariant.Integer: return cases.Integer(me);
      case ReflectionVariant.String: return cases.String(me);
      case ReflectionVariant.Array: return cases.Array(me);
      case ReflectionVariant.Object: return cases.Object(me);
      case ReflectionVariant.Class: return cases.Class(me);
    }
  },

  match2: <Reflection extends AnyReflection, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10>(
    me: Reflection,
    value: Reflection["virtualPropertyForReflectedDataType"],
    cases: MatchCases2<Reflection, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10>,
  ): R1 | R2 | R3 | R4 | R5 | R6 | R7 | R8 | R9 | R10 => {
    switch (me.brand) {
      case ReflectionVariant.Null: return cases.Null(me, value);
      case ReflectionVariant.Undefined: return cases.Undefined(me, value);
      case ReflectionVariant.Number: return cases.Number(me, value);
      case ReflectionVariant.Integer: return cases.Integer(me, value);
      case ReflectionVariant.String: return cases.String(me, value);
      case ReflectionVariant.Array: return cases.Array(me, value);
      case ReflectionVariant.Object: return cases.Object(me, value);
      case ReflectionVariant.Class: return cases.Class(me, value);
      case ReflectionVariant.Wrapper: return cases.Wrapper(me, value);
      case ReflectionVariant.Boolean: return cases.Boolean(me, value);
    }
  },

  isNull: (me: AnyReflection2): me is NullReflection => {
    return me.variant === ReflectionVariant.Null;
  },
  isUndefined: (me: AnyReflection2): me is UndefinedReflection => {
    return me.variant === ReflectionVariant.Undefined;
  },
  isNumber: (me: AnyReflection2): me is NumberReflection => {
    return me.variant === ReflectionVariant.Number;
  },
  isInteger: (me: AnyReflection2): me is IntegerReflection => {
    return me.variant === ReflectionVariant.Integer;
  },
  isBoolean: (me: AnyReflection2): me is BooleanReflection => {
    return me.variant === ReflectionVariant.Boolean;
  },
  isString: (me: AnyReflection2): me is StringReflection => {
    return me.variant === ReflectionVariant.String;
  },
  isArray: (me: AnyReflection2): me is ArrayReflection<unknown> => {
    return me.variant === ReflectionVariant.Array;
  },
  isObject: <Reflection extends AnyReflection>(me: Reflection): me is Object<unknown> => {
    return me.brand === ReflectionVariant.Object;
  },
  isClass: <Reflection extends AnyReflection>(me: Reflection): me is Class<Object<unknown>> => {
    return me.brand === ReflectionVariant.Class;
  },
  isWrapper: <Reflection extends AnyReflection>(me: Reflection): me is Wrapper<unknown, unknown> => {
    return me.brand === ReflectionVariant.Wrapper;
  },
}

const withVirtualKey = <Key extends string | number | symbol, Value, Object>(
  key: Key, 
  object: Object,
): Object & { [key in Key]: Value } => {
  return object as Object & { [key in Key]: Value };
};

const construct = <ReflectedDataType, ActualReflection>(actualReflection: ActualReflection): IsReflection<ReflectedDataType> & ActualReflection => {
  return actualReflection as IsReflection<ReflectedDataType> & ActualReflection;
};

export const enum ReflectionVariant {
  Null,
  Undefined,
  Number,
  Integer,
  Boolean,
  String,
  Array,
  Object,
  Wrapper,
  Class,
}

export interface Null extends IsReflection2 {
  readonly brand: ReflectionVariant.Null,
}

export const Null = {
  create: (): Null => {
    return withIt({
      brand: ReflectionVariant.Null,
    });
  },
};

export interface Undefined extends IsReflection2 {
  readonly brand: ReflectionVariant.Undefined,
}

export const Undefined = {
  create: (): Undefined => {
    return withIt({
      brand: ReflectionVariant.Undefined,
    });
  },
};

export interface Number extends IsReflection2 {
  readonly brand: ReflectionVariant.Number,
}

export const Number = {
  create: (): Number => {
    return withIt({
      brand: ReflectionVariant.Number,
    });
  },
};

export interface Integer extends IsReflection2 {
  readonly brand: ReflectionVariant.Integer,
}

export const Integer = {
  create: (): Integer => {
    return withIt({
      brand: ReflectionVariant.Integer,
    });
  },
};

export interface Boolean extends IsReflection2 {
  readonly brand: ReflectionVariant.Boolean,
}

export const Boolean = {
  create: (): Boolean => {
    return withIt({
      brand: ReflectionVariant.Boolean,
    });
  },
};

export interface String extends IsReflection2 {
  readonly brand: ReflectionVariant.String,
}

export const String = {
  create: (): String => {
    return withIt({
      brand: ReflectionVariant.String,
    });
  },
};

export interface Array<ItemReflection extends AnyReflection> extends IsReflection2 {
  readonly brand: ReflectionVariant.Array,
  readonly itemReflection: ItemReflection,
}

export const Array = {
  create: <ItemReflection extends AnyReflection>(itemReflection: ItemReflection): Array<ItemReflection> => {
    return withIt({
      brand: ReflectionVariant.Array,
      itemReflection,
    });
  },
};

export type PropertyName = string | number | symbol;

export interface ObjectProperty<Object, Name extends PropertyName, Value> {
  readonly name: Name,
  readonly valueGetter: (it: Object) => Value,
  readonly valueReflection: IsReflection<Value>,
}

export const ObjectProperty = {
  createFromNamedArguments: <Object, Name extends PropertyName, Value>({
    name,
    valueGetter, 
    valueReflection,
  }: {
    readonly name: Name,
    readonly valueGetter: (it: Object) => Value,
    readonly valueReflection: IsReflection<Value>,
  }): ObjectProperty<Object, Name, Value> => {
    return {
      name,
      valueGetter, 
      valueReflection,
    };
  },
};

export type GenericPropertyList =  [PropertyName, unknown][];

export type ToPropertyList<Object, Properties extends GenericPropertyList> = {
  [K in keyof Properties]: Properties[K] extends [infer Name, infer Value] 
    ? Name extends PropertyName
      ? ObjectProperty<Object, Name, Value>
      : never
    : never
};

export interface Object<It, Properties extends GenericPropertyList = GenericPropertyList> extends IsReflection2 {
  readonly brand: ReflectionVariant.Object,
  readonly properties: ToPropertyList<It, Properties>,
}

export const Object = {
  construct: <It, Properties extends GenericPropertyList>(
    properties: ToPropertyList<It, Properties>,
  ): Object<It, Properties> => {
    return withIt({
      brand: ReflectionVariant.Object,
      properties,
    });
  },

  createEmpty: <It>(): Object<It, []> => {
    return Object.construct([]);
  },

  createFromProperties: <It, Properties extends GenericPropertyList = []>({
    properties,
  }: {
    properties: ToPropertyList<It, Properties>,
  }): Object<It, Properties> => {
    return Object.construct(properties);
  },

  withProperty: <
    It, 
    Properties extends GenericPropertyList, 
    Name extends PropertyName,
    Value,
  >(
    it: Object<It, Properties>,
    name: Name,
    valueGetter: (it: It) => Value,
    valueReflection: IsReflection<Value>,
  ): Object<It, [...Properties, [Name, Value]]> => {
    return Object.construct([
      ...it.properties,
      ObjectProperty.createFromNamedArguments({
        name,
        valueGetter,
        valueReflection,
      }),
    ] as any);
  },
};

export interface Wrapper<Outer, Inner extends AnyReflection> extends IsReflection2 {
  readonly brand: ReflectionVariant.Wrapper,
  readonly name: string,
  readonly innerGetter: (outer: Outer) => GetReflectedType<Inner>,
  readonly innerReflection: Inner,
}

export const Wrapper = {
  createFromNamedArguments: <Outer, Inner>({
    name,
    innerGetter, 
    innerReflection,
  }: {
    readonly name: string,
    readonly innerGetter: (outer: Outer) => Inner,
    readonly innerReflection: IsReflection<Inner>,
  }): Wrapper<Outer, Inner> => {
    return withIt({
      brand: ReflectionVariant.Wrapper,
      name,
      innerGetter, 
      innerReflection,
    });
  },
};

export interface Class<DataProperties extends Object<unknown>> extends IsReflection2 {
  readonly brand: ReflectionVariant.Class,
  readonly name: string,
  readonly reflection: DataProperties,
}

export const Class = {
  createFromNamedArguments: <Reflection extends Object<unknown>>({
    name,
    reflection,
  }: {
    name: string,
    reflection: Reflection,
  }): Class<Reflection> => {
    return withIt({
      brand: ReflectionVariant.Class,
      name,
      reflection,
    });
  },
};


// type typeDebugger<T> = T extends any ? T : never

// export type ReflectionOf<T> = (
//   // Primitive types
//   T extends null
//     ? Null
//     : T extends undefined
//       ? Undefined
//       : T extends number
//         ? Number
//         : T extends boolean
//           ? Boolean
//           : T extends string
//             ? String
//             // Array type - using tuple wrapper to prevent distribution
//             : [T] extends [(infer Item)[]]
//               ? Array<Item>
//               // Object type - record with string keys and any values
//               : T extends Record<string, unknown>
//                 ? Object<T, ToGenericPropertyList<T>>
//                 // My issue is here. if `T` is a class instance, then `ReflectionOf` returns `never`. 
//                 // Help me make it return something usefull. 
//                 : never

//                 // // Function type
//                 // : T extends (...args: any[]) => any
//                 //   ? FunctionReflection<T>
//                 //   // Symbol type
//                 //   : T extends symbol
//                 //     ? SymbolReflection
//                 //     // BigInt type
//                 //     : T extends bigint
//                 //       ? BigIntReflection
//                 //       // Promise type
//                 //       : [T] extends [Promise<infer U>]
//                 //         ? PromiseReflection<U>
//                 //         // Date type
//                 //         : T extends Date
//                 //           ? DateReflection
//                 //           // RegExp type
//                 //           : T extends RegExp
//                 //             ? RegExpReflection
//                 //             // Map type
//                 //             : [T] extends [Map<infer K, infer V>]
//                 //               ? MapReflection<K, V>
//                 //               // Set type
//                 //               : [T] extends [Set<infer U>]
//                 //                 ? SetReflection<U>
//                 //                 // Fallback for any other type
//                 //                 : UnknownReflection
// );

// export type GetReflectionForType<T> = (
//   // Primitive types
//   T extends null
//     ? NullReflection
//     : T extends undefined
//       ? UndefinedReflection
//       : T extends number
//         ? NumberReflection
//         : T extends boolean
//           ? BooleanReflection
//           : T extends string
//             ? StringReflection
//             // Array type - using tuple wrapper to prevent distribution
//             : [T] extends [(infer Item)[]]
//               ? ArrayReflection<Item>
//               // Object type - record with string keys and any values
//               : T extends Record<string, any>
//                 ? ObjectReflection<T, ObjectProperty<>>
//                 // // Function type
//                 // : T extends (...args: any[]) => any
//                 //   ? FunctionReflection<T>
//                 //   // Symbol type
//                 //   : T extends symbol
//                 //     ? SymbolReflection
//                 //     // BigInt type
//                 //     : T extends bigint
//                 //       ? BigIntReflection
//                 //       // Promise type
//                 //       : [T] extends [Promise<infer U>]
//                 //         ? PromiseReflection<U>
//                 //         // Date type
//                 //         : T extends Date
//                 //           ? DateReflection
//                 //           // RegExp type
//                 //           : T extends RegExp
//                 //             ? RegExpReflection
//                 //             // Map type
//                 //             : [T] extends [Map<infer K, infer V>]
//                 //               ? MapReflection<K, V>
//                 //               // Set type
//                 //               : [T] extends [Set<infer U>]
//                 //                 ? SetReflection<U>
//                 //                 // Fallback for any other type
//                 //                 : UnknownReflection
// );
