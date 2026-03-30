import { Branded, VaultName, VaultData, Countdown, Instant, Duration } from "../x.ts";

const brand = Symbol();

export type Vault = Branded<typeof brand, {
  name: VaultName,
  data: VaultData,
  protection: Countdown,
}>;

export const MAXIMUM_PROTECTION_DURATION = Duration.fromMillisecondsOrThrow(1000 * 60 * 60 * 24 * 7);

export const construct = (
  name: VaultName,
  data: VaultData,
  protection: Countdown,
): Vault => {
  return Branded(brand, {
    name,
    data,
    protection,
  });
};

export const create = (
  name: VaultName,
  data: VaultData,
  protection: Countdown,
): Vault => {
  return construct(name, data, protection);
};

export const getName = (it: Vault): VaultName => {
  return it.name;
};

export const getData = (it: Vault): VaultData => {
  return it.data;
};

export const getProtection = (it: Vault): Countdown => {
  return it.protection;
};

export const isProtected = (it: Vault, now: Instant): boolean => {
  return Countdown.isRunning(it.protection, now);
};

export const extendProtectionByOrSetToMaxSafeValue = (
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

    if (Duration.isLongerThan(Duration.plusOrMax(factor, remaining), maximum)) {
      factor = Duration.minusOrZero(maximum, remaining);
    }

    Countdown.extendByOrSetToMax(it.protection, factor);
  }
};

export const Vault = {
  MAXIMUM_PROTECTION_DURATION,
  create,
  construct,
  getName,
  getData,
  getProtection,
  isProtected,
  extendProtectionByOrSetToMaxSafeValue,
};