const SUCCESS_CODE = Symbol();
const FAILURE_CODE = Symbol();

type SuccessCode = typeof SUCCESS_CODE;
type FailureCode = typeof FAILURE_CODE;
type TriedCode = SuccessCode | FailureCode;

interface Writer {
  data: string[],
}

const Writer = {
  write: (writer: Writer, slice: string) => {
    writer.data.push(slice);
  },
};

const writeNull = (writer: Writer) => {
  Writer.write(writer, "null");
};
const writeUndefined = (writer: Writer) => {
  Writer.write(writer, "undefined");
};
const writeNumber = (writer: Writer, number: number) => {
  Writer.write(writer, number.toString());
};
const writeBoolean = (writer: Writer, boolean: boolean) => {
  Writer.write(writer, boolean ? "true" : "false");
};
const writeString = (writer: Writer, string: string) => {
  string = string.replaceAll(/\"/ug, '\\"');
  string = string.replaceAll(/\n/ug, '\\n');
  string = `"${string}"`;
  Writer.write(writer, string);
};
const writeArray = <Item>(
  writer: Writer,
  array: Item[],
  itemPrinter: Printer<Item>,
) => {
  if (array.length === 0) {
    Writer.write(writer, "[]");
    return;
  }

  Writer.write(writer, "[");

  let index = 0;
  while (true) {
    if (index < array.length) {
      itemPrinter.write(writer, array[index]);
      Writer.write(writer, ", ");
      index += 1;
      continue;
    }
    
    Writer.write(writer, "]");
  }
};
const writeObject = <ObjectType>(
  writer: Writer,
  object: ObjectType,
  properties: ObjectProperty<ObjectType, any>[],
) => {
  if (properties.length === 0) {
    Writer.write(writer, "{}");
    return;
  }  

  Writer.write(writer, "{ ");

  // How many properties should we add ", " after?
  const lengthMinusOne = properties.length - 1;

  for (let index = 0; index < lengthMinusOne; index++) {
    const property = properties[index];

    Writer.write(writer, property.name);
    Writer.write(writer, ": ");
    property.printer.write(writer, property.getter(object));
    Writer.write(writer, ", ");
  }

  const property = properties[lengthMinusOne];
  Writer.write(writer, property.name);
  Writer.write(writer, ": ");
  property.printer.write(writer, property.getter(object));
  Writer.write(writer, " }");
};
const writeAdt = <AdtType>(
  writer: Writer,
  name: string,
  match: AdtMatch<AdtType>,
  value: AdtType,
) => {
  Writer.write(writer, name);
  Writer.write(writer, ".");

  match(
    value,
    (variant) => {
      Writer.write(writer, variant);
    },
    (variant, data, printer) => {
      Writer.write(writer, variant);
      Writer.write(writer, ".(");
      printer.write(writer, data);
      Writer.write(writer, ")");
    },
  )
};

interface Printer<Value> {
  readonly write: (writer: Writer, value: Value) => void;
}

export interface Null extends Printer<null> {

}

export const Null = {
  create: (): Null => {
    return {
      write: (writer, value) => {
        Writer.write(writer, "null");
        return SUCCESS_CODE;
      },
    };
  },
};

export interface Undefined extends Printer<Undefined> {

}

export const Undefined = {
  create: (): Undefined => {
    return {
      write: (writer, value) => {
        Writer.write(writer, "undefined");
        return SUCCESS_CODE;
      },
    };
  },
};


export interface Number extends Printer<number> {

}

export const Number = {
  create: (): Number => {
    return {
      write: (writer, value) => {
        Writer.write(writer, value.toString());
        return SUCCESS_CODE;
      },
    };
  },
};

export interface Boolean extends Printer<boolean> {

}

export const Boolean = {
  create: (): Boolean => {
    return {
      write: (writer, value) => {
        Writer.write(writer, value ? "true" : "false");
        return SUCCESS_CODE;
      },
    };
  },
};


export interface String extends Printer<string> {

}

export const String = {
  create: (): String => {
    return {
      write: (writer, value) => {
        value = value.replaceAll('"', '\\"');
        value = value.replaceAll('\n', '\\n"');
        Writer.write(writer, `"${value}"`);
        return SUCCESS_CODE;
      },
    };
  },
};

export interface Array<Item> extends Printer<Item[]> {
  itemPrinter: Printer<Item>,
}

export const Array = {
  create: <Item>(itemPrinter: Printer<Item>): Array<Item> => {
    return {
      itemPrinter,

      write: (writer, value) => {
        return writeArray(writer, value, itemPrinter);
      },
    };
  },
};

export interface ObjectProperty<ObjectType, PropertyValue> {
  readonly name: string,
  readonly getter: (object: ObjectType) => PropertyValue,
  readonly printer: Printer<PropertyValue>,
}

export const ObjectProperty = {
  create: <ObjectType, PropertyValue>(
    name: string,
    getter: (object: ObjectType) => PropertyValue,
    printer: Printer<PropertyValue>,
  ): ObjectProperty<ObjectType, PropertyValue> => {
    return {
      name,
      getter,
      printer,
    };
  },
};

export interface Object<ObjectType> extends Printer<ObjectType> {
  readonly properties: ObjectProperty<ObjectType, any>[]
}

export const Object = {
  create: <ObjectType>(
    properties: ObjectProperty<ObjectType, any>[],
  ): Object<ObjectType> => {
    return {
      properties,
      write: (writer, value) => {
        return writeObject(writer, value, properties);
      },
    }
  },
};

// const enum EnumVariantType {
//   Unit,
//   Data,
// }

// export interface EnumUnitVariant {
//   readonly type: EnumVariantType.Unit,
//   readonly name: string,
// }

// export const EnumUnitVariant = {
//   create: (name: string): EnumUnitVariant => {
//     return {
//       type: EnumVariantType.Unit,
//       name,
//     };
//   },
// };

// export interface EnumDataVariant<Data> {
//   readonly type: EnumVariantType.Data,
//   readonly name: string,
//   readonly printer: Printer<Data>,
// }

// export const EnumDataVariant = {
//   create: <Data>(name: string, printer: Printer<Data>): EnumDataVariant<Data> => {
//     return {
//       type: EnumVariantType.Data,
//       name,
//       printer,
//     };
//   },
// };

// export type AnyEnumVariant = EnumUnitVariant | EnumDataVariant<any>;

export interface AdtMatch<Type> {
  <R1, R2>(
    enumValue: Type,
    onUnit: (name: string) => R1,
    onData: <Data>(name: string, data: Data, printer: Printer<Data>) => R2,
  ): R1 | R2;
}

export interface Adt<Type> extends Printer<Type> {
  readonly name: string,
  readonly match: AdtMatch<Type>,
}

export const Adt = {
  create: <Type>(name: string, match: AdtMatch<Type>): Adt<Type> => {
    return {
      name,
      match,
      write: (writer, value) => {
        return writeAdt(writer, name, match, value);
      },
    };
  },
};

// export interface NumericEnum<EnumType, VariantNames> {

// }

// const serializeNull = () => {
//   return "null";
// };
// const serializeUndefined = () => {
//   return "undefined";
// };
// const serializeNumber = (number: number) => {
//   return number.toString();
// };
// const serializeString = (string: string) => {
//   return `${string.replaceAll(/"/ug, '\\"')}`;
// };
// const serializeBoolean = (boolean: boolean) => {
//   return boolean ? "true" : "false";
// };
// const serializeBigInt = (bigint: bigint) => {
//   return bigint.toString();
// };

// class Print<Value> {
//   write(
//     value: Value, 
//     writer: Writer,
//   ): Result {

//   }
// }

// class Configuration {
//   private constructor(
//     private level: number,
//     private readonly indentation: string,
//     private readonly indentationLength: number,
//   ) {}

//   getIndentation() {
//     return this.indentation.repeat(this.level);
//   }

//   goDown() {
//     this.level += 1;
//   }
  
//   goUp() {
//     this.level -= 1;
//   }
// }

// const enum WriteStatus {
//   WriteExpected,
//   WritePending,
//   WriteComplete,
//   Error,
// }

// const enum Collection {
//   Object,
//   Array,
//   Tuble,
//   Set,
//   Map,
// }

// type DestinationContext = {
//   readonly collection: Collection,
//   insertComma: boolean,
// };

// type Context2 = {
//   readonly type: "Any"
// } | {
//   readonly type: "Object"
// } | {
//   readonly type: "Array"
// };

// // complete / not complete
// // 
// // expecting any value
// // expecting array items
// // expecting tuble items
// // expecting object properties
// // exitContext()
// const enum DestinationWriteErrorType {
//   CannotWriteToDestination,
// }

// class Destination {
//   private constructor(
//     private string: string,
//     private status: WriteStatus,
//     private configuration: Configuration,
//   ) {}

//   write(string: string) {
//     this.string += string;
//   } 
// }

// class Writer {
//   private constructor(
//     private readonly destination: Destination,
//     private readonly configuration: Configuration,
//     private isClosed: boolean,
//     private earilerContexts: Context[],
//     private currentContext: Context,
//   ) {}

//   private write(string: string) {
//     this.destination.write(string)
//   }

//   private expectingNull(): boolean {}
//   private expectingUndefined(): boolean {}
//   private expectingNumber(): boolean {}
//   private expectingString(): boolean {}
//   private expectingBoolean(): boolean {}
//   private expectingOpenArray(): boolean {}
//   private expectingArrayItem(): boolean {}
//   private expectingCloseArray(): boolean {}
//   private expectingOpenObject(): boolean {}
//   private expectingCloseObject(): boolean {}
//   private expectingObjectProperty(): boolean {}
//   private exitConetxt() {
//     const earilerContext = this.earilerContexts.pop();
//     if (earilerContext === undefined) {
//       this.isClosed = true;
//     } else {
//       this.currentContext = earilerContext;
//     }
//   }
//   private enterArrayContext() {}
//   private enterObjectContext() {}
//   private enterObjectPropertyContext() {}
//   private exitObjectPropertyContext() {}
//   private exitObjectContext() {}
//   private enterArrayItemContext() {}
//   private exitArrayItemContext() {}

//   writeNull(): Success | Failure {
//     if (this.expectingNull()) {
//       this.write(serializeNull());
//       this.exitConetxt();
//       return SUCCESS;
//     } else {
//       return FAILURE;
//     }
//   }
  
//   writeUndefined() {
//     if (this.expectingUndefined()) {
//       this.write(serializeUndefined());
//       this.exitConetxt();
//       return SUCCESS;
//     } else {
//       return FAILURE;
//     }
//   }
  
//   writeNumber(number: number) {
//     if (this.expectingNumber()) {
//       this.write(serializeNumber(number));
//       this.exitConetxt();
//       return SUCCESS;
//     }

//     return FAILURE;
//   }
  
//   writeString(string: string) {
//     if (this.expectingString()) {
//       this.write(serializeString(string));
//       this.exitConetxt();
//       return SUCCESS;
//     }

//     return FAILURE;
//   }

//   writeBoolean(boolean: boolean) {
//     if (this.expectingBoolean()) {
//       this.write(serializeBoolean(boolean));
//       this.exitConetxt();
//       return SUCCESS;
//     }

//     return FAILURE;
//   }
  
//   // writeBigInt(bigint: bigint) {
//   //   this.write(serializeBigInt(bigint));
//   // }

//   openObject() {
//     if (this.expectingOpenObject()) {
//       this.enterObjectContext();
//       return SUCCESS;
//     }
    
//     return FAILURE;
//   }

//   writeObjectProperty<Value>(
//     name: string,
//     value: Value,
//     print: Print<Value>,
//   ) {
//     if (this.expectingObjectProperty()) {
//       this.write(serializeString(name));
//       this.write(": ");

//       this.enterObjectPropertyContext();
//       print.write(value, this);
//       this.exitObjectPropertyContext();

//       return SUCCESS;
//     }

//     return FAILURE;
//   }

//   closeObject() {
//     if (this.expectingCloseObject()) {
//       this.exitObjectContext();
//       return SUCCESS;
//     }

//     return FAILURE;
//   }

//   writeArrayItem<Item>(item: Item, print: Print<Item>) {
//     if (this.expectingArrayItem()) {
//       this.enterArrayItemContext();
//       print.write(item, this);
//       this.exitArrayItemContext();
//       return SUCCESS;
//     }

//     return FAILURE;
//   }

//   writeArraySlice() {}

//   openArray() {
//     if (this.expectingOpenArray()) {
//       this.enterArrayContext();
//       return SUCCESS;
//     }

//     return FAILURE;
//   }
// }

// enum ObjectWriteStatus {
//   WroteNoProperties,
//   WroteSomeProperties,
//   WritingProperty,
//   Closed,
// }

// class ObjectWriter {
//   private constructor(
//     private readonly destination: Destination,
//     private readonly configuration: Configuration,
//     private writeStatus: ObjectWriteStatus,
//   ) {}

//   static create(
//     destination: Destination,
//     configuration: Configuration,
//   ) {
//     return new ObjectWriter(destination, configuration, ObjectWriteStatus.WroteNoProperties);
//   }

//   writePropertyOrThrow<Value>(name: string, value: Value, valuePrint: Print<Value>) {
//     switch (this.writeStatus) {
//       case ObjectWriteStatus.WroteNoProperties: {
//         this.destination.write(`{\n${this.configuration.getIndentation()}`);
//         this.configuration.goDown();
//         this.writeStatus = ObjectWriteStatus.WritingProperty;
//         break;
//       }
//       case ObjectWriteStatus.WroteSomeProperties: {
//         this.destination.write(`, \n${this.configuration.getIndentation()}`);
//         this.writeStatus = ObjectWriteStatus.WritingProperty;
//         break;
//       }
//       case ObjectWriteStatus.WritingProperty: {
//         throw new Error("");
//       }
//       case ObjectWriteStatus.Closed: {
//         throw new Error("");
//       }
//     }

//     this.destination.write(name);
//     this.destination.write(": ");
//     valuePrint.write(value, this.destination, this.configuration);

//     this.writeStatus = ObjectWriteStatus.WroteSomeProperties;
//   }

//   closeOrThrow() {
//     switch (this.writeStatus) {
//       case ObjectWriteStatus.WroteNoProperties: {
//         this.destination.write(`{}`);
//         this.writeStatus = ObjectWriteStatus.Closed;
//         break;
//       }
//       case ObjectWriteStatus.WroteSomeProperties: {
//         this.configuration.goUp();
//         this.destination.write(`\n${this.configuration.getIndentation()}}`);
//         this.writeStatus = ObjectWriteStatus.Closed;
//         break;
//       }
//       case ObjectWriteStatus.WritingProperty: {
//         throw new Error("");
//       }
//       case ObjectWriteStatus.Closed: {
//         throw new Error("");
//       }
//     }
//   }
// }

// enum ArrayWriteStatus {
//   WroteNoItems,
//   WroteSomeItems,
//   WritingItem,
//   Closed,
// }

// class ArrayWriter {
//   private constructor(
//     private readonly destination: Destination,
//     private readonly configuration: Configuration,
//     private writeStatus: ArrayWriteStatus,
//   ) {}

//   static create(
//     destination: Destination,
//     configuration: Configuration,
//   ) {
//     return new ArrayWriter(destination, configuration, ArrayWriteStatus.WroteNoItems);
//   }

//   writeItemOrThrow<Value>(value: Value, valuePrint: Print<Value>) {
//     switch (this.writeStatus) {
//       case ArrayWriteStatus.WroteNoItems: {
//         this.destination.write(`[\n${this.configuration.getIndentation()}`);
//         this.configuration.goDown();
//         this.writeStatus = ArrayWriteStatus.WritingItem;
//         break;
//       }
//       case ArrayWriteStatus.WroteSomeItems: {
//         this.destination.write(`, \n${this.configuration.getIndentation()}`);
//         this.writeStatus = ArrayWriteStatus.WritingItem;
//         break;
//       }
//       case ArrayWriteStatus.WritingItem: {
//         throw new Error("");
//       }
//       case ArrayWriteStatus.Closed: {
//         throw new Error("");
//       }
//     }

//     valuePrint.write(value, this.destination, this.configuration);

//     this.writeStatus = ArrayWriteStatus.WroteSomeItems;
//   }

//   getItemWriter() {
//     return 
//   }

//   closeOrThrow() {
//     switch (this.writeStatus) {
//       case ArrayWriteStatus.WroteNoItems: {
//         this.destination.write(`[]`);
//         this.writeStatus = ArrayWriteStatus.Closed;
//         break;
//       }
//       case ArrayWriteStatus.WroteSomeItems: {
//         this.configuration.goUp();
//         this.destination.write(`\n${this.configuration.getIndentation()}]`);
//         this.writeStatus = ArrayWriteStatus.Closed;
//         break;
//       }
//       case ArrayWriteStatus.WritingItem: {
//         throw new Error("");
//       }
//       case ArrayWriteStatus.Closed: {
//         throw new Error("");
//       }
//     }
//   }
// }

// enum TubleWriteStatus {
//   WroteNoItems,
//   WroteSomeItems,
//   WritingItem,
//   Closed,
// }

// class TubleWriter {
//   private constructor(
//     private readonly destination: Destination,
//     private readonly configuration: Configuration,
//     private writeStatus: TubleWriteStatus,
//   ) {}

//   static create(
//     destination: Destination,
//     configuration: Configuration,
//   ) {
//     return new TubleWriter(destination, configuration, TubleWriteStatus.WroteNoItems);
//   }

//   writeItemOrThrow<Value>(value: Value, valuePrint: Print<Value>) {
//     switch (this.writeStatus) {
//       case TubleWriteStatus.WroteNoItems: {
//         this.destination.write(`(\n${this.configuration.getIndentation()}`);
//         this.configuration.goDown();
//         this.writeStatus = TubleWriteStatus.WritingItem;
//         break;
//       }
//       case TubleWriteStatus.WroteSomeItems: {
//         this.destination.write(`, \n${this.configuration.getIndentation()}`);
//         this.writeStatus = TubleWriteStatus.WritingItem;
//         break;
//       }
//       case TubleWriteStatus.WritingItem: {
//         throw new Error("");
//       }
//       case TubleWriteStatus.Closed: {
//         throw new Error("");
//       }
//     }

//     valuePrint.write(value, this.destination, this.configuration);

//     this.writeStatus = TubleWriteStatus.WroteSomeItems;
//   }

//   closeOrThrow() {
//     switch (this.writeStatus) {
//       case TubleWriteStatus.WroteNoItems: {
//         this.destination.write(`()`);
//         this.writeStatus = TubleWriteStatus.Closed;
//         break;
//       }
//       case TubleWriteStatus.WroteSomeItems: {
//         this.configuration.goUp();
//         this.destination.write(`\n${this.configuration.getIndentation()})`);
//         this.writeStatus = TubleWriteStatus.Closed;
//         break;
//       }
//       case TubleWriteStatus.WritingItem: {
//         throw new Error("");
//       }
//       case TubleWriteStatus.Closed: {
//         throw new Error("");
//       }
//     }
//   }
// }

// const enum Context {
//   ExpectingWrite,
//   ExpectingClose,
//   ExpectingObjectProperty,
// }


// // // const Null_toPrintableRepr = () => {
// // //   return "null";
// // // };
// // // const Undefined_toPrintableRepr = () => {
// // //   return "undefined";
// // // };
// // // const Number_toPrintableRepr = (number: number) => {
// // //   return number.toString();
// // // };
// // // const String_toPrintableRepr = (string: string) => {
// // //   return `${string.replaceAll(/"/ug, '\\"')}`;
// // // };
// // // const Boolean_toPrintableRepr = (boolean: boolean) => {
// // //   return boolean ? "true" : "false";
// // // };
// // // const BigInt_toPrintableRepr = (bigint: bigint) => {
// // //   return bigint.toString();
// // // };

// // // interface Configuration {
// // //   level: number,
// // //   indentation: string,
// // //   indentationLength: number,
// // // }

// // // const Configuration_getIndentation = (me: Configuration): string => {
// // //   return me.indentation.repeat(me.level);
// // // };
// // // const Configuration_goDown = (me: Configuration) => {
// // //   me.level += 1;
// // // };
// // // const Configuration_goUp = (me: Configuration) => {
// // //   me.level -= 1;
// // // };

// // // const enum DestinationStatus {
// // //   WriteExpected,
// // //   WritePending,
// // //   WriteComplete,
// // //   Error,
// // // }

// // // const enum DestinationContextCollection {
// // //   Object,
// // //   Array,
// // //   Tuble,
// // //   Set,
// // //   Map,
// // // }

// // // type DestinationContext = {
// // //   readonly collection: DestinationContextCollection,
// // //   insertComma: boolean,
// // // };

// // // const enum DestinationWriteErrorType {
// // //   CannotWriteToDestination,
// // // }

// // // interface Destination {
// // //   string: string,
// // //   status: DestinationStatus,
// // //   configuration: Configuration,
// // // }

// // // const write = (destination: Destination, string: string) => {
// // //   destination.string += string;
// // // };

// // // export const writeNull = (destination: Destination) => {
// // //   if (destination.status !== DestinationStatus.WriteExpected) { 
// // //     return;
// // //   }

// // //   write(destination, Null_toPrintableRepr());
// // // };

// // // export const writeUndefined = (destination: Destination) => {
// // //   if (destination.status !== DestinationStatus.WriteExpected) { 
// // //     return;
// // //   }

// // //   write(destination, Undefined_toPrintableRepr());
// // // };

// // // export const writeNumber = (destination: Destination, number: number) => {
// // //   if (destination.status !== DestinationStatus.WriteExpected) { 
// // //     return;
// // //   }

// // //   write(destination, Number_toPrintableRepr(number));
// // // };

// // // export const writeString = (destination: Destination, string: string) => {
// // //   if (destination.status !== DestinationStatus.WriteExpected) { 
// // //     return;
// // //   }

// // //   write(destination, String_toPrintableRepr(string));
// // // };

// // // export const writeBoolean = (destination: Destination, boolean: boolean) => {
// // //   if (destination.status !== DestinationStatus.WriteExpected) { 
// // //     return;
// // //   }

// // //   write(destination, Boolean_toPrintableRepr(boolean));
// // // };

// // // export const writeBigInt = (destination: Destination, bigint: bigint) => {
// // //   if (destination.status !== DestinationStatus.WriteExpected) { 
// // //     return;
// // //   }

// // //   write(destination, BigInt_toPrintableRepr(bigint));
// // // };

// // // export type ObjectWriteContext = {
// // //   didWriteSomeProperties: boolean,
// // // };

// // // const ObjectWriteContext_getDestination = (me: ObjectWriteContext): Destination => {
// // //   throw "";
// // // };
// // // const ObjectWriteContext_getConfiguration = (me: ObjectWriteContext): Configuration => {
// // //   throw "";
// // // };
// // // const ObjectWriteContext_getDidWriteSomeProperties = (me: ObjectWriteContext): boolean => {
// // //   throw "";
// // // };
// // // const ObjectWriteContext_setDidWriteSomeProperties = (me: ObjectWriteContext, newValue: boolean) => {
// // //   throw "";
// // // };

// // // export const writeObjectProperty = <Value>(
// // //   context: ObjectWriteContext,
// // //   propertyName: string,
// // //   propertyValue: Value,
// // //   propertyValuePrint: Print<Value>,
// // // ) => {
// // //   const destination = ObjectWriteContext_getDestination(context);
// // //   const configuration = ObjectWriteContext_getConfiguration(context);

// // //   // TODO: Handle maximum line length
// // //   // TODO: Handle spliting properties across new lines

// // //   if (ObjectWriteContext_getDidWriteSomeProperties(context)) { 
// // //     write(destination, ", ");
// // //   }
  
// // //   write(destination, Configuration_getIndentation(configuration));
// // //   write(destination, '"');
// // //   write(destination, propertyName.replaceAll(/"/ug, '\\"'));
// // //   write(destination, '"');
// // //   write(destination, ": ");

// // //   Configuration_goDown(configuration);
// // //   propertyValuePrint.write(propertyValue, destination);
// // //   Configuration_goUp(configuration);

// // //   ObjectWriteContext_setDidWriteSomeProperties(context, true);
// // // };

// // // export type ArrayWriteContext = {
// // //   didWriteSomeItems: boolean,
// // // };

// // // const ArrayWriteContext_getDestination = (me: ArrayWriteContext): Destination => {
// // //   throw "";
// // // };
// // // const ArrayWriteContext_getConfiguration = (me: ArrayWriteContext): Configuration => {
// // //   throw "";
// // // };
// // // const ArrayWriteContext_getDidWriteSomeItems = (me: ArrayWriteContext): boolean => {
// // //   throw "";
// // // };
// // // const ArrayWriteContext_setDidWriteSomeItems = (me: ArrayWriteContext, newValue: boolean) => {
// // //   throw "";
// // // };

// // // export const writeArrayItem = <Item>(
// // //   context: ArrayWriteContext,
// // //   item: Item,
// // //   Item_print: Print<Item>,
// // // ) => {
// // //   // TODO: Handle maximum line length
// // //   // TODO: Handle spliting items across new lines

// // //   const destination = ArrayWriteContext_getDestination(context);
// // //   const configuration = ArrayWriteContext_getConfiguration(context);

// // //   if (ArrayWriteContext_getDidWriteSomeItems(context)) {
// // //     write(destination, ", ");
// // //   }

// // //   Configuration_goDown(configuration);
// // //   Item_print.write(item, destination);
// // //   Configuration_goUp(configuration);

// // //   ArrayWriteContext_setDidWriteSomeItems(context, true);
// // // };

// // export interface Print<Value> {
// //   name(): string;
// //   describe(value: Value): Descriptor;
// //   // write(value: Value, destination: Destination): void;
// // }

// // // interface InternalPrint<Value> {
// // //   toPrintable(value: Value): Descriptor;
// // // }

// // export const implement = <Value>({
// //   name,
// //   describe,
// // }: {
// //   name: () => string,
// //   describe: (value: Value) => Descriptor,
// // }): Print<Value> => {
// //   return {
// //     name: name,
// //     describe: describe,
// //   };
// // };

// // // const implement = <Value>({
// // //   describe: toPrintable,
// // // }: {
// // //   describe: (value: Value) => Descriptor,
// // // }): InternalPrint<Value> => {
// // //   return {
// // //     toPrintable,
// // //   };
// // // };

// // const enum SerializerType {
// //   Null,
// //   Undefined,
// //   Number,
// //   String,
// //   Boolean,
// //   Array,
// //   Object,
// //   Tuble,
// //   Error,
// //   Set,
// //   Map,
// //   EnumUnitVariant,
// //   EnumDataVariant,
// //   Wrapper,
// // }

// // type Descriptor = (
// //   | NullDescriptor
// //   | NumberDescriptor
// //   | UndefinedDescriptor
// //   | StringDescriptor
// //   | ArrayDescriptor
// //   | TubleDescriptor
// //   | EnumDataVariantDescriptor
// //   | EnumUnitVariantDescriptor
// //   | ObjectDescriptor
// //   | BooleanDescriptor
// //   // | PrintableWrapperType
// // );

// // export interface NullDescriptor {
// //   readonly type: SerializerType.Null,
// // }

// // export const NullDescriptor = (): NullDescriptor => {
// //   return {
// //     type: SerializerType.Null,
// //   };
// // };

// // export interface UndefinedDescriptor {
// //   readonly type: SerializerType.Undefined,
// // }

// // export const UndefinedDescriptor = (): UndefinedDescriptor => {
// //   return {
// //     type: SerializerType.Undefined,
// //   };
// // };

// // export interface NumberDescriptor {
// //   readonly type: SerializerType.Number,
// //   readonly number: number,
// // }

// // export const NumberDescriptor = (number: number): NumberDescriptor => {
// //   return {
// //     type: SerializerType.Number,
// //     number,
// //   };
// // };

// // export interface StringDescriptor {
// //   readonly type: SerializerType.String,
// //   readonly string: string,
// // }

// // export const StringDescriptor = (string: string): StringDescriptor => {
// //   return {
// //     type: SerializerType.String,
// //     string,
// //   };
// // };

// // export interface BooleanDescriptor {
// //   readonly type: SerializerType.Boolean,
// //   readonly boolean: boolean,
// // }

// // export const BooleanDescriptor = (boolean: boolean): BooleanDescriptor => {
// //   return {
// //     type: SerializerType.Boolean,
// //     boolean,
// //   };
// // };

// // export interface ArrayDescriptor {
// //   readonly type: SerializerType.Array,
// //   readonly items: Descriptor[],
// // }

// // export const ArrayDescriptor = (): ArrayDescriptor => {
// //   return {
// //     type: SerializerType.Array,
// //     items: [],
// //   };
// // };

// // export const ArrayDescriptor_append = <Item>(
// //   me: ArrayDescriptor,
// //   item: Item,
// //   itemPrint: Print<Item>,
// // ) => {
// //   me.items.push(itemPrint.describe(item));
// // };

// // export interface TubleDescriptor {
// //   readonly type: SerializerType.Tuble,
// //   readonly items: Descriptor[],
// // }

// // export const TubleDescriptor = (): TubleDescriptor => {
// //   return {
// //     type: SerializerType.Tuble,
// //     items: [],
// //   };
// // };

// // export const TubleDescriptor_append = <Item>(
// //   me: TubleDescriptor,
// //   item: Item,
// //   itemPrint: Print<Item>,
// // ) => {
// //   me.items.push(itemPrint.describe(item));
// // };

// // export interface ObjectDescriptor {
// //   readonly type: SerializerType.Object,
// //   readonly properties: Map<string, Descriptor>,
// // }

// // export const ObjectDescriptor = (): ObjectDescriptor => {
// //   return {
// //     type: SerializerType.Object,
// //     properties: new Map(),
// //   };
// // };

// // export const ObjectDescriptor_setProperty = <Value>(
// //   me: ObjectDescriptor,
// //   propertyName: string,
// //   propertyValue: Value,
// //   propertyValuePrint: Print<Value>,
// // ) => {
// //   me.properties.set(propertyName, propertyValuePrint.describe(propertyValue));
// // };

// // export const ObjectDescriptor_setPropertyWithDescriptor = (
// //   me: ObjectDescriptor,
// //   propertyName: string,
// //   propertyValueDescriptor: Descriptor,
// // ) => {
// //   me.properties.set(propertyName, propertyValueDescriptor);
// // };

// // interface EnumUnitVariantDescriptor {
// //   readonly type: SerializerType.EnumUnitVariant,
// //   readonly enumName: string,
// //   readonly enumVariantName: string,
// // }

// // export const EnumUnitVariantDescriptor = (enumName: string, enumVariantName: string): EnumUnitVariantDescriptor => {
// //   return {
// //     type: SerializerType.EnumUnitVariant,
// //     enumName,
// //     enumVariantName,
// //   };
// // };

// // interface EnumDataVariantDescriptor {
// //   readonly type: SerializerType.EnumDataVariant,
// //   readonly enumVariantName: string,
// //   readonly enumVariantData: Descriptor,
// // }

// // export const EnumDataVariantDescriptor = <Data>(
// //   enumVariantName: string, 
// //   enumVariantData: Data,
// //   enumVariantDataPrint: Print<Data>,
// // ): EnumDataVariantDescriptor => {
// //   return {
// //     type: SerializerType.EnumDataVariant,
// //     // enumName,
// //     enumVariantName,
// //     enumVariantData: enumVariantDataPrint.describe(enumVariantData),
// //   };
// // };

// // // export interface PrintableWrapperType {
// // //   readonly type: SerializerType.Wrapper,
// // //   readonly inner: Descriptor,
// // // }

// // // export const PrintableWrapperType = <Inner>(
// // //   inner: Inner, 
// // //   innerPrint: Print<Inner>,
// // // ): PrintableWrapperType => {
// // //   return {
// // //     type: SerializerType.Wrapper,
// // //     inner: innerPrint.describe(inner),
// // //   };
// // // };

// // export const String_print = implement<string>({
// //   name() {
// //     return "String";
// //   },
// //   describe(value) {
// //     return StringDescriptor(value);
// //   },
// // });

// // export const Number_print = implement<number>({
// //   name() {
// //     return "Number";
// //   },
// //   describe(value) {
// //     return NumberDescriptor(value);
// //   },
// // });

// // export const Boolean_print = implement<boolean>({
// //   name() {
// //     return "Boolean";
// //   },
// //   describe(value) {
// //     return BooleanDescriptor(value);
// //   },
// // });


// // interface Configuration {
// //   level: number,
// //   indentation: string,
// //   indentationLength: number,
// // }

// // const Configuration_getIndentation = (me: Configuration): string => {
// //   return me.indentation.repeat(me.level);
// // };
// // const Configuration_goDown = (me: Configuration) => {
// //   me.level += 1;
// // };
// // const Configuration_goUp = (me: Configuration) => {
// //   me.level -= 1;
// // };

// // interface Destination {
// //   string: string,
// // }

// // const Destination_write = (destination: Destination, string: string) => {
// //   destination.string += string;
// // };

// // const Descriptor_serialize = (
// //   descriptor: Descriptor, 
// //   destination: Destination,
// //   configuration: Configuration,
// // ) => {
// //   switch (descriptor.type) {
// //     case SerializerType.Null: {
// //       Destination_write(destination, "null");
// //       return;
// //     }
// //     case SerializerType.Undefined: {
// //       Destination_write(destination, "undefined");
// //       return;
// //     }
// //     case SerializerType.Number: {
// //       Destination_write(destination, descriptor.number.toString());
// //       return;
// //     }
// //     case SerializerType.String: {
// //       Destination_write(destination, `"${descriptor.string.replaceAll(/"/, '\\"')}"`);
// //       return;
// //     }
// //     case SerializerType.Boolean: {
// //       Destination_write(destination, descriptor.boolean ? "true" : "false");
// //       return;
// //     }
// //     case SerializerType.Array: {
// //       Destination_write(destination, "[\n");

// //       Configuration_goDown(configuration);
// //       for (const item of descriptor.items) {
// //         Descriptor_serialize(item, destination, configuration);
// //       }
// //       Configuration_goUp(configuration);

// //       const closingIndentation = Configuration_getIndentation(configuration);
// //       Destination_write(destination, `\n${closingIndentation}]`);
// //       return;
// //     }
// //     case SerializerType.Object: {
// //       Destination_write(destination, "{\n");

// //       Configuration_goDown(configuration);
// //       const propertyIndentation = Configuration_getIndentation(configuration);

// //       let insertComma = false;
// //       for (const [ propertyName, propertyValue ] of descriptor.properties) {
// //         if (insertComma) {
// //           Destination_write(destination, `, `);
// //         } else {
// //           insertComma = true;
// //         }

// //         Destination_write(destination, `\n${propertyIndentation}${propertyName}: `);
// //         Descriptor_serialize(propertyValue, destination, configuration);
// //       }

// //       Configuration_goUp(configuration);
// //       const closingIndentation = Configuration_getIndentation(configuration);

// //       Destination_write(destination, `${closingIndentation}}`);
// //       return;
// //     }
// //     case SerializerType.Tuble: {
      
// //     }
// //     case SerializerType.EnumUnitVariant:
// //     case SerializerType.EnumDataVariant:
// //     case SerializerType.Wrapper:
// //   }
// // };

// // // interface Display {
// // //   readonly name: string,
// // //   readonly descriptor: Descriptor,
// // // }

// // // const implementForNull = ({
// // //   typeName
// // // }: {
// // //   typeName: string,
// // // }) => {};

// // // const implementForNumber = ({
// // //   typeName
// // // }: {
// // //   typeName: string,
// // // }) => {};

// // // const implementForString = ({
// // //   typeName
// // // }: {
// // //   typeName: string,
// // // }) => {};

// // // const implementForBoolean = ({
// // //   typeName
// // // }: {
// // //   typeName: string,
// // // }) => {};

// // // const implementForArray = ({
// // //   typeName
// // // }: {
// // //   typeName: string,
// // // }) => {};

// // // const implementForTuble = ({
// // //   typeName
// // // }: {
// // //   typeName: string,
// // // }) => {};

// // // const implementForNumber = ({
// // //   typeName
// // // }: {
// // //   typeName: string,
// // // }) => {};

// // // const implement = <T>({
// // //   name,
// // //   descriptor,
// // // }: {
// // //   name: string,
// // //   descriptor: Descriptor,
// // // }): Display => {

// // // };

// // // const enum DescriptorType {
// // //   Null,
// // //   Undefined,
// // //   Number,
// // //   String,
// // //   Boolean,
// // //   BigInt,
// // //   Date,
// // //   Set,
// // //   Map,
// // //   Array,
// // //   Object,
// // //   Error,
// // //   Unknown,
// // //   Tuble,
// // // }

// // // type Descriptor = (
// // //   | NullDescriptor
// // //   | UndefinedDescriptor
// // //   | NumberDescriptor
// // //   | StringDescriptor
// // //   | BooleanDescriptor
// // //   | BigIntDescriptor
// // //   | SetDescriptor
// // // )

// // // interface NullDescriptor {
// // //   readonly type: DescriptorType.Null,
// // // }

// // // export const NullDescriptor = (): NullDescriptor => {
// // //   return {
// // //     type: DescriptorType.Null,
// // //   };
// // // };

// // // interface UndefinedDescriptor {
// // //   readonly type: DescriptorType.Undefined,
// // // }

// // // export const UndefinedDescriptor = (): UndefinedDescriptor => {
// // //   return {
// // //     type: DescriptorType.Undefined,
// // //   };
// // // };

// // // interface NumberDescriptor {
// // //   readonly type: DescriptorType.Number,
// // // }

// // // export const NumberDescriptor = (): NumberDescriptor => {
// // //   return {
// // //     type: DescriptorType.Number,
// // //   };
// // // };

// // // interface StringDescriptor {
// // //   readonly type: DescriptorType.String,
// // // }

// // // export const StringDescriptor = (): StringDescriptor => {
// // //   return {
// // //     type: DescriptorType.String,
// // //   };
// // // };

// // // interface BooleanDescriptor {
// // //   readonly type: DescriptorType.Boolean,
// // // }

// // // export const BooleanDescriptor = (): BooleanDescriptor => {
// // //   return {
// // //     type: DescriptorType.Boolean,
// // //   };
// // // };

// // // interface BigIntDescriptor {
// // //   readonly type: DescriptorType.BigInt,
// // // }

// // // export const BigIntDescriptor = (): BigIntDescriptor => {
// // //   return {
// // //     type: DescriptorType.BigInt,
// // //   };
// // // };

// // // interface DateDescriptor {
// // //   readonly type: DescriptorType.Date,
// // // }

// // // export const DateDescriptor = (): DateDescriptor => {
// // //   return {
// // //     type: DescriptorType.Date,
// // //   };
// // // };

// // // interface SetDescriptor<Member> {
// // //   readonly type: DescriptorType.Set,
// // //   readonly memberDescriptor: Descriptor,
// // // }

// // // export const SetDescriptor = <Member>(memberDescriptor: Descriptor): SetDescriptor<Member> => {
// // //   return {
// // //     type: DescriptorType.Set,
// // //     memberDescriptor,
// // //   };
// // // };

// // // interface MapDescriptor<Key, Value> {
// // //   readonly type: DescriptorType.Map,
// // //   readonly keyDescriptor: Descriptor,
// // //   readonly valueDescriptor: Descriptor,
// // // }

// // // export const MapDescriptor = <Key, Value>(
// // //   keyDescriptor: Descriptor, 
// // //   valueDescriptor: Descriptor,
// // // ): MapDescriptor<Key, Value> => {
// // //   return {
// // //     type: DescriptorType.Map,
// // //     keyDescriptor,
// // //     valueDescriptor,
// // //   };
// // // };

// // // interface ArrayDescriptor<Item> {
// // //   readonly type: DescriptorType.Array,
// // //   readonly itemDescriptor: Descriptor,
// // // }

// // // export const ArrayDescriptor = <Item>(itemDescriptor: Descriptor): ArrayDescriptor<Item> => {
// // //   return {
// // //     type: DescriptorType.Array,
// // //     itemDescriptor,
// // //   };
// // // };

// // // interface ErrorDescriptor {
// // //   readonly type: DescriptorType.Error,
// // // }

// // // export const ErrorDescriptor = (): ErrorDescriptor => {
// // //   return {
// // //     type: DescriptorType.Error,
// // //   };
// // // };

// // // interface TubleDescriptor<Items> {
// // //   readonly type: DescriptorType.Tuble,
// // //   readonly itemDescriptors: Descriptor[],
// // // }

// // // export const TubleDescriptor = <Items>(...itemDescriptors: Descriptor[]): TubleDescriptor<Items> => {
// // //   return {
// // //     type: DescriptorType.Tuble,
// // //     itemDescriptors,
// // //   };
// // // };

// // // interface PropertyDescriptor<Object, Value> {
// // //   readonly get: (object: Object) => Value,
// // //   readonly descriptor: Descriptor,
// // // }

// // // interface ObjectDescriptor {
// // //   readonly type: DescriptorType.Object,
// // //   readonly propertyDescriptors: { [key: string]: PropertyDescriptor<unknown, unknown> },
// // // }

// // // export const ObjectDescriptor = (
// // //   propertyDescriptors: { [key: string]: Descriptor },
// // // ): ObjectDescriptor => {
// // //   return {
// // //     type: DescriptorType.Object,
// // //     propertyDescriptors,
// // //   };
// // // };

// // // interface Destination {
// // //   string: string,
// // //   level: number,
// // //   indentation: string,
// // //   prependIndentation: boolean,
// // // }

// // // // interface Display<Value> {
// // // //   serializeForDisplay(value: Value): void;
// // // //   write(destination: Destination, value: Value): void;
// // // // }

// // // structuredClone
// // // const Null_serializeForDisplay = () => {
// // //   return "null";
// // // };
// // // const Undefined_serializeForDisplay = () => {
// // //   return "undefined";
// // // };
// // // const String_serializeForDisplay = (string: string) => {
// // //   return `"${string.replaceAll(/"/gu, '\\"')}"`;
// // // };
// // // const Number_serializeForDisplay = (number: number) => {
// // //   return number.toString();
// // // };
// // // const Boolean_serializeForDisplay = (boolean: boolean) => {
// // //   return boolean ? "true" : "false";
// // // };

// // // const writeNull = (destination: Destination) => {
// // //   if (destination.prependIndentation) {
// // //     destination.string += destination.indentation
// // //   }
// // //   destination.string += Null_serializeForDisplay();
// // // };
// // // const writeUndefined = (destination: Destination) => {
// // //   destination.string += Undefined_serializeForDisplay();
// // // };
// // // const writeString = (destination: Destination, string: string) => {
// // //   destination.string += String_serializeForDisplay(string);
// // // };
// // // const writeNumber = (destination: Destination, number: number) => {
// // //   destination.string += Number_serializeForDisplay(number);
// // // };
// // // const writeBoolean = (destination: Destination, boolean: boolean) => {
// // //   destination.string += Boolean_serializeForDisplay(boolean);
// // // };
// // // const writeArray = <Item>(destination: Destination, array: Item[], itemDisplay: Display<Item>) => {
// // //   destination.string += destination.indentation.repeat(destination.level);
// // //   destination.string += "[";
// // //   destination.level += 1;

// // //   for (const item of array) {
    
// // //   }
// // // };

// // // export const writeString = (destination: ValueDestination, string: string) => {
// // //   destination.string.string += `"${string.replaceAll(/"/gu, '\\"')}"`;
// // // };

// // // export const writeNumber = (destination: ValueDestination, number: number) => {
// // //   destination.string.string += number.toString();
// // // };

// // // export const writeBoolean = (destination: ValueDestination, boolean: boolean) => {
// // //   destination.string.string += boolean ? "true" : "false";
// // // };

// // // export const writeNull = (destination: ValueDestination) => {
// // //   destination.string.string += "null";
// // // };

// // // export const writeSymbol = (destination: ValueDestination, symbol: symbol) => {
// // //   destination.string.string += symbol.toString();
// // // };

// // // export const writeBigInt = (destination: ValueDestination, bigint: bigint) => {
// // //   destination.string.string += bigint.toString();
// // // };

// // // export const writeArray = <Item>(destination: ValueDestination, array: Item[], itemDisplay: Display<Item>) => {
// // //   destination.string.string += "[";
// // //   let x = true;

// // //   for (const item of array) {

// // //     destination.string.string += ", \n" + destination;
// // //     itemDisplay.write(destination, item);
// // //   }
// // // };

// // // export const writeMap = () => {}
// // // export const writeSet = () => {}
// // // export const writeDate = () => {}
// // // export const writeError = () => {}
// // // export const writeTuble = () => {}
// // // export const writeTubleItem = () => {}
// // // export const writeObject = () => {}
// // // export const writeObjectProperty = () => {}
// // // const enum DestinationType {
// // //   Array,
// // //   Object,
// // //   Tuble,
// // // }

// // // interface StringRef {
// // //   string: string,
// // // }

// // // interface ArrayDestination {
// // //   string: StringRef,
// // // }

// // // interface TubleDestination {
// // //   string: StringRef,
// // // }

// // // interface ValueDestination {
// // //   string: StringRef,

// // // }

// // // const Countdown_print = Print.implement<CountdownT>({
// // //   name: "Countdown",
// // //   descriptor: Print.Descriptor_Object({
// // //     remainingDuration: Print.Descriptor_Property({
// // //       get: Countdown.getRemainingDuration,
// // //       descriptor: Duration_printDescriptor,
// // //     }),
// // //     previousSynchronizationTime: Print.Descriptor_Property({
// // //       get: Countdown.getPreviousSycnhronizationTime,
// // //       descriptor: DateTime_printDescriptor,
// // //     }),
// // //   }),
// // // });

// // const SUCCESS = Symbol();
// // const FAILURE = Symbol();

// // type Success = typeof SUCCESS;
// // type Failure = typeof FAILURE;

// // type Result = Success | Failure;

// // interface ToTextualRepr<Value> {

// // }

// // interface Writer {
// //   writeNull(): Result;
// //   writeUndefined(): Result;
// //   writeNumber(number: number): Result;
// //   writeBoolean(boolean: boolean): Result;
// //   writeBigInt(bigint: bigint): Result;
// //   writeString(string: string): Result;
// //   writeArray<Item>(array: Item[], print: ToTextualRepr<Item>): Result;
// //   openArrayWriter<Item>(): ArrayWriter<Item> | Failure;
// //   writeTuble<Item>(tuble: Item[], print: ToTextualRepr<Item>): Result;
// //   openTubleWriter<Item>(): TubleWriter<Item> | Failure;
// //   openObjectWriter(): ObjectWriter | Failure;
// //   writeEnumUnitVariant(enumName: string, variantName: string): Result;
// //   writeEnumDataVariant<Data>(enumName: string, variantName: string, variantData: Data, print: ToTextualRepr<Data>): Result;
// // }

// // interface ArrayWriter<Item> {
// //   writeItem(item: Item, print: ToTextualRepr<Item>): Result;
// //   close(): Result;
// // }

// // interface TubleWriter<Item> {
// //   writeItem(item: Item, print: ToTextualRepr<Item>): Result;
// //   close(): Result;
// // }

// // interface ObjectWriter {
// //   writeProperty<Value>(name: string, value: Value, print: ToTextualRepr<Value>): Result;
// //   close(): Result;
// // }

// // // class Print2<Value> {
// // //   private constructor(
// // //     readonly write: IsWriter,
// // //   ) {}
// // // }

// // // class Writer {
// // //   private constructor(private string: string) {}

// // //   private write(string: string) {
// // //     this.string += string;
// // //   }

// // //   writeNull() {
// // //     this.write("null");
// // //   }

// // //   writeUndefined() {
// // //     this.write("undefined");
// // //   }

// // //   writeBoolean(boolean: boolean) {
// // //     this.write(boolean ? "true" : "false");
// // //   }

// // //   writeNumber(number: number) {
// // //     this.write(number.toString());
// // //   }

// // //   writeBigInt(bigint: bigint) {
// // //     this.write(bigint.toString());
// // //   }

// // //   writeString(string: string) {
// // //     this.write(`"${string.replaceAll(/"/ug, '\\"')}"`);
// // //   }

// // //   createArrayWriter() {

// // //   }
// // // }

// // // class ArrayWriter<Item> {
// // //   private constructor(writer: Writer) {
    
// // //   }

// // //   writeItem(item: Item) {
    
// // //   }
// // // }

// // // export type It = {
// // //   string: string,
// // // };

// // // const It_write = (it: It, string: string) => {
// // //   it.string += string;
// // // };

// // // const It_writeNull = (it: It) => {
// // //   It_write(it, "null");
// // // };

// // // const It_writeUndefined = (it: It) => {
// // //   It_write(it, "undefined")
// // // }


// // export const writeObjectProperty = <Value>(
// //   context: ObjectWriteContext,
// //   propertyName: string,
// //   propertyValue: Value,
// //   propertyValuePrint: Print<Value>,
// // ) => {
// //   const destination = ObjectWriteContext_getDestination(context);
// //   const configuration = ObjectWriteContext_getConfiguration(context);

// //   // TODO: Handle maximum line length
// //   // TODO: Handle spliting properties across new lines

// //   if (ObjectWriteContext_getDidWriteSomeProperties(context)) { 
// //     write(destination, ", ");
// //   }
  
// //   write(destination, Configuration_getIndentation(configuration));
// //   write(destination, '"');
// //   write(destination, propertyName.replaceAll(/"/ug, '\\"'));
// //   write(destination, '"');
// //   write(destination, ": ");

// //   Configuration_goDown(configuration);
// //   propertyValuePrint.write(propertyValue, destination);
// //   Configuration_goUp(configuration);

// //   ObjectWriteContext_setDidWriteSomeProperties(context, true);
// // };

// // export type ArrayWriteContext = {
// //   didWriteSomeItems: boolean,
// // };

// // const ArrayWriteContext_getDestination = (me: ArrayWriteContext): Destination => {
// //   throw "";
// // };
// // const ArrayWriteContext_getConfiguration = (me: ArrayWriteContext): Configuration => {
// //   throw "";
// // };
// // const ArrayWriteContext_getDidWriteSomeItems = (me: ArrayWriteContext): boolean => {
// //   throw "";
// // };
// // const ArrayWriteContext_setDidWriteSomeItems = (me: ArrayWriteContext, newValue: boolean) => {
// //   throw "";
// // };

// // export const writeArrayItem = <Item>(
// //   context: ArrayWriteContext,
// //   item: Item,
// //   Item_print: Print<Item>,
// // ) => {
// //   // TODO: Handle maximum line length
// //   // TODO: Handle spliting items across new lines

// //   const destination = ArrayWriteContext_getDestination(context);
// //   const configuration = ArrayWriteContext_getConfiguration(context);

// //   if (ArrayWriteContext_getDidWriteSomeItems(context)) {
// //     write(destination, ", ");
// //   }

// //   Configuration_goDown(configuration);
// //   Item_print.write(item, destination);
// //   Configuration_goUp(configuration);

// //   ArrayWriteContext_setDidWriteSomeItems(context, true);
// // };

// // export interface Print<Value> {
// //   name(): string;
// //   describe(value: Value): Descriptor;
// //   // write(value: Value, destination: Destination): void;
// // }

// // interface InternalPrint<Value> {
// //   toPrintable(value: Value): Descriptor;
// // }

// // export const implement = <Value>({
// //   name,
// //   describe,
// // }: {
// //   name: () => string,
// //   describe: (value: Value) => Descriptor,
// // }): Print<Value> => {
// //   return {
// //     name: name,
// //     describe: describe,
// //   };
// // };

// // // const implement = <Value>({
// // //   describe: toPrintable,
// // // }: {
// // //   describe: (value: Value) => Descriptor,
// // // }): InternalPrint<Value> => {
// // //   return {
// // //     toPrintable,
// // //   };
// // // };

// // // interface Configuration {
// // //   level: number,
// // //   indentationLength: number,
// // //   maximumStringLineLength: number,
// // // }

// // const Configuration_getIndentation = (configuration: Configuration) => {
// //   return " ".repeat(configuration.level * configuration.indentationLength);
// // };

// // // interface Destination {
// // //   string: string,
// // // }

// // const Destination_write = (destination: Destination, string: string) => {
// //   destination.string += string;
// // };

// // // type Writer = {
// // //   destination: Destination,
// // //   configuration: Configuration,
// // // }

// // export const writeNullOrThrow = (
// //   destination: Destination,
// //   configuration: Configuration,
// // ) => {
// //   Destination_write(destination, "null");
// // };

// // export const writeUndefinedOrThrow = (
// //   destination: Destination,
// //   configuration: Configuration,
// // ) => {
// //   Destination_write(destination, "undefined");
// // };

// // export const writeStringOrThrow = (
// //   destination: Destination,
// //   configuration: Configuration,
// //   string: string,
// // ) => {
// //   Destination_write(destination, `"${
// //     string
// //       .replaceAll(/"/ug, '\\"')
// //       .replaceAll(/\n/ug, "\\n")
// //   }"`);
// // };

// // export const writeNumberOrThrow = (
// //   destination: Destination,
// //   configuration: Configuration,
// //   number: number,
// // ) => {
// //   Destination_write(destination, number.toString());
// // };

// // export const writeBigIntOrThrow = (
// //   destination: Destination,
// //   configuration: Configuration,
// //   bigint: bigint,
// // ) => {
// //   Destination_write(destination, bigint.toString());
// // };

// // export const writeBooleanOrThrow = (
// //   destination: Destination,
// //   configuration: Configuration,
// //   boolean: boolean,
// // ) => {
// //   Destination_write(destination, boolean ? "true" : "false");
// // };

// // export const openArrayOrThrow = (
// //   destination: Destination,
// //   configuration: Configuration,
// // ) => {
// //   Destination_write(destination, `[\n`)
// // };

// // export const writeArrayItemOrThrow = <Item>(
// //   destination: Destination,
// //   configuration: Configuration,
// //   item: Item,
// //   itemPrint: Print<Item>
// // ) => {
  
// // };

// // // closeArrayOrThrow();

// const enum DescriptorType {
//   Null,
//   Undefined,
//   Number,
//   String,
//   Boolean,
//   Array,
//   Object,
// }

// type Null = {
//   readonly type: DescriptorType.Null,
// };

// export const Null = (): Null => {
//   return {
//     type: DescriptorType.Null,
//   };
// };

// type Undefined = {
//   readonly type: DescriptorType.Undefined,
// };

// export const Undefined = (): Undefined => {
//   return {
//     type: DescriptorType.Undefined,
//   };
// };

// type Number = {
//   readonly type: DescriptorType.Number,
// };

// export const Number = (): Number => {
//   return {
//     type: DescriptorType.Number,
//   };
// };

// type String = {
//   readonly type: DescriptorType.String,
// };

// export const String = (): String => {
//   return {
//     type: DescriptorType.String,
//   };
// };

// type Boolean = {
//   readonly type: DescriptorType.Boolean,
// };

// export const Boolean = (): Boolean => {
//   return {
//     type: DescriptorType.Boolean,
//   };
// };

// type Array<Item> = {
//   readonly type: DescriptorType.Array,
//   readonly itemDescriptor: any
// };

// export const Array = <Item>({
//   itemDescriptor,
// }: {
//   itemDescriptor: any,
// }): Array<Item> => {
//   return {
//     type: DescriptorType.Array,
//     itemDescriptor,
//   };
// };

// type ObjectProperty<Me, Value> = {
//   readonly name: string,
//   readonly get: (me: Me) => Value,
//   readonly valueDescriptor: any,
// }

// type Object<Me> = {
//   readonly type: DescriptorType.Object,
  
// }

export type PrintWriter = {};

export const enum PrinterType {
  Null,
  Undefined,
  Number,
  String,
  Boolean,
  Object,
  Array,
  Enum,
  Union,
  NewType,
}

export type NullPrinter = {
  readonly type: PrinterType.Null,
  readonly print: (value: null, writer: PrintWriter) => void,
};

export type UndefinedPrinter = {
  readonly type: PrinterType.Undefined,
};
export type NumberPrinter = {
  readonly type: PrinterType.Number,
};