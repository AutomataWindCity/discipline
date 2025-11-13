// export type JsonSerializer

import { Branded, Tried } from "../discipline_ui_bridge/mod.ts";

const BRAND = Symbol();

export type Storage<Value> = Branded<typeof BRAND, {
  readonly path: string,
  readonly serialize: (value: Value) => string,
  readonly deserialize: (text: string) => Tried.Tried<Value, Error>,
  readonly fallback: () => Value,
}>;

export type Initializer<Value> = {
  readonly path: string,
  readonly serialize: (value: Value) => string,
  readonly deserialize: (text: string) => Tried.Tried<Value, Error>,
  readonly fallback: () => Value,
};

export const create = <Value>(initializer: Initializer<Value>): Storage<Value> => {
  return Branded(BRAND, initializer);
};

export const write = async <Value>(me: Storage<Value>, value: Value): Promise<Tried.Tried<null, Error>> => {
  const serialized = me.serialize(value);

  try {
    await Deno.writeTextFile(me.path, serialized, {
      create: true,
    });
  } catch (error) {
    return Tried.Failure(new Error(`Writing value of storage: Underlying FileSystem call failed. Storage path: ${me.path}`, {
      cause: error
    }));
  }

  return Tried.Success(null);
};

export const read = async <Value>(me: Storage<Value>): Promise<Tried.Tried<Value, Error>> => {
  let text: string;
  
  try {
    text = await Deno.readTextFile(me.path);
  } catch (error) {
    if (error instanceof Deno.errors.NotFound) {
      const value = me.fallback();
      const maybeError = await write(me, value);
      if (Tried.isFailure(maybeError)) {
        return Tried.Failure(new Error(`Reading value of Storage: Storge is not initialized: Initializing Storage`, {
          cause: Tried.error(maybeError),
        }));
      } else {
        return Tried.Success(value);
      }
    }

    return Tried.Failure(new Error(`Reading value of Storage: Underlying FileSystem call failed. Storage path: ${me.path}`, {
      cause: error
    }));
  }

  return Tried.mapError(
    me.deserialize(text),
    error => 
      new Error(`Reading value of Storage: Value deserialization failed. Storage path: ${me.path}`, {
        cause: error,
      })
  );
};