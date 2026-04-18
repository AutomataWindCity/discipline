import { Integer, Float, Nullable } from "../../x.ts"

export interface ScalarWriteDestination<Destination> {
  readonly writeNull: (destination: Destination) => void;
  readonly writeInteger: (destination: Destination, value: Integer) => void;
  readonly writeNullableInteger: (destination: Destination, value: Nullable<Integer>) => void;
  readonly writeReal: (destination: Destination, value: Float) => void;
  readonly writeNullableReal: (destination: Destination, value: Nullable<Float>) => void;
  readonly writeBoolean: (destination: Destination, value: boolean) => void;
  readonly writeNullableBoolean: (destination: Destination, value: Nullable<boolean>) => void;
  readonly writeString: (destination: Destination, value: string) => void;
  readonly writeNullableString: (destination: Destination, value: Nullable<string>) => void;
  readonly writeScalarValue: <Value>(destination: Destination, value: Value, write: ScalarWrite<Value>) => void;
  readonly writeNullable: <Value>(destination: Destination, value: Nullable<Value>, write: ScalarWrite<Value>) => void;
}

export interface ScalarWrite<Value> {
  readonly write: <Destination>(
    value: Value,
    destination: Destination,
    destinationImpl: ScalarWriteDestination<Destination>,
  ) => void;
}

export const ScalarWrite =  {
  implement: <Value>(initializer: ScalarWrite<Value>): ScalarWrite<Value> => {
    return initializer;
  },
};