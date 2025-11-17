import { Duration } from "../discipline_ui_bridge/mod.ts"

export const registerIntervalTimer = (duration: Duration.Duration, fn: () => void): number => {
  return setInterval(fn, Duration.milliseconds(duration));
};

export const clearIntervalTimer = (timerId: number) => {
  clearInterval(timerId)
};