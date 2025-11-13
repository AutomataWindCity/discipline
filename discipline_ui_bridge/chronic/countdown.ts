import { Duration, DateTime, Branded } from "../mod.ts";

const BRAND = Symbol();

export type Countdown = Branded<typeof BRAND, {
  remainingDuration: Duration.Duration,
  previousSynchronizationTime: DateTime.DateTime,
}>;

export const construct = (
  remainingDuration: Duration.Duration,
  previousSynchronizationTime: DateTime.DateTime,
): Countdown => {
  return Branded(BRAND, {
    remainingDuration,
    previousSynchronizationTime,
  });
};

export const create = (duration: Duration.Duration, now: DateTime.DateTime): Countdown => {
  return construct(duration, now);
}

export const synchronize = (me: Countdown, now: DateTime.DateTime) => {
  const interval = DateTime.tillOrZero(
    me.previousSynchronizationTime, 
    now,
  );

  me.remainingDuration = Duration.minusOrZero(
    me.remainingDuration,
    interval,
  );

  me.previousSynchronizationTime = now;
}

export const isFinished = (me: Countdown): boolean => {
  return Duration.isZero(me.remainingDuration);
};