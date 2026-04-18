import { Duration, Unique } from "../x.ts";

const BRAND = Symbol();

export type Instant = Unique<typeof BRAND, "Instant", Duration>;

export const construct = (elapsedTime: Duration): Instant => {
  return elapsedTime as Instant;
};

export const fromElapsedTime = (elapsedTime: Duration): Instant => {
  return construct(elapsedTime);
};

export const isAt = (lhs: Instant, rhs: Instant): boolean => {
  return Duration.isEqualTo(lhs, rhs);
};

export const isEarilerThan = (lhs: Instant, rhs: Instant): boolean => {
  return Duration.isShorterThan(lhs, rhs);
};

export const isEarilerThanOrAt = (lhs: Instant, rhs: Instant): boolean => {
  return Duration.isShorterThanOrEqualTo(lhs, rhs);
};

export const isLaterThan = (lhs: Instant, rhs: Instant): boolean => {
  return Duration.isLongerThan(lhs, rhs);
};

export const isLaterThanOrAt = (lhs: Instant, rhs: Instant): boolean => {
  return Duration.isLongerThanOrEqualTo(lhs, rhs);
};

export const tillOrZero = (lhs: Instant, rhs: Instant): Duration => {
  return Duration.saturatingSub(lhs, rhs);
};

export const sinceOrZero = (lhs: Instant, rhs: Instant): Duration => {
  return Duration.saturatingSub(rhs, lhs);
};

export const saturatingSub = (it: Instant, duration: Duration): Instant => {
  return construct(Duration.saturatingSub(it, duration));
};

export const saturatingAdd = (it: Instant, duration: Duration): Instant => {
  return construct(Duration.saturatingAdd(it, duration));
};

export const toElapsedTime = (it: Instant): Duration => {
  return it;
};

export const Instant = {
  fromElapsedTime,
  isAt,
  isEarilerThan,
  isEarilerThanOrAt,
  isLaterThan,
  isLaterThanOrAt,
  tillOrZero,
  sinceOrZero,
  saturatingSub,
  saturatingAdd,
  toElapsedTime,
};