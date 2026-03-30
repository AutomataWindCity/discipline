const SUCCESS_CODE = Symbol();
type SuccessCode = typeof SUCCESS_CODE;

const FAILURE_CODE = Symbol();
type FailureCode = typeof FAILURE_CODE;

type TriedCode = SuccessCode | FailureCode;

interface Writer {

}

interface Description<T> {
  readonly serialize: (value: T, writer: Writer) => TriedCode,
}

interface Null extends Description<null> {
  
}

export const Null = {
  create: (): Null => {
    return {
      serialize: () => {
        return SUCCESS_CODE;
      },
    };
  },
};

interface Undefined extends Description<undefined> {
  
}

export const Undefined = {
  create: (): Undefined => {
    return {
      serialize: () => {
        return SUCCESS_CODE;
      },
    };
  },
};

interface String extends Description<string> {
  
}

export const String = {
  create: (): String => {
    return {
      serialize: () => {
        return SUCCESS_CODE;
      },
    };
  },
};

interface Number extends Description<number> {
  
}

export const Number = {
  create: (): Number => {
    return {
      serialize: () => {
        return SUCCESS_CODE;
      },
    };
  },
};

interface Boolean extends Description<boolean> {
  
}

export const Boolean = {
  create: (): Boolean => {
    return {
      serialize: () => {
        return SUCCESS_CODE;
      },
    };
  },
};

interface Array<Item> extends Description<Item[]> {
  itemDescription: Description<Item>,
}

export const Array = {
  create: <Item>({
    itemDescription,
  }: {
    itemDescription: Description<Item>,
  }): Array<Item> => {
    return {
      itemDescription,

      serialize: () => {
        return SUCCESS_CODE;
      },
    };
  },
};

interface ObjectProperty<Object, Value> {
  name: string,
  get: (object: Object) => Value,
  description: Description<Value>,
}

export const ObjectProperty = {
  create: <Object, Value>({
    name,
    getter: get,
    description,
  }: {
    name: string,
    getter: (it: Object) => Value,
    description: Description<Value>
  }): ObjectProperty<Object, Value> => {
    return {
      name,
      get,
      description,
    };
  },
};

export const ObjectProperties = {
  create: <Object, P extends unknown[]>(
    ...p: {
      [Key in keyof P]: ObjectProperty<Object, P[Key]>
    }
  ): { [Key in keyof P]: ObjectProperty<Object, P[Key]> } => {
    return p;
  },
};

interface Object<Object, Properties extends [string, unknown][] = [string, unknown][]> extends Description<Object> {
  readonly properties: {
    [Key in keyof Properties]: ObjectProperty<Object, Properties[Key]>[]
  }
}

export const Object = {
  createFinal: <It>({
    properties,
  }: {
    properties: ObjectProperty<It, any>[]
  }) => {},

  create: <It>(): Object<It> => {
    return {
      properties: [],

      serialize: () => {
        return SUCCESS_CODE;
      },
    };
  },

  pushProperty: <It, Value>(
    it: Object<It>, 
    name: string,
    getter: (it: It) => Value,
    description: Description<Value>
  ): Object<It> => {
    it.properties.push(ObjectProperty.create({
      name,
      getter: getter,
      description,
    }) as any);

    return it;
  },
};

const enum EnumVariantType {
  Unit,
  Data,
}

interface EnumUnitVariant {
  readonly type: EnumVariantType.Unit,
}

export const EnumUnitVariant = {
  create: (): EnumUnitVariant => {
    return {
      type: EnumVariantType.Unit,
    };
  },
};

interface EnumDataVariant<Data> {
  readonly type: EnumVariantType.Data,
  readonly dataDescription: Description<Data>,
}

export const EnumDataVariant = {
  create: <Data>({
    dataDescription,
  }: {
    dataDescription: Description<Data>,
  }): EnumDataVariant<Data> => {
    return {
      type: EnumVariantType.Data,
      dataDescription,
    };
  },
};

interface Enum<Value, Variants extends Record<string, unknown>> extends Description<Value> {
  readonly name: string,

  readonly variants: {
    readonly [Key in keyof Variants]: Description<Variants[Key]>
  },

  readonly match: <Return>(
    value: Value, 
    variants: Variants,
    onUnit: () => Return,
    onData: <Data>(data: Data, variant: EnumDataVariant<Data>) => Return,
  ) => Return,
}

export const Enum = {
  create: <Value, Variants extends Record<string, unknown>>({
    name,
    match,
    variants,
  }: {
    name: string,
    variants: {
      [Key in keyof Variants]: Description<Variants[Key]>
    },
    match: <Return>(
      value: Value, 
      variants: Variants,
      onUnit: () => Return,
      onData: <Data>(data: Data, variant: EnumDataVariant<Data>) => Return,
    ) => Return,
  }): Enum<Value, Variants> => {
    return {
      name,
      match,
      variants,
      serialize: () => {
        return SUCCESS_CODE;
      },
    };
  },
};