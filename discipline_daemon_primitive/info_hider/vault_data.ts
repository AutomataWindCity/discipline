import { Branded, Tried } from "../../discipline_ui_bridge/mod.ts";
import * as CreateError from "./vault_data_create_error.ts";

const BRAND = Symbol();

export const minimumLength = 1;
export const maximumLength = 500;

export type VaultData = Branded<typeof BRAND, string>;

export const create = (string: string): Tried.Tried<VaultData, CreateError.Error> => {
  if (
    string.length < minimumLength 
    || 
    string.length > maximumLength
  ) {
    return Tried.Failure(CreateError.LengthViolation(string));
  }

  return Tried.Success(Branded(BRAND, string));
};