
import { Unique } from "../x.ts";

const BRAND = Symbol();
export type Integer = Unique<typeof BRAND, "Integer", number>;

const uncheckedFromNumber = (number: number): Integer => {
  return number as Integer;
};

const isInteger = Number.isInteger as ((value: unknown) => value is number);

const fromString = (string: string): number | null => {
  let result: number;

  try {
    result = parseInt(string);
  } catch (error) {
    return null;
  }

  if (!Number.isSafeInteger(result)) {
    return null;
  }

  return result;
};

const parseInteger = (string: string) => {
  let number;

  try {
    number = parseInt(string);
  } catch (error) {
    return null;
  }

  if (number !== number) {
    return null;
  }

  return number;
};

export const Integer = {
  uncheckedFromNumber,
};