import { Branded, UserName } from "../mod.ts";

const BRAND = Symbol();

const VARIANT_STRING_LENGTH = 0;

export type UserNameCreateError = Branded<typeof BRAND, {
  readonly type: typeof VARIANT_STRING_LENGTH,
  readonly string: string,
  readonly minimumLength: number,
  readonly maximumLength: number,
}>;

export const StringLength = (string: string): UserNameCreateError => {
  return Branded(BRAND, {
    type: VARIANT_STRING_LENGTH,
    string,
    minimumLength: UserName.MINIMUM_LENGTH,
    maximumLength: UserName.MAXIMUM_LENGTH,
  });
};