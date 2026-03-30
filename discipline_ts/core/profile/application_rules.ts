import { ApplicationName, ApplicationRegulation, Branded } from "../x.ts";

const BRAND = Symbol();

export type ApplicationRules = Branded<typeof BRAND, {
  regulations: Map<ApplicationName, ApplicationRegulation>,
}>;

export const construct = (
  regulations: Map<ApplicationName, ApplicationRegulation>,
): ApplicationRules => {
  return Branded(BRAND, {
    regulations,
  });
};

export const createDefault = () => {
  return construct(new Map());
};

export const ApplicationRules = {
  createDefault,
};