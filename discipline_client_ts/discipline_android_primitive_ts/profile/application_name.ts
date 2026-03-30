import { Nominal, TextualError, Tried } from "../x.ts";

const BRAND = Symbol();

export type ApplicationName = Nominal<typeof BRAND, string>;

export const MINIMUM_LENGTH = 1;
export const MAXIMUM_LENGTH = 30;

export const construct = (string: string): ApplicationName => {
  return Nominal.create(BRAND, string);
};

export const create = (string: string): Tried<ApplicationName, TextualError> => {
  if (string.length < MINIMUM_LENGTH) {
    const it = TextualError.create("Creating an ApplicationName from string");
    TextualError.addMessage(it, "String's length is less than the minimum valid length");
    TextualError.addStringAttachment(it, "String", string);
    TextualError.addNumberAttachment(it, "String length", string.length);
    TextualError.addNumberAttachment(it, "Minimum valid length", MINIMUM_LENGTH);
    return Tried.Failure(it);
  }

  if (string.length > MAXIMUM_LENGTH) {
    const it = TextualError.create("Creating an ApplicationName from string");
    TextualError.addMessage(it, "String's length is greater than the maximum allowed length");
    TextualError.addStringAttachment(it, "String", string);
    TextualError.addNumberAttachment(it, "String length", string.length);
    TextualError.addNumberAttachment(it, "Maximum valid length", MAXIMUM_LENGTH);
    return Tried.Failure(it);
  }

  return Tried.Success(construct(string));
};

export const ApplicationName = {
  create,
};