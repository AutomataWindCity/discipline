import * as SQLite from "jsr:@db/sqlite";
import { Branded, Serialization, Tried } from "../x.ts";

type GetRowReturn = {
  readonly data?: null | string | number | Uint8Array
}

const SQLiteStringLiteral_create = (string: string) => {
  return `"${string.replaceAll(/\"/g, "\"\"")}"`;
};

const BRAND = Symbol();

export type DatabaseT<T> = Branded<typeof BRAND, {
  readonly connection: SQLite.Database,
  readonly serialize: Serialization.Serialize<T>,
  readonly deserialize: Serialization.Deserialize<T>,
  readonly initial: () => T,
}>;

export const open = <T>(
  databaseDirectory: string,
  serialize: Serialization.Serialize<T>,
  deserialize: Serialization.Deserialize<T>,
  initial: () => T,
): DatabaseT<T> | Error => {
  const me: DatabaseT<T> = Branded(BRAND, {
    connection: new SQLite.Database(databaseDirectory),
    serialize,
    deserialize,
    initial,
  });

  const maybeError = initializeSchema(me);
  if (maybeError instanceof Error) {
    return new Error("Database: Opening database", { cause: maybeError });
  }

  return me;
};

const generateInitializeSchema = (): string => {
  return `
    -- Enable Write-Ahead Logging (WAL) mode for better durability
    PRAGMA journal_mode = WAL;
    -- Or FULL for maximum safety
    PRAGMA synchronous = FULL; 

    CREATE TABLE IF NOT EXISTS Main (
      id INTEGER PRIMARY KEY,
      data TEXT NOT NULL
    ) STRICT, WITHOUT ROWID;
  `;
};

const generateInitializeData = (initialData: string): string => {
  return `
    INSERT INTO Main (id, data) VALUES (
      0,
      ${SQLiteStringLiteral_create(initialData)}
    );
  `;
};

const generateSelectData = () => {
  // TODO
  return `
    SELECT data from Main WHERE id = 0;
  `;
};

const generateUpdateData = <T>(newData: string) => {
  return `
    UPDATE Main 
      SET data = ${SQLiteStringLiteral_create(newData)}
      WHERE id = 0;
  `;
};

const initializeSchema = <T>(me: DatabaseT<T>) => {
  const code = generateInitializeSchema();
  try {
    me.connection.exec(code);
  } catch (cause) {
    return new Error("Database: Initializing database schema: A SQLite error occured", {
      cause,
    });
  }
};

const initializeData = <T>(me: DatabaseT<T>): Tried.Tried<T, null> => {
  const initialData = me.initial();
  const initialDataSerialized = me.serialize.serialize(initialData);
  if (Tried.isFailure(initialDataSerialized)) {
    return Tried.Failure(null);
  }

  const code = generateInitializeData(Tried.value(initialDataSerialized));

  try {
    me.connection.exec(code);
  } catch (error) {
    return Tried.Failure(null);
  }

  return Tried.Success(initialData);
};

export const get = <T>(me: DatabaseT<T>) => {
  let statement: SQLite.Statement;
  try {
    statement = me.connection.prepare(generateSelectData());
  } catch (error) {
    return Tried.Failure(null);
  }

  let row: GetRowReturn | undefined;
  try {
    row = statement.get<GetRowReturn>();
  } catch (error) {
    return Tried.Failure(null);
  }

  if (row === undefined) { 
    return initializeData(me);
  }

  if (typeof row.data !== "string") {
    return Tried.Failure(null);
  }

  return me.deserialize.deserialize(row.data);
};

export const set = <T>(me: DatabaseT<T>, newData: T) => {
  const newDataSerialized = me.serialize.serialize(newData);
  if (Tried.isFailure(newDataSerialized)) {
    return newDataSerialized;
  }

  const code = generateUpdateData(Tried.value(newDataSerialized));
  try {
    me.connection.exec(code);
  } catch (error) {
    return Tried.Failure(null);
  }

  return Tried.Success(null);
};