import { TypeId, Unique } from "../mod.ts";

export type Success<Value> = Unique<TypeId.TriedSuccess> & {
  readonly value: Value
};

export type Failure<Error> = Unique<TypeId.TriedFailure> & {
  readonly error: Error
};

export type Tried<Value, Error> = Success<Value> | Failure<Error>;

export const Success = <Value>(value: Value): Success<Value> => {
  return {
    typeId: TypeId.TriedSuccess,
    value,
  };
};

export const Failure = <Error>(error: Error): Failure<Error> => {
  return {
    typeId: TypeId.TriedFailure,
    error,
  };
};

export const isSuccess = <Value>(me: Tried<Value, unknown>): me is Success<Value> => {
  return me.typeId === TypeId.TriedSuccess;
};

export const isFailure = <Error>(me: Tried<unknown, Error>): me is Failure<Error> => {
  return me.typeId === TypeId.TriedFailure;
};

export const map = <Value, Error>(
  me: Tried<Value, Error>,
  fn: (value: Value) => Value,
): Tried<Value, Error> => {
  return isSuccess(me) ? Success(fn(me.value)) : me;
};

export const mapError = <Value, Error>(
  me: Tried<Value, Error>,
  fn: (error: Error) => Error,
): Tried<Value, Error> => {
  return isFailure(me) ? Failure(fn(me.error)) : me;
};

export const value = <Value>(me: Success<Value>): Value => {
  return me.value;
};

export const error = <Error>(me: Failure<Error>): Error => {
  return me.error;
};