import { Countdown, Branded } from "../x.ts";

const BRAND = Symbol();

export type CountdownRule = Branded<typeof BRAND, {
  readonly isEnabled: boolean,
  readonly countdown: Countdown,
}>;

export const construct = (isEnabled: boolean, countdown: Countdown): CountdownRule => {
  return Branded(BRAND, {
    countdown,
    isEnabled,
  });
};

export const create = (countdown: Countdown): CountdownRule => {
  return construct(false, countdown);
};

export const getIsEnabled = (it: CountdownRule): boolean => {
  return it.isEnabled;
};

export const getCountdown = (it: CountdownRule): Countdown => {
  return it.countdown;
};

export const CountdownRule = {
  construct,
  create,
  getIsEnabled,
  getCountdown,
};