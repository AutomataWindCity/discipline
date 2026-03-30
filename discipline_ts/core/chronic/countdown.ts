import { Duration, Instant, Branded } from "../x.ts";

export const enum CountdownStatus {
  Pending,
  Running,
  Finished,
}

const BRAND = Symbol();
const STATUS = Symbol();

export type Countdown = Branded<typeof BRAND, {
  from: Instant,
  duration: Duration,
}>;

export type CountdownPending = Countdown & { 
  [STATUS]: CountdownStatus.Pending 
};

export type CountdownRunning = Countdown & { 
  [STATUS]: CountdownStatus.Running 
};

export type CountdownFinished = Countdown & { 
  [STATUS]: CountdownStatus.Finished 
};

export const construct = (from: Instant, duration: Duration): Countdown => {
  return Branded(BRAND, {
    from,
    duration,
  });
};

export const create = (from: Instant, duration: Duration): Countdown => {
  return construct(from, duration);
};

export const getFrom = (it: Countdown): Instant => {
  return it.from;
};

export const getTill = (it: Countdown): Instant => {
  return Instant.plusOrMax(it.from, it.duration);
};

export const getTotalDuration = (it: Countdown): Duration => {
  return it.duration;
};

export const setTotalDuration = (it: Countdown, duration: Duration): void => {
  it.duration = duration;
};

export const getTimeTillStartOrZero = (it: Countdown, now: Instant): Duration => {
  return Instant.tillOrZero(now, it.from);
};

export const getTimeSinceStartOrZero = (it: Countdown, now: Instant): Duration => {
  return Instant.sinceOrZero(now, it.from);
};

export const getElapsedTimeOrZero = (it: Countdown, now: Instant): Duration => {
  return Duration.min(
    getTimeSinceStartOrZero(it, now),
    it.duration
  );
};

export const getRemainingTimeOrZero = (it: Countdown, now: Instant): Duration => {
  return Duration.minusOrZero(
    getTotalDuration(it),
    getElapsedTimeOrZero(it, now)
  );
};

export const getTimeTillFinishOrZero = (it: Countdown, now: Instant): Duration => {
  return Instant.tillOrZero(now, getTill(it));
};

export const getStatus = (it: Countdown, now: Instant): CountdownStatus => {
  if (Instant.isEarilerThan(now, it.from)) {
    return CountdownStatus.Pending;
  }

  const elapsedTime = Instant.tillOrZero(it.from, now);
  if (Duration.isShorterThanOrEqualTo(elapsedTime, it.duration)) {
    return CountdownStatus.Running;
  }

  return CountdownStatus.Finished;
};

export const isPending = (it: Countdown, now: Instant): boolean => {
  return getStatus(it, now) === CountdownStatus.Pending;
};

export const isRunning = (it: Countdown, now: Instant): boolean => {
  return getStatus(it, now) === CountdownStatus.Running;
};

export const isFinished = (it: Countdown, now: Instant): boolean => {
  return getStatus(it, now) === CountdownStatus.Finished;
};

export const extendByOrSetToMax = (it: Countdown, factor: Duration): void => {
  it.duration = Duration.plusOrMax(it.duration, factor);
};

export const match = <A, B, C>(
  it: Countdown,
  now: Instant,
  cases: {
    Pending: (it: CountdownPending) => A,
    Running: (it: CountdownRunning) => B,
    Finished: (it: CountdownFinished) => C,
  }
): A | B | C => {
  switch (getStatus(it, now)) {
    case CountdownStatus.Pending: return cases.Pending(it as CountdownPending);
    case CountdownStatus.Running: return cases.Running(it as CountdownRunning);
    case CountdownStatus.Finished: return cases.Finished(it as CountdownFinished);
  }
};

export const Countdown = {
  construct,
  create,
  getFrom,
  getTill,
  getTotalDuration,
  setTotalDuration,
  getTimeTillStartOrZero,
  getTimeSinceStartOrZero,
  getElapsedTimeOrZero,
  getRemainingTimeOrZero,
  getTimeTillFinishOrZero,
  getStatus,
  isPending,
  isRunning,
  isFinished,
  extendByOrSetToMax,
  match,
};