import { TUnique, Unique } from "../discipline_ts/mod.ts";

type Writer = TUnique<"Writer", { 
  buffer: (number | boolean | string)[],
  x: boolean
}>;

export const writeInteger = (writer: Writer, number: number) => {
  writer.buffer.push(number);
};

export const writeString = (writer: Writer, value: string) => {
  writer.buffer.push(value.length);
  writer.buffer.push(value);
};

export const writeArrayLength = (writer: Writer, value: number) => {
  writer.buffer.push(value);
};

export const writeArray
// export type Serializer = TUnique<"Serializer", {
//   readonly serialize: ()
// }>;

const enum JsonableType {
  Null,
  String,
  Number,
  Array,
  Object,
  EnumWithData,
  EnumWithoutData,
}

export type Jsonable = TUnique<"Jsonable", 
  | null 
  | string 
  | number 
  | Jsonable[]
  | { [key in string]: Jsonable }
>;

type Null = TUnique<"Null", {
  readonly type: JsonableType.Null,
}>;

type String = TUnique<"String", {
  readonly type: JsonableType.String,
  readonly value: string,
}>;

type Number = TUnique<"Number", {
  readonly type: JsonableType.Number,
  readonly value: number,
}>;

type Array = TUnique<"Number", {
  readonly type: JsonableType.Array,
  readonly value: Jsonable[],
}>;

type Object = TUnique<"Number", {
  readonly type: JsonableType.Object,
  readonly value: { [key in string]: Jsonable },
}>;

export const Null = (): Null => {
  return Unique.create({
    type: JsonableType.Null,
  });
};

export const String = (value: string): String => {
  return Unique.create({
    type: JsonableType.String,
    value,
  });
};

export const Number = (value: number): Number => {
  return Unique.create({
    type: JsonableType.Number,
    value,
  });
};

export const Array = (value: Jsonable[]): Array => {
  return Unique.create({
    type: JsonableType.Array,
    value,
  });
};

export const Array_withItemSerializer = <Item>(value: Item[], itemSerializer: Serializer<Item>): Array => {
  return Unique.create({
    type: JsonableType.Array,
    value: value.map(itemSerializer.__jsonable),
  })
};

export const Array_addItem = (value: Array, item: Jsonable) => {
  value.value.push(item);
};

export const Array_addItemWithSerializer = <Item>(value: Array, item: Item, serializer: Serializer<Item>) => {
  value.value.push(serializer.__jsonable(item));
};

export const Object = (properties: { [key in string]: Jsonable }): Object => {
  return Unique.create({
    type: JsonableType.Object,
    value: properties,
  })
};

export const Object_addField = (me: Object, fieldName: string, fieldValue: Jsonable) => {
  me.value[fieldName] = fieldValue;
};

export const Object_addFieldWithSerializer = <Value>(me: Object, fieldName: string, fieldValue: Value, serializer: Serializer<Value>) => {
  me.value[fieldName] = serializer.__jsonable(fieldValue);
};

export const Enum_cr
type EnumWithData = TUnique<"Number", { variant: string, data: Jsonable }>;

type EnumWithoutData = TUnique<"Number", { variant: string }>;



export const implement = ({

}: {

}) => {

};