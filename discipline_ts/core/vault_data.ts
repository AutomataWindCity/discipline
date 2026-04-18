import { TextualError, Unique } from "../x.ts";

const BRAND = Symbol();

export type VaultData = Unique<typeof BRAND, "VaultData", string>;

const MINIMUM_LENGTH = 1;
const MAXIMUM_LENGTH = 500;

const construct = (string: string): VaultData => {
  return string as VaultData;
};

const wrapOrThrow = (string: string): VaultData => {
  if (
    string.length < MINIMUM_LENGTH 
    || 
    string.length > MAXIMUM_LENGTH
  ) {
    const textualError = TextualError.create("Creating VaultData from string");
    TextualError.addMessage(textualError, "String violates length invariants. ");
    TextualError.addNumberAttachment(textualError, "Minimum length", MINIMUM_LENGTH);
    TextualError.addNumberAttachment(textualError, "Maximum length", MAXIMUM_LENGTH);
    TextualError.addNumberAttachment(textualError, "Provided string's length", string.length);
    TextualError.addStringAttachment(textualError, "Provided string", string);
    throw TextualError.toJsError(textualError);
  }

  return construct(string);
};

const getValue = (it: VaultData): string => {
  return it;
};

const isEqualTo = (it: VaultData, rhs: VaultData): boolean => {
  return it === rhs;
};

export const VaultData = {
  MINIMUM_LENGTH,
  MAXIMUM_LENGTH,
  wrapOrThrow,
  construct,
  isEqualTo,
  getValue,
};