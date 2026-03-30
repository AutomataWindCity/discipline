import { Duration, Nominal } from "../x.ts";

const BRAND = Symbol();

export type Instant = Nominal<typeof BRAND, Duration>;

export const construct = (elapsedTime: Duration): Instant => {
  return Nominal.create(BRAND, elapsedTime);
};

export const fromElapsedTime = (elapsedTime: Duration): Instant => {
  return construct(elapsedTime);
};

export const isAt = (lhs: Instant, rhs: Instant): boolean => {
  return (
    Duration.toTotalMilliseconds(Nominal.get(lhs))
    ===
    Duration.toTotalMilliseconds(Nominal.get(rhs))
  );
};

export const isEarilerThan = (lhs: Instant, rhs: Instant): boolean => {
  return (
    Duration.toTotalMilliseconds(Nominal.get(lhs))
    <
    Duration.toTotalMilliseconds(Nominal.get(rhs))
  );
};

export const isEarilerThanOrAt = (lhs: Instant, rhs: Instant): boolean => {
  return (
    Duration.toTotalMilliseconds(Nominal.get(lhs))
    <=
    Duration.toTotalMilliseconds(Nominal.get(rhs))
  );
};

export const isLaterThan = (lhs: Instant, rhs: Instant): boolean => {
  return (
    Duration.toTotalMilliseconds(Nominal.get(lhs))
    >
    Duration.toTotalMilliseconds(Nominal.get(rhs))
  );
};

export const isLaterThanOrAt = (lhs: Instant, rhs: Instant): boolean => {
  return (
    Duration.toTotalMilliseconds(Nominal.get(lhs))
    >=
    Duration.toTotalMilliseconds(Nominal.get(rhs))
  );
};

export const tillOrZero = (lhs: Instant, rhs: Instant): Duration => {
  const lhsTimestamp = Duration.toTotalMilliseconds(Nominal.get(lhs));
  const rhsTimestamp = Duration.toTotalMilliseconds(Nominal.get(rhs));

  if (lhsTimestamp < rhsTimestamp) {
    return Duration.fromMillisecondsOrThrow(rhsTimestamp - lhsTimestamp);
  } else {
    return Duration.zero();
  }
};

export const sinceOrZero = (lhs: Instant, rhs: Instant): Duration => {
  const lhsTimestamp = Duration.toTotalMilliseconds(Nominal.get(lhs));
  const rhsTimestamp = Duration.toTotalMilliseconds(Nominal.get(rhs));

  if (lhsTimestamp > rhsTimestamp) {
    return Duration.fromMillisecondsOrThrow(lhsTimestamp - rhsTimestamp);
  } else {
    return Duration.zero();
  }
};

export const minusOrZero = (it: Instant, duration: Duration): Instant => {
  return construct(Duration.minusOrZero(
    toElapsedTime(it), 
    duration,
  ));
};

export const plusOrMax = (it: Instant, duration: Duration): Instant => {
  return construct(Duration.plusOrMax(
    toElapsedTime(it), 
    duration,
  ));
};

export const toElapsedTime = (it: Instant): Duration => {
  return Nominal.get(it);
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
  minusOrZero,
  plusOrMax,
  toElapsedTime,
};