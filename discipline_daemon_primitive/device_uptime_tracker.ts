import { TDateTime, None, TCountdown, TDuration, TOption, TUnique, Unique, Countdown } from "../discipline_ts/mod.ts";

export type TDeviceUptimeTracker = TUnique<"DailyDeviceActiveAllowanceConditional", {
  readonly allowance: TDuration,
  readonly countdown: TCountdown,
}>;

export const create = (allowance: TDuration, now: TDateTime): TDeviceUptimeTracker => {
  return Unique.create({
    allowance: allowance,
    countdown: Countdown.create(allowance, now),
  })
};

export const synchronize = (me: TDeviceUptimeTracker, now: TDateTime) => {
  Countdown.synchronize(me.countdown, now);
};

export const isAllowanceUp = (me: TDeviceUptimeTracker): boolean => {
  return Countdown.isFinished(me.countdown);
};