import { Branded } from "../x.ts";

const SUCCESS_BRAND = Symbol();
const FAILURE_BRAND = Symbol();

const SUCCESS_TYPE = 0;
const FAILURE_TYPE = 1;

export type Success<Value> = Branded<typeof SUCCESS_BRAND, {
  readonly type: typeof SUCCESS_TYPE,
  readonly value: Value,
}>;

export type Failure<Error> = Branded<typeof FAILURE_BRAND, {
  readonly type: typeof FAILURE_TYPE,
  readonly error: Error,
}>;

export type Tried<Value, Error> = (
  | Success<Value> 
  | Failure<Error>
);

export type TriedMatchCases<Value, Error, Return> = {
  readonly Success: (value: Value) => Return,
  readonly Failure: (error: Error) => Return,
};

export const Tried = {
  Success: <Value>(value: Value): Success<Value> => {
    return Branded(SUCCESS_BRAND, {
      type: SUCCESS_TYPE,
      value,
    });
  },
  
  Failure: <Error>(error: Error): Failure<Error> => {
    return Branded(FAILURE_BRAND, {
      type: FAILURE_TYPE,
      error,
    })
  },
  
  isSuccess: <Value>(me: Tried<Value, unknown>): me is Success<Value> => {
    return me.type === SUCCESS_TYPE;
  },
  
  isFailure: <Error>(me: Tried<unknown, Error>): me is Failure<Error> => {
    return me.type === FAILURE_TYPE;
  },
  
  map: <Value, Error>(
    me: Tried<Value, Error>,
    fn: (value: Value) => Value,
  ): Tried<Value, Error> => {
    return Tried.isSuccess(me) ? Tried.Success(fn(me.value)) : me;
  },
  
  mapError: <Value, Error>(
    me: Tried<Value, Error>,
    fn: (error: Error) => Error,
  ): Tried<Value, Error> => {
    return Tried.isFailure(me) ? Tried.Failure(fn(me.error)) : me;
  },

  match: <Value, Error, Return>(
    me: Tried<Value, Error>,
    cases: TriedMatchCases<Value, Error, Return>,
  ): Return => {
    switch (me.type) {
      case SUCCESS_TYPE: return cases.Success(Tried.value(me));
      case FAILURE_TYPE: return cases.Failure(Tried.error(me));
    }
  },
  
  value: <Value>(me: Success<Value>): Value => {
    return me.value;
  },
  
  error: <Error>(me: Failure<Error>): Error => {
    return me.error;
  },

  unwrap: <Value, Error>(it: Tried<Value, Error>): Value => {
    if (Tried.isSuccess(it)) {
      return Tried.value(it);
    } else {
      throw new Error(`Calling 'unwrap' on 'Failure'. Error: ${Tried.error(it)}`);
    }
  },
};