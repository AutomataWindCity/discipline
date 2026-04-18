import { TextualError, Unique } from "../x.ts";

const BRAND = Symbol();

export type VaultName = Unique<typeof BRAND, "VaultData", string>;

const MINIMUM_LENGTH = 1;
const MAXIMUM_LENGTH = 300;

const construct = (string: string): VaultName => {
  return string as VaultName;
};

const wrapOrThrow = (string: string): VaultName => {
  if (
    string.length < MINIMUM_LENGTH 
    || 
    string.length > MAXIMUM_LENGTH
  ) {
    const textualError = TextualError.create("Creating VaultName from string");
    TextualError.addMessage(textualError, "String violates length invariants. ");
    TextualError.addNumberAttachment(textualError, "Minimum length", MINIMUM_LENGTH);
    TextualError.addNumberAttachment(textualError, "Maximum length", MAXIMUM_LENGTH);
    TextualError.addNumberAttachment(textualError, "Provided string's length", string.length);
    TextualError.addStringAttachment(textualError, "Provided string", string);
    throw TextualError.toJsError(textualError);
  }

  return construct(string);
};

const getValue = (it: VaultName): string => {
  return it;
};

const isEqualTo = (it: VaultName, rhs: VaultName): boolean => {
  return it === rhs;
};

export const VaultName = {
  MINIMUM_LENGTH,
  MAXIMUM_LENGTH,
  wrapOrThrow,
  construct,
  isEqualTo,
  getValue,
};