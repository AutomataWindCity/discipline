import { Branded, FailureCode, TextualError } from "../x.ts";

const brand = Symbol();

export type VaultName = Branded<typeof brand, string>;

export const MINIMUM_LENGTH = 1;
export const MAXIMUM_LENGTH = 300;

export const construct = (string: string): VaultName => {
  return Branded(brand, string);
};

export const create = (string: string): VaultName | Error => {
  if (
    string.length < MINIMUM_LENGTH 
    || 
    string.length > MAXIMUM_LENGTH
  ) {
    return new Error(`Creating VaultName from string: String violates length invariants. Minimum length: ${MINIMUM_LENGTH}. Maximum length: ${MAXIMUM_LENGTH}. Provided string's length: ${string.length}. Provided string: ${string}`);
  }

  return construct(string);
};

export const createOrErrorCode = (string: string, textualError: TextualError): VaultName | FailureCode => {};

export const toString = (it: VaultName): string => {
  return it;
};

export const isEqualTo = (it: VaultName, rhs: VaultName): boolean => {
  return it === rhs;
};

export const VaultName = {
  MINIMUM_LENGTH,
  MAXIMUM_LENGTH,
  create,
  construct,
  toString,
  isEqualTo,
  createOrErrorCode,
};