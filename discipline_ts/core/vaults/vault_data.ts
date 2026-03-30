import { Branded, FailureCode, TextualError } from "../x.ts";

const brand = Symbol();

export type VaultData = Branded<typeof brand, string>;

export const MINIMUM_LENGTH = 1;
export const MAXIMUM_LENGTH = 500;

export const construct = (string: string): VaultData => {
  return Branded(brand, string);
};

export const create = (string: string): VaultData | Error => {
  if (
    string.length < MINIMUM_LENGTH 
    || 
    string.length > MAXIMUM_LENGTH
  ) {
    return new Error(`Creating VaultData from string: String violates length invariants. Minimum length: ${MINIMUM_LENGTH}. Maximum length: ${MAXIMUM_LENGTH}. Provided string's length: ${string.length}. Provided string: ${string}`);
  }

  return construct(string);
};

export const createOrErrorCode = (string: string, textualError: TextualError): VaultData | FailureCode => {};

export const toString = (it: VaultData): string => {
  return it;
};

export const isEqualTo = (it: VaultData, rhs: VaultData): boolean => {
  return it === rhs;
};

export const VaultData = {
  MINIMUM_LENGTH,
  MAXIMUM_LENGTH,
  create,
  createOrErrorCode,
  construct,
  toString,
  isEqualTo,
};