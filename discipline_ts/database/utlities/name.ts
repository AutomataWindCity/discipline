import { TextualError, Unique } from "../../x.ts";

const BRAND = Symbol();

export type Name = Unique<typeof BRAND, "Name", string>;

/**
 * @throws {TextualError}
 */
const createOrThrow = (string: string): Name => {
  // TODO: Throw if the column name is invalid.
  return string as Name;
};

const toString = (it: Name): string => {
  return it;
};

export const Name = {
  // create: (string: string, textualError: TextualError): Column | FailureCode => {
  //   if (string.length === 0) {
  //     TextualError.changeContext(textualError, "Creating a 'ColumnName' from string");
  //     TextualError.addMessage(textualError, "String is empty");
  //     return FAILURE;
  //   }

  //   if (string.length > 100) {
  //     TextualError.changeContext(textualError, "Creating a 'ColumnName' from string");
  //     TextualError.addMessage(textualError, "String is too long");
  //     TextualError.addStringAttachment(textualError, "String", string);
  //     return FAILURE;
  //   }

  //   if (string.startsWith(/\d/ug, ))
  // },

  createOrThrow,
  toString,
};