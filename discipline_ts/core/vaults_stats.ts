import { Integer, Unique } from "../x.ts";

const BRAND = Symbol();

type RawVaultsStats = {
  vaultsNumber: Integer,
  maximumVaultsNumber: Integer,
};

export type VaultsStats = Unique<typeof BRAND, "VaultsStats", RawVaultsStats>;

const construct = (
  vaultsNumber: Integer,
  maximumVaultsNumber: Integer,
): VaultsStats => {
  return {
    vaultsNumber,
    maximumVaultsNumber,
  } satisfies RawVaultsStats as VaultsStats;
};

export const create = (maximumVaultsNumber: Integer): VaultsStats => {
  return construct(
    Integer.uncheckedFromNumber(0), 
    maximumVaultsNumber,
  );
};

export const getVaultsNumber = (it: VaultsStats): Integer => {
  return it.vaultsNumber;
};

export const getMaximumVaultsNumber = (it: VaultsStats): Integer => {
  return it.maximumVaultsNumber;
};

export const VaultsStats = {
  create,
  getVaultsNumber,
  getMaximumVaultsNumber,
};