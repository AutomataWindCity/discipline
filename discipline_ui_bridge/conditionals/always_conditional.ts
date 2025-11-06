import { Branded, EmptyObject } from "../mod.ts";

const BRAND = Symbol();

export type AlwaysConditional = Branded<typeof BRAND, EmptyObject>;

const construct = (): AlwaysConditional => {
  return Branded(BRAND, {});
};

export const create = (): AlwaysConditional => {
  return construct();
};

export const isEffective = (_me: AlwaysConditional): boolean => {
  return true;
};