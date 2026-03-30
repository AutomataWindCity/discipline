import { CountdownRule, Nominal } from "../x.ts";

const BRAND = Symbol();

export type CountdownRules = Nominal<typeof BRAND, CountdownRule[]>;

export const construct = (rules: CountdownRule[]): CountdownRules => {
  return Nominal.create(BRAND, rules);
};

export const createDefault = (): CountdownRules => {
  return construct([]);
};

export const CountdownRules = {
  createDefault,
};