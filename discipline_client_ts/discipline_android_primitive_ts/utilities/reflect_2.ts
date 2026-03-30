import {} from "./async.ts" with { type: "comptime" };

export const enum Type {
  Null,
  String,
  Number,
  Boolean,
  Array,
  Object,
  Unknown,
}

class Null {
  readonly type = Type.Null;

  private constructor() {}

  static create() {
    return new Null();
  }
}

class String {

}

type Reflection<Value> = {
  readonly type: "Null"
  readonly access: (value: Value) => Value extends null ? null : never,
} | {
  readonly type: "Undefined"
  readonly access: (value: Value) => Value extends undefined ? undefined : never,
} | {
  readonly type: "String"
  readonly access: (value: Value) => Value extends string ? string : never,
} | {
  readonly type: "Number"
  readonly access: (value: Value) => Value extends number ? number : never,
} | {
  readonly type: "Boolean"
  readonly access: (value: Value) => Value extends boolean ? boolean : never,
} | {
  readonly type: "Array"
  readonly access: (value: Value) => unknown[],
  readonly itemReflection: Reflection<unknown>,
} | {
  readonly type: "Object",
  readonly properties: {
    readonly name: string,
    readonly getter: (it: Value) => unknown,
    readonly reflection: Reflection<unknown>,
  }[],
} | {
  readonly type: "NewType",
  readonly getter: (it: Value) => unknown,
  readonly reflection: Reflection<unknown>,
}



const x = <Value>(value: Value, reflection: Reflection<Value>) => {
  switch (reflection.type) {
    case "Null": {
      let it = reflection.access(value);
      break;
    }
    case "Undefined": {
      let it = reflection.access(value);
      break;
    }
    case "String": {
      let it = reflection.access(value);
      it.localeCompare("");
      break;
    }
    case "Number": {
      let it = reflection.access(value);
      break;
    }
    case "Boolean": {
      let it = reflection.access(value);
      break;
    }
    case "Array": {
      let it = reflection.access(value);
      break;
    }
    case "Object": {
      let str = "";

      for (const property of reflection.properties) {
        str += property.name;
        str += ": ";
        x(property.getter(value), property.reflection);
      }
      break;
    }
    case "NewType": {
      let it = reflection.access(value);
      break;
    }
  }
};


// export interface NullReflectionFactory {
//   readonly variant: ReflectionVariant.Null,
// }
// export interface UndefinedReflectionFactory {
//   readonly variant: ReflectionVariant.Undefined,
// }
// export interface NumberReflectionFactory {
//   readonly variant: ReflectionVariant.Number,
// }
// export interface StringReflectionFactory {
//   readonly variant: ReflectionVariant.String,
// }
// export interface BooleanReflectionFactory {
//   readonly variant: ReflectionVariant.Boolean,
// }
// export interface ArrayReflectionFactory {
//   readonly variant: ReflectionVariant.Array,
//   readonly itemReflectionFactory: AnyReflectionFactory,
// }
// export interface ObjectPropertyReflectionFactory {
//   readonly name: string,
// }
// export interface ObjectReflectionFactory {
//   readonly variant: ReflectionVariant.Object,
//   readonly properties: ObjectPropertyReflectionFactory[],
// }
// export interface ClassReflectionFactory {
//   readonly variant: ReflectionVariant.Class,
//   readonly properties: ObjectPropertyReflectionFactory[],
// }
// export interface NewTypeReflectionFactory {
//   readonly variant: ReflectionVariant.NewType,
//   readonly innerTypeReflectionFactory: AnyReflectionFactory[],
// }

// export type AnyReflectionFactory = (
//   | NullReflectionFactory
//   | UndefinedReflectionFactory
//   | NumberReflectionFactory
//   | StringReflectionFactory
//   | BooleanReflectionFactory
//   | ArrayReflectionFactory
// );

// export interface Visitor<It> {
//   readonly onNull: () => void,
//   readonly onUndefined: () => void,
//   readonly onString: (value: string) => void,
//   readonly onNumber: (value: number) => void,
//   readonly onBoolean: (value: boolean) => void,
//   readonly onOpenArray: () => void,
//   readonly onArrayItem: () => void,
//   readonly onCloseArray: () => void,
// }

// export const macro = () => {
//   return `
    
//     {
//       write: (it, context) => {
//         context.buffer.push(it);
//       },

//       serialize: (it) => {
//         let context = Context.create();
        
//         let jsonText;

//         try {
//           jsonText = JSON.stringify();
//         }
//       },
//     };
//   `
// };

export const enum ReflectionVariant {
  Null,
  Undefined,
  String,
  Number,
  Boolean,
  Array,
  Class,
  Object,
  NewType,
}

export interface IsReflection<Reflected> {
  readonly variant: ReflectionVariant,
  readonly reflected: Reflected,
}

export interface NullReflection {
  readonly variant: ReflectionVariant.Null,
  readonly reflected: null,
}

export interface UndefinedReflection {
  readonly variant: ReflectionVariant.Undefined,
  readonly reflected: undefined,
}

export interface StringReflection {
  readonly variant: ReflectionVariant.String,
  readonly reflected: string,
}

export interface NumberReflection {
  readonly variant: ReflectionVariant.Number,
  readonly reflected: number,
}

export interface BooleanReflection {
  readonly variant: ReflectionVariant.Boolean,
  readonly reflected: boolean,
}

export interface ArrayReflection<Item> {
  readonly variant: ReflectionVariant.Array,
  // readonly reflection: 
  readonly reflected: Item[],
}

export interface ObjectPropertyReflection<
  Object, 
  Name,
  Value,
> {
  readonly valueGetter: (object: Object) => Value,
}

export type GenericPropertyList = ObjectPropertyReflection<unknown, unknown, unknown>[];

export interface ObjectReflection<
  Object,
  DataProperties extends GenericPropertyList,
> {
  readonly variant: ReflectionVariant.Object,
  readonly reflected: Object,
  readonly properties: DataProperties,
}

export interface ClassReflection<
  Class,
  DataProperties extends GenericPropertyList,
> {
  readonly variant: ReflectionVariant.Class,
  readonly reflected: Class,
  readonly properties: DataProperties,
}

export interface WrapperReflection<
  Outer,
  Inner,
> {
  readonly variant: ReflectionVariant.NewType,
  readonly reflected: Outer,
  readonly innerGetter: (outer: Outer) => Inner,
  // readonly innerReflection: 
}

export type AnyReflection = (
  | NullReflection
  | UndefinedReflection
  | StringReflection
  | NumberReflection
  | BooleanReflection
  | ObjectReflection<unknown, GenericPropertyList>
  | ArrayReflection<unknown>
  | ClassReflection<unknown, GenericPropertyList>
  | WrapperReflection<unknown, unknown>
);

type Return  = ( null | string | boolean | number | Return[])

const toJson = (
  value: unknown, 
  reflection: AnyReflection,
): Return => {
  switch (reflection.variant) {
    case ReflectionVariant.Null: {
      return value as null;
    }
    case ReflectionVariant.Undefined: {
      throw new Error();
    }
    case ReflectionVariant.Number: {
      return value as number;
    }
    case ReflectionVariant.Boolean: {
      return value as boolean;
    }
    case ReflectionVariant.String: {
      return value as string;
    }
    case ReflectionVariant.Object: {
      return reflection
        .properties
        .map(property => toJson(
          property.valueGetter(value), 
          property.valueReflection,
        ));
    }
    case ReflectionVariant.Class: return reflection
      .properties
      .map(property => toJson(
        property.valueGetter(value),
        property.valueReflection,
      ));
    case ReflectionVariant.NewType: return toJson(
      reflection.innerGetter(value),
      reflection.innerReflection,
    );
  }
};