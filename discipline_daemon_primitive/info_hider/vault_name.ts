import { Branded, Tried } from "../../discipline_ui_bridge/mod.ts";
import * as CreateError from "./vault_name_create_error.ts";

const BRAND = Symbol();

export const minimumLength = 1;
export const maximumLength = 300;

export type VaultName = Branded<typeof BRAND, string>;

export const create = (string: string): Tried.Tried<VaultName, CreateError.Error> => {
  if (
    string.length < minimumLength 
    || 
    string.length > maximumLength
  ) {
    return Tried.Failure(CreateError.LengthViolation(string));
  }

  return Tried.Success(Branded(BRAND, string));
};