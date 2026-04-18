import { Countdown, Branded, Unique } from "../../x.ts";

const BRAND = Symbol();

type RawAlwaysRule = {
  readonly enabled: boolean,
  readonly condition: Countdown,
};

export type AlwaysRule = Unique<typeof BRAND, "AlwaysRule", RawAlwaysRule>;

const construct = (enabled: boolean, condition: Countdown): AlwaysRule => {
  return {
    condition: condition,
    enabled: enabled,
  };
};

export const create = (countdown: Countdown): AlwaysRule => {
  return construct(false, countdown);
};

export const getIsEnabled = (it: AlwaysRule): boolean => {
  return it.enabled;
};

export const getCountdown = (it: AlwaysRule): Countdown => {
  return it.condition;
};

export const AlwaysRule = {
  construct,
  create,
  getIsEnabled,
  getCountdown,
};