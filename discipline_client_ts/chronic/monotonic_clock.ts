import { Duration, DateTime, Instant, Branded } from "../x.ts";

const BRAND = Symbol();

export type MonotonicClock = Branded<typeof BRAND, {
  elapsedTime: Duration,
  previousSynchronizationTime: DateTime,
}>;

export const construct = (
  elapsedTime: Duration,
  previousSynchronizationTime: DateTime,
): MonotonicClock => {
  return Branded(BRAND, {
    elapsedTime,
    previousSynchronizationTime,
  });
};

export const create = (now: DateTime): MonotonicClock => {
  return construct(
    Duration.zero(),
    now,
  );
};

export const synchronize = (it: MonotonicClock, now: DateTime): void => {
  const interval = DateTime.tillOrZero(it.previousSynchronizationTime, now);
  it.elapsedTime = Duration.plusOrMax(it.elapsedTime, interval);
  it.previousSynchronizationTime = now;
};

export const getElapsedTime = (it: MonotonicClock): Duration => {
  return it.elapsedTime;
};

export const getPreviousSynchronizationTime = (it: MonotonicClock): DateTime => {
  return it.previousSynchronizationTime;
};

export const getNow = (it: MonotonicClock): Instant => {
  return Instant.fromElapsedTime(it.elapsedTime);
};

export const getNowAsDateTime = (it: MonotonicClock): DateTime => {
  const now = DateTime.now();
  if (DateTime.isEarilerThan(now, it.previousSynchronizationTime)) {
    return now;
  } else {
    return it.previousSynchronizationTime;
  }
};

export const MonotonicClock = {
  create,
  construct,
  synchronize,
  getElapsedTime,
  getPreviousSynchronizationTime,
  getNow,
  getNowAsDateTime,
};