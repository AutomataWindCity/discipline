import { DateTime, Duration, TDateTime, Time, TimeRange } from "../discipline_ts/mod.ts"

import * as DeviceUptimeTracker from "./device_uptime_tracker.ts"
import { type TDeviceUptimeTracker } from "./device_uptime_tracker.ts"

// Sleep time condition
const sleepTimeRange = TimeRange.fromTimes(
  Time.fromHourAndMinutePmOrThrow(7, 0),
  Time.fromHourAndMinuteAmOrThrow(4, 0),
);

const condition1 = (now: TDateTime): boolean => {
  return TimeRange.contains(sleepTimeRange, DateTime.time(now));
};

const allowance = DeviceUptimeTracker.create(
  Duration.fromHoursOrThrow(2),
  DateTime.now(),
);

const condition2 = (): boolean => {
  return DeviceUptimeTracker.isAllowanceUp(allowance);
};