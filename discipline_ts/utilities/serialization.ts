import { Tried, isInteger } from "../x.ts";

const FAILURE = Symbol();

export type Failure = typeof FAILURE;

export const Failure = (): Failure => FAILURE;

export interface Destination {
  buffer: unknown[],
}

const Destination_create = (): Destination => {
  return {
    buffer: [],
  };
};

export const Destination_writeNull = (me: Destination) => {
  me.buffer.push(null);
};

export const Destination_writeString = (me: Destination, string: string) => {
  me.buffer.push(string);
};

export const Destination_writeNumber = (me: Destination, number: number) => {
  me.buffer.push(number);
};

export const Destination_writeBoolean = (me: Destination, boolean: boolean) => {
  me.buffer.push(boolean);
};

export const Destination_writeValue = <Value>(me: Destination, value: Value, serialize: Serialize<Value>) => {
  serialize.write(value, me);
};

export const Destination_writeArray = <Item>(me: Destination, array: Item[], serialize: Serialize<Item>) => {
  Destination_writeNumber(me, array.length);
  
  for (const item of array) {
    Destination_writeValue(me, item, serialize);
  }
};

export interface Serialize<Value> {
  write(value: Value, destination: Destination): void;
  serialize(value: Value): string | Error;
}

export const Serialize_implement = <Value>({
  write,
}: {
  write(value: Value, destination: Destination): void;
}): Serialize<Value> => {
  return {
    write,

    serialize(value) {
      const destination = Destination_create();
      write(value, destination);

      try {
        return JSON.stringify(destination.buffer);
      } catch (cause) {
        return new Error("Serializing a value: JSON.stringify failed", { cause });
      }
    },
  }
};

export interface Source {
  buffer: unknown[],
  index: number,
}

const Source_create = (buffer: unknown[]): Source => {
  return {
    buffer,
    index: 0,
  };
};

export const Source_readInteger = (me: Source): Failure | number => {
  const value = me.buffer.at(me.index);
  if (value === undefined) {
    return Failure();
  }
  
  if (!isInteger(value)) {
    return Failure();
  }

  me.index += 1;
  return value;
};

export const Source_readString = (me: Source): Failure | string => {
  const value = me.buffer[me.index];
  if (value === undefined) {
    return Failure();
  }
  
  if (typeof value !== "string") {
    return Failure();
  }

  me.index += 1;
  return value;
};

export const Source_readBoolean = (me: Source): Failure | boolean => {
  const value = me.buffer[me.index];
  if (value === undefined) {
    return Failure();
  }

  if (typeof value !== "boolean") {
    return Failure();
  }

  me.index += 1;
  return value;
};

export const Source_readValue = <Value>(me: Source, deserialize: Deserialize<Value>): Failure | Value => {
  return deserialize.read(me);
};

export const Source_readArray = <Item>(me: Source, deserialize: Deserialize<Item>): Failure | Item[] => {
  const itemsCount = Source_readInteger(me);
  if (itemsCount === Failure() || itemsCount < 0) {
    return Failure();
  }
  
  const array = [];
  let itemsRead = 0;

  while (itemsRead < itemsCount) {
    const item = Source_readValue(me, deserialize);
    if (item === Failure()) {
      return Failure();
    }

    array.push(item);
    itemsRead += 1;
  }

  return array;
};

export interface Deserialize<Value> {
  read(source: Source): Value | Failure;
  deserialize(string: string): Value | Error;
}

export const Deserialize_implement = <Value>({
  read,
}: {
  read(source: Source): Value | Failure,
}): Deserialize<Value> => {
  return {
    read,

    deserialize(string) {
      let buffer;
      
      try {
        buffer = JSON.parse(string);
      } catch (cause) {
        return new Error("", { cause });
      }

      if (!Array.isArray(buffer)) {
        return new Error("");
      }

      const source = Source_create(buffer);
      const value = read(source);
      if (value === Failure()) {
        return new Error("");
      }
      if (source.index !== source.buffer.length) {
        return new Error("");
      }

      return value;
    },
  };
};
