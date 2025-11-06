import { Branded, Tried, UserNameCreateError } from "../mod.ts";

const BRAND = Symbol();

export type UserName = Branded<typeof BRAND, string>;

export const MINIMUM_LENGTH = 1;
export const MAXIMUM_LENGTH = 3;

export const create = (string: string): Tried.Tried<UserName, UserNameCreateError.UserNameCreateError> => {
  if (
    string.length < MINIMUM_LENGTH
    ||
    string.length > MAXIMUM_LENGTH
  ) {
    return Tried.Failure(UserNameCreateError.StringLength(string));
  }

  return Tried.Success(Branded(BRAND, string));
};