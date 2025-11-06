import { Time, Duration, withVirtualKey, Branded } from "../mod.ts";

const BRAND = Symbol();

export type DateTime = Branded<typeof BRAND, Date>;

const construct = (inner: Date): DateTime => {
  return Branded(BRAND, inner);
};

export const now = (): DateTime => {
  return construct(new Date());
};

export const time = (me: DateTime): Time.Time => {
  const hour = me.getUTCHours();
  const minute = me.getUTCMinutes();
  return Time.fromHourAndMinuteOrThrow(hour, minute);
};

export const tillOrZero = (lhs: DateTime, rhs: DateTime): Duration.Duration => {
  const lhsTimestamp = lhs.getTime();
  const rhsTimestamp = rhs.getTime();

  if (lhsTimestamp < rhsTimestamp) {
    return Duration.fromMillisecondsOrThrow(rhsTimestamp - lhsTimestamp);
  } else {
    return Duration.zero();
  }
};

export const sinceOrZero = (lhs: DateTime, rhs: DateTime): Duration.Duration => {
  const lhsTimestamp = lhs.getTime();
  const rhsTimestamp = rhs.getTime();

  if (lhsTimestamp > rhsTimestamp) {
    return Duration.fromMillisecondsOrThrow(lhsTimestamp - rhsTimestamp);
  } else {
    return Duration.zero();
  }
};

// export const equality = Equality.implement<DateTime>({
//   isEqualTo(lhs, rhs) {
//     return lhs.inner.getTime() === rhs.inner.getTime();
//   },
// });