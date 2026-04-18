import { Integer, Float, Nullable, TextualError } from "../../x.ts"

export interface ScalarReadSource<Source> {
  readonly isNull: (source: Source) => boolean;
  /**
   * @throws {TextualError} 
   */
  readonly readNullOrThrow: (source: Source) => null;
  /**
   * @throws {TextualError} 
   */
  readonly readIntegerOrThrow: (source: Source) => Integer;
  /**
   * @throws {TextualError} 
   */
  readonly readNullableIntegerOrThrow: (source: Source) => Nullable<Integer>;
  /**
   * @throws {TextualError} 
   */
  readonly readRealOrThrow: (source: Source) => Float;
  /**
   * @throws {TextualError} 
   */
  readonly readNullableRealOrThrow: (source: Source) => Nullable<Float>;
  /**
   * @throws {TextualError} 
   */
  readonly readBooleanOrThrow: (source: Source) => boolean;
  /**
   * @throws {TextualError} 
   */
  readonly readNullableBooleanOrThrow: (source: Source) => Nullable<boolean>;
  /**
   * @throws {TextualError} 
   */
  readonly readStringOrThrow: (source: Source) => string;
  /**
   * @throws {TextualError} 
   */
  readonly readNullableStringOrThrow: (source: Source) => Nullable<string>;
  /**
   * @throws {TextualError} 
   */
  readonly readOrThrow: <Value>(it: Source, read: ScalarRead<Value>) => Value;
  /**
   * @throws {TextualError} 
   */
  readonly readNullableOrThrow: <Value>(it: Source, read: ScalarRead<Value>) => Nullable<Value>;
}

export interface ScalarRead<Value> {
  /**
   * @throws {TextualError} 
   */
  readonly readOrThrow: <Source>(
    source: Source, 
    sourceImpl: ScalarReadSource<Source>,
  ) => Value;
}

export const ScalarRead = {
  implement: <Value>(initializer: ScalarRead<Value>): ScalarRead<Value> => {
    return initializer;
  },
};