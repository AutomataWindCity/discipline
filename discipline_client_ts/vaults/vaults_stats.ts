import { Branded } from "../x.ts";

const BRAND = Symbol();

export type VaultsStats = Branded<typeof BRAND, {
  vaultsNumber: number,
  maximumVaultsNumber: number,
}>;

export const construct = (
  vaultsNumber: number,
  maximumVaultsNumber: number,
): VaultsStats => {
  return Branded(BRAND, {
    vaultsNumber,
    maximumVaultsNumber,
  });
};

export const create = (maximumVaultsNumber: number): VaultsStats => {
  return construct(0, maximumVaultsNumber);
};

export const VaultsStats = {
  create,
};