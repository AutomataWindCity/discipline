import { Float, Integer, Nullable } from "../../x.ts";
import { Scalar } from "./mod.ts";

export interface OrderedWriteDestination<Destination> {
  writeNull: (destination: Destination) => void;
  writeInteger: (destination: Destination, integer: Nullable<Integer>) => void;
  writeNullableInteger: (destination: Destination, integer: Nullable<Integer>) => void;
  writeReal: (destination: Destination, real: Float) => void;
  writeNullableReal: (destination: Destination, real: Nullable<Float>) => void;
  writeString: (destination: Destination, string: string) => void;
  writeNullableString: (destination: Destination, string: Nullable<string>) => void;
  writeBoolean: (destination: Destination, boolean: boolean) => void;
  writeNullanleBoolean: (destination: Destination, boolean: Nullable<boolean>) => void;
  writeScalarValue: <Value>(destination: Destination, value: Value, adapter: Scalar<Value>) => void;
  writeNullableScalarValue: <Value>(destination: Destination, value: Nullable<Value>, adapter: Scalar<Value>) => void;
  writeCompoundValue: <Value>(destination: Destination, value: Value, orderedWrite: OrderedWrite<Value>) => void;
  // fn as_ordered_write_null_destination(&mut self) -> &mut impl OrderedWriteNullDestination;
}

export interface OrderedWrite<Value> {
  readonly write: <Destination>(
    value: Value, 
    destination: Destination, 
    destinationImpl: OrderedWriteDestination<Destination>,
  ) => void;
}

export const OrderedWrite = {
  implement: <Value>(initializer: OrderedWrite<Value>): OrderedWrite<Value> => {
    return initializer;
  },
};