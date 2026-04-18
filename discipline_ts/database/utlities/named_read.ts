import { Float, Integer, Nullable } from "../../x.ts";
import { Name, Scalar } from "./mod.ts";

export interface NamedReadSource<Source> {
  readonly readIntegerOrThrow: (source: Source, name: Name) => Integer;
  readonly readNullableIntegerOrThrow: (source: Source, name: Name) => Nullable<Integer>;
  readonly readRealOrThrow: (source: Source, name: Name) => Float;
  readonly readNullableRealOrThrow: (source: Source, name: Name) => Nullable<Float>;
  readonly readStringOrThrow: (source: Source, name: Name) => string,
  readonly readNullableStringOrThrow: (source: Source, name: Name) => Nullable<string>,
  readonly readBooleanOrThrow: (source: Source, name: Name) => boolean,
  readonly readNullableBooleanOrThrow: (source: Source, name: Name) => Nullable<boolean>,
  readonly readScalarValueOrThrow: <Value>(source: Source, name: Name, adapter: Scalar<Value>) => Value;
  readonly readNullableScalarValueOrThrow: <Value>(source: Source, name: Name, adapter: Scalar<Value>) => Nullable<Value>;
  readonly readCompoundValueOrThrow: <Value, Names>(source: Source, names: Names, namedRead: NamedRead<Value, Names>) => Value;
}

export interface NamedRead<Value, Names> {
  readonly readOrThrow: <Source>(names: Names, source: Source, sourceImpl: NamedReadSource<Source>) => Value;
}

export const NamedRead = {
  implement: <Value, Names>(initializer: NamedRead<Value, Names>): NamedRead<Value, Names> => {
    return initializer;
  },
};