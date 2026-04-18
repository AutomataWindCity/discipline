import { VaultName, VaultData, Countdown, Instant, Duration, Unique } from "../x.ts";

const BRAND = Symbol();

type RawVault = {
  name: VaultName,
  data: VaultData,
  protection: Countdown,
};

export type Vault = Unique<typeof BRAND, "Vault", RawVault>;

const construct = (
  name: VaultName,
  data: VaultData,
  protection: Countdown,
): Vault => {
  return {
    name,
    data,
    protection,
  } satisfies RawVault as Vault;
};

const reconstruct = construct;

const create = construct;

const getName = (it: Vault): VaultName => {
  return it.name;
};

const getData = (it: Vault): VaultData => {
  return it.data;
};

const getProtection = (it: Vault): Countdown => {
  return it.protection;
};

const isProtected = (it: Vault, now: Instant): boolean => {
  return Countdown.isRunning(it.protection, now);
};

const extendProtectionByOrSetToMaxSafeValue = (
  it: Vault,
  now: Instant,
  factor: Duration,
): void => {
  if (Countdown.isFinished(it.protection, now)) {
    it.protection = Countdown.create(
      now,
      Duration.min(MAXIMUM_PROTECTION_DURATION, factor),
    );
  } else {
    const remaining = Countdown.getRemainingTimeOrZero(it.protection, now);
    const maximum = MAXIMUM_PROTECTION_DURATION;

    if (Duration.isLongerThan(Duration.saturatingAdd(factor, remaining), maximum)) {
      factor = Duration.saturatingSub(maximum, remaining);
    }

    Countdown.saturatingIncrementDuration(it.protection, factor);
  }
};

export const Vault = {
  reconstruct,
  create,
  construct,
  getName,
  getData,
  getProtection,
  isProtected,
  extendProtectionByOrSetToMaxSafeValue,
};