import { Duration } from "../x.ts"

export const registerIntervalTimer = (duration: Duration, fn: () => void): number => {
  return setInterval(fn, Duration.toTotalMilliseconds(duration));
};

export const registerTimeout = (duration: Duration, fn: () => void): number => {
  return setTimeout(fn, Duration.toTotalMilliseconds(duration));
};

export const clearIntervalTimer = (timerId: number) => {
  clearInterval(timerId);
};