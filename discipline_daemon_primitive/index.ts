import { DateTime, Duration, Time, TimeRange, Tried } from "../discipline_ui_bridge/mod.ts"
import * as DeviceUptimeTracker from "./device_uptime_tracker.ts"
import * as Async from "./async.ts"
import * as Storage from "./file.ts";

// Sleep time condition
const sleepTimeRange = TimeRange.fromTimes(
  Time.fromHourAndMinutePmOrThrow(7, 0),
  Time.fromHourAndMinuteAmOrThrow(4, 0),
);

const isNowSleepTime = (now: DateTime.DateTime): boolean => {
  return TimeRange.contains(sleepTimeRange, DateTime.time(now));
};

// Device usage alloawnce
const allowanceStorage = Storage.create({
  path: "",
  fallback: () => DeviceUptimeTracker.create(
    Duration.fromHoursOrThrow(2),
    DateTime.now(),
  ),
  serialize: DeviceUptimeTracker.toJson,
  deserialize: DeviceUptimeTracker.fromJson,
});

const allowance = DeviceUptimeTracker.create(
  Duration.fromHoursOrThrow(2),
  DateTime.now(),
);

const isAllowanceUp = (): boolean => {
  return DeviceUptimeTracker.isAllowanceUp(allowance);
};

Async.registerIntervalTimer(Duration.fromMillisecondsOrThrow(1000 * 60), async () => {
  DeviceUptimeTracker.synchronize(allowance, DateTime.now());
  const tried = await Storage.write(allowanceStorage, allowance);
  if (Tried.isFailure(tried)) {
    console.error(Tried.error(tried));
  }
});