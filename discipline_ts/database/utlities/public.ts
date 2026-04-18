// import { Scalar } from "./scalar.ts";

// export interface NullDescriptor extends Scalar<null> {}

// export const NullDescriptor = {
//   create: (): NullDescriptor => {
//     return {
//       name: "Null",

//       read(source, sourceImpl) {
//         return sourceImpl.readNullOrThrow(source);
//       },

//       write: (_, destination, destinationImpl) => {
//         destinationImpl.writeNull(destination);
//       },
//     };
//   },
// };

// export interface StringDescriptor extends Scalar<string> {}

// export const StringDescriptor = {
//   create: (): StringDescriptor => {
//     return {
//       name: "string",

//       read: (source, sourceImpl) => {
//         return sourceImpl.readStringOrThrow(source);
//       },

//       write: (value, destination, destinationImpl) => {
//         destinationImpl.writeString(destination, value);
//       },
//     };
//   },
// };

// export interface IntegerDescriptor extends Scalar<Integer> {}

// export const IntegerDescriptor = {
//   create: (): IntegerDescriptor => {
//     return {
//       name: "Integer",
      
//       read: (source, sourceImpl) => {
//         return sourceImpl.readIntegerOrThrow(source)
//       },

//       write: (value, destination, destinationImpl) => {
//         destinationImpl.writeInteger(destination, value);
//       },
//     };
//   },
// };

// export interface RealDescriptor extends Scalar<Float> {}

// export const RealDescriptor = {
//   create: (): RealDescriptor => {
//     return {
//       name: "Real",

//       read: (source, sourceImpl) => {
//         return sourceImpl.readRealOrThrow(source);
//       },

//       write: (value, destination, destinationImpl) => {
//         destinationImpl.writeReal(destination, value);
//       },
//     };
//   },
// };

// export interface BooleanDescriptor extends Scalar<boolean> {}

// export const BooleanDescriptor = {
//   create: (): BooleanDescriptor => {
//     return {
//       name: "boolean",

//       read: (source, sourceImpl) => {
//         return sourceImpl.readBooleanOrThrow(source);
//       },

//       write: (value, destination, destinationImpl) => {
//         destinationImpl.writeBoolean(destination, value);
//       },
//     };
//   },
// };


// export interface NullableDescriptor<Value> extends Scalar<Nullable<Value>> {
//   readonly descriptor: Scalar<Value>,
// }

// export const NullableDescriptor = {
//   create: <T>(
//     descriptor: Scalar<T>,
//   ): NullableDescriptor<T> => {
//     const name = `Nullable<${descriptor.name}>`;

//     return {
//       name,

//       descriptor,

//       write: (value, destination, destinationImpl) => {
//         if (value === null) {
//           destinationImpl.writeNull(destination);
//           return;
//         }

//         descriptor.write(value, destination, destinationImpl);
//       },

//       read: (source, sourceImpl) => {
//         if (sourceImpl.isNull(source)) {
//           return null;
//         }

//         const value = descriptor.readOrThrow(source, sourceImpl);
//         if (value !== FAILURE) {
//           return value;
//         }

//         TextualError.changeContext(textualError, `Reading a ${name}`)
//         TextualError.addMessage(textualError, "Value is not 'null' and an error occured while reading the non-null variant");
//         return FAILURE;
//       },
//     };
//   },
// };

// export interface CustomScalarDescriptor<Value, Inner> extends Scalar<Value> {
//   readonly innerDescriptor: Scalar<Inner>,
// }

// export const CustomScalarDescriptor = {
//   create: <Value, Inner extends SqliteValue>({
//     name,
//     fromInner,
//     innerDescriptor,
//     intoInner,
//   }: {
//     name: string,
//     innerDescriptor: Scalar<Inner>,
//     fromInner: (sqliteValue: Inner, textualError: TextualError) => Value | FailureCode,
//     intoInner: (value: Value) => Inner,
//   }): CustomScalarDescriptor<Value, Inner> => {
//     const fullName = `${name}(${innerDescriptor.name})`;

//     return {
//       name: fullName,

//       innerDescriptor,

//       read(source, sourceImpl) {
//         let it;
        
//         it = innerDescriptor.read(source, sourceImpl);
//         if (it === FAILURE) {
//           TextualError.changeContext(textualError, `Reading the inner value of ${fullName}`);
//           return FAILURE;
//         }
        
//         it = fromInner(it, textualError);
//         if (it === FAILURE) {
//           TextualError.changeContext(textualError, `Constructing a value of type ${fullName}`);
//           TextualError.changeContext(textualError, `Reading a value of type ${fullName}`);
//           return FAILURE;
//         }

//         return it;
//       },

//       write(value, destination, destinationImpl) {
//         return innerDescriptor.write(intoInner(value), destination, destinationImpl);
//       },
//     };
//   },
// };


// const enum PropertyType {
//   Scalar,
//   Compound,
// }

// export interface ObjectScalarPropertyDescriptor<Object, ObjectColumns, PropertyValue> {
//   readonly type: PropertyType.Scalar,
//   readonly name: string,
//   readonly getter: (object: Object) => PropertyValue,
//   readonly column: (columns: ObjectColumns) => Column,
//   readonly descriptor: Scalar<PropertyValue>,
// }

// export const ObjectScalarPropertyDescriptor = {
//   create: <Object, ObjectColumns, PropertyValue>({
//     name,
//     column,
//     getter,
//     descriptor,
//   }: {
//     readonly name: string,
//     readonly getter: (object: Object) => PropertyValue,
//     readonly column: (columns: ObjectColumns) => Column,
//     readonly descriptor: Scalar<PropertyValue>,
//   }): ObjectScalarPropertyDescriptor<Object, ObjectColumns, PropertyValue> => {
//     return {
//       type: PropertyType.Scalar,
//       name,
//       getter,
//       column,
//       descriptor,
//     };
//   },
// };

// export interface ObjectCompoundPropertyDescriptor<Object, ObjectColumns, PropertyValue, PropertyValueColumns> {
//   readonly type: PropertyType.Compound,
//   readonly name: string,
//   readonly getter: (object: Object) => PropertyValue,
//   readonly columns: (columns: ObjectColumns) => PropertyValueColumns,
//   readonly descriptor: Compound<PropertyValue, PropertyValueColumns>,
// }

// export const ObjectCompoundPropertyDescriptor = {
//   create: <Object, ObjectColumns, PropertyValue, PropertyValueColumns>({
//     name,
//     getter,
//     columns,
//     descriptor,
//   }: {
//     readonly name: string,
//     readonly getter: (object: Object) => PropertyValue,
//     readonly columns: (columns: ObjectColumns) => PropertyValueColumns,
//     readonly descriptor: Compound<PropertyValue, PropertyValueColumns>,
//   }): ObjectCompoundPropertyDescriptor<Object, ObjectColumns, PropertyValue, PropertyValueColumns> => {
//     return {
//       type: PropertyType.Compound,
//       name,
//       getter,
//       columns,
//       descriptor,
//     };
//   },
// };

// export type ObjectPropertyDescriptor<Object, ObjectColumns, PropertyValue> = (
//   | ObjectScalarPropertyDescriptor<Object, ObjectColumns, PropertyValue>
//   | ObjectCompoundPropertyDescriptor<Object, ObjectColumns, PropertyValue, any>
// );

// export interface ObjectDescriptor<Object, Properties extends unknown[], Columns> extends Compound<Object, Columns> {
//   readonly name: string,
//   readonly properties: {
//     [Key in keyof Properties]: ObjectPropertyDescriptor<Object, Columns, Properties[Key]>
//   },
// }

// export const ObjectDescriptor = {
//   create: <Object, Properties extends unknown[], Columns>({
//     name,
//     properties,
//     construct,
//   }: {
//     readonly name: string,
//     readonly properties: {
//       [Key in keyof Properties]: ObjectPropertyDescriptor<Object, Columns, Properties[Key]>
//     },
//     construct: (...properties: [...Properties, TextualError]) => Object | FailureCode,
//   }): ObjectDescriptor<Object, Properties, Columns> => {
//     return {
//       name,

//       properties,

//       read: (source, sourceImpl, columns, textualError) => {
//         const values: unknown[] = [];

//         for (const property of properties) {
//           switch (property.type) {
//             case PropertyType.Scalar: {
//               const column = property.column(columns);
//               let value;
              
//               value = sourceImpl.readAny(source, column, textualError);
//               if (value === FAILURE) {
//                 return FAILURE;
//               }

//               value = property.descriptor.read(value, textualError);
//               if (value === FAILURE) {
//                 return FAILURE;
//               }

//               values.push(value);
//               continue;
//             }
//             case PropertyType.Compound: {
//               const propertyValueColumns = property.columns(columns);
//               const value = property.descriptor.read(source, sourceImpl, propertyValueColumns, textualError);
//               if (value === FAILURE) {
//                 return FAILURE;
//               }

//               values.push(value);
//               continue;
//             }
//           }
//         }

//         const it = construct(...values as Properties, textualError);
//         if (it !== FAILURE) {
//           return FAILURE;
//         }

//         return it;
//       },

//       write: () => {
        
//       }
//     };
//   },
// };

// export interface EnumVariantDescriptor<NumericValue extends number, TextualValue extends string> extends Scalar<NumericValue> {
//   readonly numericValue: NumericValue,
//   readonly textualValue: TextualValue,
// }

// export const EnumVariantDescriptor = {
//   create: <
//     NumericValue extends number, 
//     TextualValue extends string,
//   >(
//     initializre: EnumVariantDescriptor<NumericValue, TextualValue>
//   ): EnumVariantDescriptor<NumericValue, TextualValue> => {
//     return initializre;
//   },
// };

// export interface EnumDescriptor<Variants extends [number, string][]> {
//   readonly name: string,
//   readonly variantsByNumericValue: {
//     [Key in keyof Variants]: Variants[Key]
//   },
//   readonly variantsByTextualValue: {
//     [Key in Variants[number][1]]: Extract<Variants[number], [number, Key]>
//   },
// }

// export const EnumDescriptor = {
//   create: <Variants extends [number, string][]>({
//     name,
//     variants,
//   }: {
//     readonly name: string,
//     readonly variants: {
//       [Key in keyof Variants]: EnumVariantDescriptor<Variants[Key][0], Variants[Key][1]>
//     }
//   }): EnumDescriptor<Variants> => {
//     return {
//       name,
//       // variantsByNumericValue: 
//     };
//   },
// };

// const enum AlgebricDataTypeVariantType {
//   Unit,
//   Data,
// }

// export interface AlgebricDataTypeUnitVariantDescriptor {
//   readonly type: AlgebricDataTypeVariantType.Unit,
//   readonly name: string,
// }

// export const AlgebricDataTypeUnitVariantDescriptor = {
//   create: ({
//     name,
//   }: {
//     readonly name: string,
//   }): AlgebricDataTypeUnitVariantDescriptor => {
//     return {
//       type: AlgebricDataTypeVariantType.Unit,
//       name,
//     };
//   },
// };

// export interface AlgebricDataTypeDataVariantDescriptor<Data, Columns> {
//   readonly type: AlgebricDataTypeVariantType.Data,
//   readonly name: string,
//   readonly dataDescriptor: Compound<Data, Columns>,
// }

// export const AlgebricDataTypeDataVariantDescriptor = {
//   create: <Data, Columns>({
//     name,
//     dataDescriptor,
//   }: {
//     readonly name: string,
//     readonly dataDescriptor: Compound<Data, Columns>
//   }): AlgebricDataTypeDataVariantDescriptor<Data, Columns> => {
//     return {
//       type: AlgebricDataTypeVariantType.Data,
//       name,
//       dataDescriptor,
//     };
//   },
// };

// export interface AlgebricDataTypeDescriptor<Type, Tag, Columns> extends Compound<Type, Columns> {
//   readonly name: string,
//   readonly tagDescriptor: Scalar<Tag>,
//   readonly getTagColumn: (columns: Column) => Column,
  
//   readonly matchTag: <R1, R2>(
//     tag: Tag,
//     onUnit: (descriptor: AlgebricDataTypeUnitVariantDescriptor) => R1,
//     onData: <Data, DataColumns>(descriptor: AlgebricDataTypeDataVariantDescriptor<Data, DataColumns>, columns: Columns) => R2,
//   ) => R1 | R2,
  
//   readonly matchValue: <R1, R2>(
//     value: Type,
//     onUnit: (descriptor: AlgebricDataTypeUnitVariantDescriptor) => void,
//     onData: <Data, DataColumns>(data: Data, descriptor: AlgebricDataTypeDataVariantDescriptor<Data, DataColumns>, columns: Columns) => void,
//   ) => R1 | R2,
// }

// export const AlgebricDataTypeDescriptor = {
//   create: <Type, Tag, Columns>(
//     initializre: AlgebricDataTypeDescriptor<Type, Tag, Columns>,
//   ): AlgebricDataTypeDescriptor<Type, Tag, Columns> => {
//     return initializre;
//   },
// };

// export interface OptionSchema<InnerValueSchema> {
//   tag: Column,
//   innerValueSchema: InnerValueSchema,
// }

// export interface OptionSerDe<InnerValue, InnerValueSchema> extends 
//   CompoundValueReadWrite<Nullable<InnerValue>, OptionSchema<InnerValueSchema>>
// {
//   readonly innerValueSerDe: CompoundValueReadWrite<InnerValue, InnerValueSchema>,   
// }

// // export const OptionTagSerDe = 
// export const OptionSerDe = {
//   create: <InnerValue, InnerValueSchema>(
//     innerValueSerDe: CompoundValueReadWrite<InnerValue, InnerValueSchema>,
//   ): OptionSerDe<InnerValue, InnerValueSchema> => {
//     return {
//       innerValueSerDe,

//       write(value, destination, destinationImpl, write, schema, textualError) {
//         let it;

//         if (value === null) {
//           it = write.fallableWriteInteger(writer, schema.tag, 0, textualError);
//           if (it === SUCCESS) {
//             return SUCCESS;
//           } else {
//             TextualError.changeContext(textualError, "Writing an integer '0' representing the tag of the 'None' variant");
//             TextualError.changeContext(textualError, "Writing an 'Option'");
//             return FAILURE;
//           }
//         }

//         it = write.fallableWriteInteger(writer, schema.tag, 1, textualError);
//         if (it === FAILURE) {
//           TextualError.changeContext(textualError, "Writing an integer '1' representing the tag of the 'Some' variant");
//           TextualError.changeContext(textualError, "Writing an 'Option'");
//           return FAILURE;
//         }

//         it = innerValueSerDe.write(value, destination, destinationImpl, write, schema.innerValueSchema, textualError);
//         if (it === FAILURE) {
//           TextualError.changeContext(textualError, "Writing the inner value of the 'Some' variant");
//           TextualError.changeContext(textualError, "Writing an 'Option'");
//           return FAILURE;
//         }

//         return SUCCESS;
//       },

//       read: (reader, read, schema, textualError) => {
//         const tag = read.readInteger(reader, schema.tag, textualError);
//         if (tag === FAILURE) {
//           TextualError.changeContext(textualError, "Reading the 'Option' tag");
//           TextualError.changeContext(textualError, "Reading an 'Option'")
//           return FAILURE;
//         }
        
//         if (tag === 0) {
//           return null;
//         }

//         if (tag !== 1) {
//           TextualError.changeContext(textualError, "Reading the 'Option' tag");
//           TextualError.addMessage(textualError, "Expected integers '0' or '1', but something else");
//           TextualError.addNumberAttachment(textualError, "SQLite value", tag);
//           TextualError.changeContext(textualError, "Reading an 'Option'")
//           return FAILURE;
//         }

//         const innerValue = innerValueSerDe.read(reader, read, schema.innerValueSchema, textualError);
//         if (innerValue === SUCCESS) {
//           return innerValue;
//         }

//         TextualError.changeContext(textualError, "Reading the 'Option' tag");
//         TextualError.addMessage(textualError, "Expected integers '0' or '1', but something else");
//         TextualError.addNumberAttachment(textualError, "SQLite value", tag);
//         TextualError.changeContext(textualError, "Reading an 'Option'")
//         return FAILURE;
//       },
//     }
//   },
// };
