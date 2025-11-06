import { Tried, TypeId, Unique } from "../mod.ts";

type SerializerSerialize<Type> = (value: Type, writer: Writer) => void;

export class Serializer<Type> implements Unique {
  readonly typeId = TypeId.SerializationSerialiezr;

  private constructor(
    readonly __typeName: string,
    readonly __serialize: SerializerSerialize<Type>
  ) {}
}

export class Writer implements Unique {
  readonly typeId = TypeId.SerializationWriter;

  private constructor(private readonly buffer: (number | boolean | string)[]) {}

  writeString(value: string) {
    this.buffer.push(value.length);
    this.buffer.push(value);
  }

  writeInteger(value: number) {
    this.buffer.push(value);
  }

  writeBoolean(value: boolean) {
    this.buffer.push(value);
  }

  writeArray<Item>(value: Item[], itemSerializer: Serializer<Item>) {
    this.buffer.push(value.length);
    value.map(item => itemSerializer.__serialize(item, this))
  }

  writeSerializable<Value>(value: Value, serializer: Serializer<Value>) {
    serializer.__serialize(value, this)
  }
}

export class Deserializer<Type> implements Unique {
  readonly typeId = TypeId.SerializationDeserialiezr;

  private constructor(
    readonly __typeName: string,
    readonly __deserialize: (reader: Reader) => Tried<Type, void>
  ) {}
}

export class Reader {
  
}