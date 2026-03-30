import { Nominal, Nullable, TextualError, SUCCESS, FAILURE, SuccessCode, FailureCode, TriedCode } from "../x";

const INTEGER_BRAND = Symbol();
export type Integer = Nominal<typeof INTEGER_BRAND, number>;

const FLOAT_BRAND = Symbol();
export type Float = Nominal<typeof FLOAT_BRAND, number>;

export type SqliteValue = null | number | string;

const serializeNull = () => {
  return "NULL";
};

const serializeInteger = (number: number) => {
  if (Number.isInteger(number)) {
    return number.toString();
  } else {
    return FAILURE;
  }
};

const serializeReal = (number: number) => {
  if (Number.isFinite(number)) {
    return number.toString();
  } else {
    return FAILURE;
  }
};

const serializeString = (string: string) => {
  return `'${string.replaceAll(/'/g, "''")}'`;
};

const serializeBoolean = (boolean: boolean) => {
  if (boolean) {
    return "TRUE";
  } else {
    return "FALSE";
  }
};

export interface ScalarTypeWriter {
  buffer: string[],
}

export const ScalarTypeWriter = {
  writeNull: (it: ScalarTypeWriter) => {
    it.buffer.push(serializeNull());
  },

  fallableWriteInteger: (it: ScalarTypeWriter, number: number): TriedCode => {
    const stringOrError = serializeInteger(number);
    if (stringOrError === FAILURE) {
      return FAILURE;
    }

    it.buffer.push(stringOrError);
    return SUCCESS;
  },

  fallableWriteReal: (it: ScalarTypeWriter, number: number): TriedCode => {
    const stringOrError = serializeReal(number);
    if (stringOrError === FAILURE) {
      return FAILURE;
    }

    it.buffer.push(stringOrError);
    return SUCCESS;
  },

  writeBoolean: (it: ScalarTypeWriter, boolean: boolean) => {
    it.buffer.push(serializeBoolean(boolean));
  },

  writeString: (it: ScalarTypeWriter, string: string) => {
    it.buffer.push(serializeString(string));
  },
};

// export interface ScalarValueWrite<ScalarType> {
//   readonly name: string,
//   readonly write: (value: ScalarType, writer: ScalarTypeWriter, textualError: TextualError) => TriedCode;
// }

// export interface ScalarValueRead<ScalarType> {
//   readonly name: string,
//   readonly read: (value: SqliteValue, textualError: TextualError) => ScalarType | FailureCode;
// }

interface ScalarRead<Type> {
  (value: SqliteValue, textualError: TextualError): Type | FailureCode;
}

interface ScalarWrite<Type, > {
  (
    value: Type, 
    writer: ScalarTypeWriter, 
    textualError: TextualError,
  ): TriedCode;
}

export interface Scalar<Type> {
  readonly name: string,
  readonly read: ScalarRead<Type>,
  readonly write: ScalarWrite<Type>,
}

export interface NullDescriptor extends Scalar<null> {

}

export const NullDescriptor = {
  create: (): NullDescriptor => {
    return {
      name: "Null",

      write: (_value, writer) => {
        ScalarTypeWriter.writeNull(writer);
        return SUCCESS;
      },

      read: (sqliteValue, textualError) => {
        if (sqliteValue === null) {
          return null;
        }

        TextualError.changeContext(textualError, "Reading 'null'");
        TextualError.addMessage(textualError, "SQLite value is not 'null'");
        TextualError.addUnknownAttachment(textualError, "SQLite value", sqliteValue);
        return FAILURE;
      },
    };
  },
};

export interface StringDescriptor extends Scalar<string> {

}

export const StringDescriptor = {
  create: (): StringDescriptor => {
    return {
      name: "string",

      write: (value, writer) => {
        ScalarTypeWriter.writeString(writer, value);
        return SUCCESS;
      },

      read: (sqliteValue, textualError) => {
        if (typeof sqliteValue === "string") {
          return sqliteValue;
        }

        TextualError.changeContext(textualError, "Read a string");
        TextualError.addMessage(textualError, "SQLite value is not a string");
        TextualError.addUnknownAttachment(textualError, "SQLite value", sqliteValue);
        return FAILURE;
      },
    };
  },
};

export interface IntegerDescriptor extends Scalar<number> {

}

export const IntegerDescriptor = {
  create: (): IntegerDescriptor => {
    return {
      name: "Integer",

      write: (value, writer, textualError) => {
        const status = ScalarTypeWriter.fallableWriteInteger(writer, value);
        if (status === SUCCESS) {
          return SUCCESS;
        }

        TextualError.changeContext(textualError, "Writing an integer");
        TextualError.addMessage(textualError, "Number is not an integer");
        TextualError.addNumberAttachment(textualError, "Number", value);
        return FAILURE;
      },

      read: (sqliteValue, textualError) => {
        if (Number.isInteger(sqliteValue)) {
          return sqliteValue as number;
        }

        TextualError.changeContext(textualError, "Reading an integer");
        TextualError.addMessage(textualError, "SQLite value is not an integer");
        TextualError.addUnknownAttachment(textualError, "SQLite value", sqliteValue);
        return FAILURE;
      },
    };
  },
};

export interface RealDescriptor extends Scalar<number> {

}

export const RealDescriptor = {
  create: (): RealDescriptor => {
    return {
      name: "Real",

      write: (value, writer, textualError) => {
        const status = ScalarTypeWriter.fallableWriteReal(writer, value);
        if (status === SUCCESS) {
          return SUCCESS;
        }

        TextualError.changeContext(textualError, "Writing a real");
        TextualError.addMessage(textualError, "Number is not a real");
        TextualError.addNumberAttachment(textualError, "Number", value);
        return FAILURE;
      },

      read: (sqliteValue, textualError) => {
        if (Number.isFinite(sqliteValue)) {
          return sqliteValue as number;
        }

        TextualError.changeContext(textualError, "Reading a real");
        TextualError.addMessage(textualError, "SQLite value is not a real");
        TextualError.addUnknownAttachment(textualError, "SQLite value", sqliteValue);
        return FAILURE;
      },
    };
  },
};

export interface BooleanDescriptor extends Scalar<boolean> {

}

export const BooleanDescriptor = {
  create: (): BooleanDescriptor => {
    return {
      name: "boolean",

      write: (value, writer) => {
        ScalarTypeWriter.writeBoolean(writer, value);
        return SUCCESS;
      },

      read: (sqliteValue, textualError) => {
        if (sqliteValue === 0) {
          return false;
        }
        if (sqliteValue === 1) {
          return true;
        }

        TextualError.changeContext(textualError, "Reading a boolean, which is represented in SQLite as an integer of '0' or '1'");
        TextualError.addMessage(textualError, "SQLite value is neither '0' nor '1'");
        TextualError.addUnknownAttachment(textualError, "SQLite value", sqliteValue);
        return FAILURE;
      },
    };
  },
};


export interface NullableDescriptor<Type> extends Scalar<Nullable<Type>> {
  readonly descriptor: Scalar<Type>,
}

export const NullableDescriptor = {
  create: <T>(
    descriptor: Scalar<T>,
  ): NullableDescriptor<T> => {
    const name = `Nullable<${descriptor.name}>`;

    return {
      name,
      descriptor,

      write: (value, writer, textualError) => {
        if (value === null) {
          ScalarTypeWriter.writeNull(writer);
          return SUCCESS;
        }

        const status = descriptor.write(value, writer, textualError)
        if (status === SUCCESS) {
          return SUCCESS;
        }

        TextualError.changeContext(textualError, `Writing a ${name}`)
        TextualError.addMessage(textualError, "Value is not 'null' and an error occured while writing the non-null variant");
        return FAILURE;
      },

      read: (sqliteValue, textualError) => {
        if (sqliteValue === null) {
          return null;
        }

        const nonNullVariant = descriptor.read(sqliteValue, textualError);
        if (nonNullVariant !== FAILURE) {
          return nonNullVariant;
        }

        TextualError.changeContext(textualError, `Reading a ${name}`)
        TextualError.addMessage(textualError, "Value is not 'null' and an error occured while reading the non-null variant");
        return FAILURE;
      },
    };
  },
};

export interface CustomScalarDescriptor<Value, Inner> extends Scalar<Value> {
  readonly innerDescriptor: Scalar<Inner>,
}

export const CustomScalarDescriptor = {
  create: <Value, Inner extends SqliteValue>({
    name,
    fromInner,
    innerDescriptor,
    intoInner,
  }: {
    name: string,
    innerDescriptor: Scalar<Inner>,
    fromInner: (sqliteValue: Inner, textualError: TextualError) => Value | FailureCode,
    intoInner: (value: Value) => Inner,
  }): CustomScalarDescriptor<Value, Inner> => {
    const fullName = `${name}(${innerDescriptor.name})`;

    return {
      name: fullName,

      innerDescriptor,

      read(sqliteValue, textualError) {
        let it;
        
        it = innerDescriptor.read(sqliteValue, textualError);
        if (it === FAILURE) {
          TextualError.changeContext(textualError, `Reading the inner value of ${fullName}`);
          return FAILURE;
        }
        
        it = fromInner(it, textualError);
        if (it === FAILURE) {
          TextualError.changeContext(textualError, `Constructing a value of type ${fullName}`);
          TextualError.changeContext(textualError, `Reading a value of type ${fullName}`);
          return FAILURE;
        }

        return it;
      },

      write(value, writer, textualError) {
        const status = innerDescriptor.write(intoInner(value), writer, textualError);
        if (status === SUCCESS) {
          return SUCCESS;
        }

        TextualError.changeContext(textualError, `Writing a value of type ${fullName}`);
        return FAILURE;
      },
    };
  },
};

const COLUMN_BRAND = Symbol();

export type Column = Nominal<typeof COLUMN_BRAND, string>;

export const Column = {
  // create: (string: string, textualError: TextualError): Column | FailureCode => {
  //   if (string.length === 0) {
  //     TextualError.changeContext(textualError, "Creating a 'ColumnName' from string");
  //     TextualError.addMessage(textualError, "String is empty");
  //     return FAILURE;
  //   }

  //   if (string.length > 100) {
  //     TextualError.changeContext(textualError, "Creating a 'ColumnName' from string");
  //     TextualError.addMessage(textualError, "String is too long");
  //     TextualError.addStringAttachment(textualError, "String", string);
  //     return FAILURE;
  //   }

  //   if (string.startsWith(/\d/ug, ))
  // },

  createOrThrow: (string: string): Column => {
    // TODO: Throw if the column name is invalid.
    return Nominal.create(COLUMN_BRAND, string);
  },

  toString: Nominal.get,
};

interface CompoundValueWriteDestination<It> {
  writeNull: (it: It, column: Column, textualError: TextualError) => void;
  fallableWriteInteger: (it: It, column: Column, integer: number, textualError: TextualError) => TriedCode;
  fallableWriteReal: (it: It, column: Column, real: number, textualError: TextualError) => TriedCode;
  writeString: (it: It, column: Column, string: string, textualError: TextualError) => void;
  writeBoolean: (it: It, column: Column, boolean: boolean, textualError: TextualError) => void;
  writeScalarValue: <Type>(it: It, column: Column, value: Type, descriptor: Scalar<Type>, textualError: TextualError) => TriedCode;
  writeCompoundValue: <Type, Columns>(it: It, columns: Columns, value: Type, descriptor: Compound<Type, Columns>, textualError: TextualError) => TriedCode;
}

interface CompoundValueReadSource<It> {
  readonly readAny: (it: It, column: Column, textualError: TextualError) => SqliteValue | FailureCode,
  readonly readNull: (it: It, column: Column, textualError: TextualError) => null | FailureCode;
  readonly readInteger: (it: It, column: Column, textualError: TextualError) => number | FailureCode;
  readonly readReal: (it: It, column: Column, textualError: TextualError) => number | FailureCode;
  readonly readString: (it: It, column: Column, textualError: TextualError) => string | FailureCode;
  readonly readBoolean: (it: It, column: Column, textualError: TextualError) => boolean | FailureCode;
  readonly readScalar: <Type>(it: It, column: Column, descriptor: Scalar<Type>, textualError: TextualError) => Type | FailureCode;
  readonly readCompound: <Type, Columns>(it: It, columns: Columns, descriptor: Compound<Type, Columns>, textualError: TextualError) => Type | FailureCode; 
}

type CompoundRead<Type, Columns> = (
  <Source>(
    source: Source, 
    sourceImpl: CompoundValueReadSource<Source>,
    columns: Columns,
    textualError: TextualError,
  ) => Type | FailureCode
);

type CompoundWrite<Type, Columns> = (
  <Destination>(
    destination: Destination,
    destinationImpl: CompoundValueWriteDestination<Destination>,
    columns: Columns,
    value: Type,
    textualError: TextualError,
  ) => TriedCode
);

export interface Compound<Type, Columns> {
  readonly name: string,
  readonly read: CompoundRead<Type, Columns>,
  readonly write: CompoundWrite<Type, Columns> ,
}

const enum PropertyType {
  Scalar,
  Compound,
}

export interface ObjectScalarPropertyDescriptor<Object, ObjectColumns, PropertyValue> {
  readonly type: PropertyType.Scalar,
  readonly name: string,
  readonly getter: (object: Object) => PropertyValue,
  readonly column: (columns: ObjectColumns) => Column,
  readonly descriptor: Scalar<PropertyValue>,
}

export const ObjectScalarPropertyDescriptor = {
  create: <Object, ObjectColumns, PropertyValue>({
    name,
    column,
    getter,
    descriptor,
  }: {
    readonly name: string,
    readonly getter: (object: Object) => PropertyValue,
    readonly column: (columns: ObjectColumns) => Column,
    readonly descriptor: Scalar<PropertyValue>,
  }): ObjectScalarPropertyDescriptor<Object, ObjectColumns, PropertyValue> => {
    return {
      type: PropertyType.Scalar,
      name,
      getter,
      column,
      descriptor,
    };
  },
};

export interface ObjectCompoundPropertyDescriptor<Object, ObjectColumns, PropertyValue, PropertyValueColumns> {
  readonly type: PropertyType.Compound,
  readonly name: string,
  readonly getter: (object: Object) => PropertyValue,
  readonly columns: (columns: ObjectColumns) => PropertyValueColumns,
  readonly descriptor: Compound<PropertyValue, PropertyValueColumns>,
}

export const ObjectCompoundPropertyDescriptor = {
  create: <Object, ObjectColumns, PropertyValue, PropertyValueColumns>({
    name,
    getter,
    columns,
    descriptor,
  }: {
    readonly name: string,
    readonly getter: (object: Object) => PropertyValue,
    readonly columns: (columns: ObjectColumns) => PropertyValueColumns,
    readonly descriptor: Compound<PropertyValue, PropertyValueColumns>,
  }): ObjectCompoundPropertyDescriptor<Object, ObjectColumns, PropertyValue, PropertyValueColumns> => {
    return {
      type: PropertyType.Compound,
      name,
      getter,
      columns,
      descriptor,
    };
  },
};

export type ObjectPropertyDescriptor<Object, ObjectColumns, PropertyValue> = (
  | ObjectScalarPropertyDescriptor<Object, ObjectColumns, PropertyValue>
  | ObjectCompoundPropertyDescriptor<Object, ObjectColumns, PropertyValue, any>
);

export interface ObjectDescriptor<Object, Properties extends unknown[], Columns> extends Compound<Object, Columns> {
  readonly name: string,
  readonly properties: {
    [Key in keyof Properties]: ObjectPropertyDescriptor<Object, Columns, Properties[Key]>
  },
}

export const ObjectDescriptor = {
  create: <Object, Properties extends unknown[], Columns>({
    name,
    properties,
    construct,
  }: {
    readonly name: string,
    readonly properties: {
      [Key in keyof Properties]: ObjectPropertyDescriptor<Object, Columns, Properties[Key]>
    },
    construct: (...properties: [...Properties, TextualError]) => Object | FailureCode,
  }): ObjectDescriptor<Object, Properties, Columns> => {
    return {
      name,

      properties,

      read: (source, sourceImpl, columns, textualError) => {
        const values: unknown[] = [];

        for (const property of properties) {
          switch (property.type) {
            case PropertyType.Scalar: {
              const column = property.column(columns);
              let value;
              
              value = sourceImpl.readAny(source, column, textualError);
              if (value === FAILURE) {
                return FAILURE;
              }

              value = property.descriptor.read(value, textualError);
              if (value === FAILURE) {
                return FAILURE;
              }

              values.push(value);
              continue;
            }
            case PropertyType.Compound: {
              const propertyValueColumns = property.columns(columns);
              const value = property.descriptor.read(source, sourceImpl, propertyValueColumns, textualError);
              if (value === FAILURE) {
                return FAILURE;
              }

              values.push(value);
              continue;
            }
          }
        }

        const it = construct(...values as Properties, textualError);
        if (it !== FAILURE) {
          return FAILURE;
        }

        return it;
      },

      write: () => {
        
      }
    };
  },
};

export interface EnumVariantDescriptor<NumericValue extends number, TextualValue extends string> extends Scalar<NumericValue> {
  readonly numericValue: NumericValue,
  readonly textualValue: TextualValue,
}

export const EnumVariantDescriptor = {
  create: <
    NumericValue extends number, 
    TextualValue extends string,
  >(
    initializre: EnumVariantDescriptor<NumericValue, TextualValue>
  ): EnumVariantDescriptor<NumericValue, TextualValue> => {
    return initializre;
  },
};

export interface EnumDescriptor<Variants extends [number, string][]> {
  readonly name: string,
  readonly variantsByNumericValue: {
    [Key in keyof Variants]: Variants[Key]
  },
  readonly variantsByTextualValue: {
    [Key in Variants[number][1]]: Extract<Variants[number], [number, Key]>
  },
}

export const EnumDescriptor = {
  create: <Variants extends [number, string][]>({
    name,
    variants,
  }: {
    readonly name: string,
    readonly variants: {
      [Key in keyof Variants]: EnumVariantDescriptor<Variants[Key][0], Variants[Key][1]>
    }
  }): EnumDescriptor<Variants> => {
    return {
      name,
      // variantsByNumericValue: 
    };
  },
};

const enum AlgebricDataTypeVariantType {
  Unit,
  Data,
}

export interface AlgebricDataTypeUnitVariantDescriptor {
  readonly type: AlgebricDataTypeVariantType.Unit,
  readonly name: string,
}

export const AlgebricDataTypeUnitVariantDescriptor = {
  create: ({
    name,
  }: {
    readonly name: string,
  }): AlgebricDataTypeUnitVariantDescriptor => {
    return {
      type: AlgebricDataTypeVariantType.Unit,
      name,
    };
  },
};

export interface AlgebricDataTypeDataVariantDescriptor<Data, Columns> {
  readonly type: AlgebricDataTypeVariantType.Data,
  readonly name: string,
  readonly dataDescriptor: Compound<Data, Columns>,
}

export const AlgebricDataTypeDataVariantDescriptor = {
  create: <Data, Columns>({
    name,
    dataDescriptor,
  }: {
    readonly name: string,
    readonly dataDescriptor: Compound<Data, Columns>
  }): AlgebricDataTypeDataVariantDescriptor<Data, Columns> => {
    return {
      type: AlgebricDataTypeVariantType.Data,
      name,
      dataDescriptor,
    };
  },
};

export interface AlgebricDataTypeDescriptor<Type, Tag, Columns> extends Compound<Type, Columns> {
  readonly name: string,
  readonly tagDescriptor: Scalar<Tag>,
  readonly getTagColumn: (columns: Column) => Column,
  
  readonly matchTag: <R1, R2>(
    tag: Tag,
    onUnit: (descriptor: AlgebricDataTypeUnitVariantDescriptor) => R1,
    onData: <Data, DataColumns>(descriptor: AlgebricDataTypeDataVariantDescriptor<Data, DataColumns>, columns: Columns) => R2,
  ) => R1 | R2,
  
  readonly matchValue: <R1, R2>(
    value: Type,
    onUnit: (descriptor: AlgebricDataTypeUnitVariantDescriptor) => void,
    onData: <Data, DataColumns>(data: Data, descriptor: AlgebricDataTypeDataVariantDescriptor<Data, DataColumns>, columns: Columns) => void,
  ) => R1 | R2,
}

export const AlgebricDataTypeDescriptor = {
  create: <Type, Tag, Columns>(
    initializre: AlgebricDataTypeDescriptor<Type, Tag, Columns>,
  ): AlgebricDataTypeDescriptor<Type, Tag, Columns> => {
    return initializre;
  },
};

export interface OptionSchema<InnerValueSchema> {
  tag: Column,
  innerValueSchema: InnerValueSchema,
}

export interface OptionSerDe<InnerValue, InnerValueSchema> extends 
  CompoundValueReadWrite<Nullable<InnerValue>, OptionSchema<InnerValueSchema>>
{
  readonly innerValueSerDe: CompoundValueReadWrite<InnerValue, InnerValueSchema>,   
}

// export const OptionTagSerDe = 
export const OptionSerDe = {
  create: <InnerValue, InnerValueSchema>(
    innerValueSerDe: CompoundValueReadWrite<InnerValue, InnerValueSchema>,
  ): OptionSerDe<InnerValue, InnerValueSchema> => {
    return {
      innerValueSerDe,

      write(value, writer, write, schema, textualError) {
        let it;

        if (value === null) {
          it = write.fallableWriteInteger(writer, schema.tag, 0, textualError);
          if (it === SUCCESS) {
            return SUCCESS;
          } else {
            TextualError.changeContext(textualError, "Writing an integer '0' representing the tag of the 'None' variant");
            TextualError.changeContext(textualError, "Writing an 'Option'");
            return FAILURE;
          }
        }

        it = write.fallableWriteInteger(writer, schema.tag, 1, textualError);
        if (it === FAILURE) {
          TextualError.changeContext(textualError, "Writing an integer '1' representing the tag of the 'Some' variant");
          TextualError.changeContext(textualError, "Writing an 'Option'");
          return FAILURE;
        }

        it = innerValueSerDe.write(value, writer, write, schema.innerValueSchema, textualError);
        if (it === FAILURE) {
          TextualError.changeContext(textualError, "Writing the inner value of the 'Some' variant");
          TextualError.changeContext(textualError, "Writing an 'Option'");
          return FAILURE;
        }

        return SUCCESS;
      },

      read: (reader, read, schema, textualError) => {
        const tag = read.readInteger(reader, schema.tag, textualError);
        if (tag === FAILURE) {
          TextualError.changeContext(textualError, "Reading the 'Option' tag");
          TextualError.changeContext(textualError, "Reading an 'Option'")
          return FAILURE;
        }
        
        if (tag === 0) {
          return null;
        }

        if (tag !== 1) {
          TextualError.changeContext(textualError, "Reading the 'Option' tag");
          TextualError.addMessage(textualError, "Expected integers '0' or '1', but something else");
          TextualError.addNumberAttachment(textualError, "SQLite value", tag);
          TextualError.changeContext(textualError, "Reading an 'Option'")
          return FAILURE;
        }

        const innerValue = innerValueSerDe.read(reader, read, schema.innerValueSchema, textualError);
        if (innerValue === SUCCESS) {
          return innerValue;
        }

        TextualError.changeContext(textualError, "Reading the 'Option' tag");
        TextualError.addMessage(textualError, "Expected integers '0' or '1', but something else");
        TextualError.addNumberAttachment(textualError, "SQLite value", tag);
        TextualError.changeContext(textualError, "Reading an 'Option'")
        return FAILURE;
      },
    }
  },
};