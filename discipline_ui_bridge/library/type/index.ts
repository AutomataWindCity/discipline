import { TUnique } from "../../mod.ts"

const enum ReflectionType {
  Number,
  Boolean,
  Array,
  Struct,
  Set,
  Map,
  Wrapper,
};

type NumberReflection = TUnique<"Number", {
  readonly type: ReflectionType.Number,
}>;

type BooleanReflection = TUnique<"Boolean", {
  readonly type: ReflectionType.Boolean,
}>;

type ArrayReflection<ArrayItemType> = TUnique<"Array", {
  readonly type: ReflectionType.Array,
  readonly itemType: AnyTypeReflection,
}>;

type WrapperReflection<Inner> = TUnique<"WrapperType", {
  readonly type: ReflectionType.Wrapper,
  readonly name: string,
  readonly innerType: AnyTypeReflection,
}>;

type EnumVariantReflection = {
  readonly 
}

type StructDataFieldReflection<Struct, Field> = TUnique<"StructDataField", {
  readonly type: AnyTypeReflection,
  readonly getter: (struct: Struct) => Field,
}>;

type StructVirtualFieldReflection<Struct, Field> = TUnique<"StructVirtualField", {
  readonly type: AnyTypeReflection,
  readonly getter: (struct: Struct) => Field,
}>;

type StructReflection<
  DataFields extends Record<string, unknown>, 
  VirtualFields extends Record<string, unknown>, 
> = TUnique<"Struct", {
  readonly type: ReflectionType.Struct,
  readonly dataFieldTypes: Map<string, StructDataFieldReflection<unknown, unknown>>,
  readonly virtualFieldTypes: Map<string, StructVirtualFieldReflection<unknown, unknown>>,
}>;


export type AnyTypeReflection = (
  | NumberReflection
  | BooleanReflection
  | ArrayReflection
  | StructReflection
  | WrapperReflection
);