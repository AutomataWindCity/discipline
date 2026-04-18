import { Float, Integer, Nullable } from "../../x.ts";
import { Name, Scalar } from "./mod.ts"

export interface NamedWriteDestination<Destination> {
  writeNull: (destination: Destination, name: Name) => void;
  writeInteger: (destination: Destination, name: Name, integer: Integer) => void;
  writeReal: (destination: Destination, name: Name, real: Float) => void;
  writeString: (destination: Destination, name: Name, string: string) => void;
  writeBoolean: (destination: Destination, name: Name, boolean: boolean) => void;
  writeScalarValue: <Value>(destination: Destination, name: Name, value: Value, descriptor: Scalar<Value>) => void;
  writeNullableScalarValue: <Value>(destination: Destination, name: Name, value: Nullable<Value>, descriptor: Scalar<Value>) => void;
  writeCompoundValue: <Value, Names>(destination: Destination, schema: Names, value: Value, valueWrite: NamedWrite<Value, Names>) => void;

  // fn write_optional() {}

  // fn as_namef_write_null_destination(&mut self) -> &mut impl NamedWriteNullDestination;
}

export const NamedWriteDestination = {
  implement: <Destination>(initializer: NamedWriteDestination<Destination>): NamedWriteDestination<Destination> => {
    return initializer;
  },
  
  autoImplementForWritable: <Destination>({

  }: { 
    write: (destination: Destination, slice: string) => void, 
  }): NamedWriteDestination<Destination> => {
    // TODO: implement
  },
};

/**
 * Serializes a compound value to be stored in SQLite.
 * 
 * Implemented by compound types that are to be stored in SQLite.
 * 
 * Fields whose value is null don't have to be written.
 */
export interface NamedWrite<Value, Names> {
  readonly write: <Destination>(
    value: Value, 
    names: Names, 
    destination: Destination,
    destinationImpl: NamedWriteDestination<Destination>,
  ) => void;
}

export const NamedWrite = {
  implement: <Value, Names>(initializer: NamedWrite<Value, Names>): NamedWrite<Value, Names> => {
    return initializer;
  },
};