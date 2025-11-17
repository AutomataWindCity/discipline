import { Branded } from "../../discipline_ui_bridge/mod.ts";

const BRAND = Symbol();

const LENGTH_VIOLATION = Symbol();

export type LengthViolation = Branded<typeof BRAND, {
  readonly type: typeof LENGTH_VIOLATION,
  readonly string: string,
}>;

export const LengthViolation = (string: string): LengthViolation => {
  return Branded(BRAND, {
    type: LENGTH_VIOLATION,
    string,
  });
};

export const isLengthViolation = (me: Error): me is LengthViolation => {
  return me.type === LENGTH_VIOLATION;
};

export type Error = LengthViolation;