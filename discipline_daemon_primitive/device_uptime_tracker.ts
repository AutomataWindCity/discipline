import { Branded, Countdown, DateTime, Duration, Tried } from "../discipline_ui_bridge/mod.ts";

const BRAND = Symbol();

export type DeviceUptimeTracker = Branded<typeof BRAND, {
  readonly allowance: Duration.Duration,
  readonly countdown: Countdown.Countdown,
}>;

export const create = (allowance: Duration.Duration, now: DateTime.DateTime): DeviceUptimeTracker => {
  return Branded(BRAND, {
    allowance,
    countdown: Countdown.create(allowance, now),
  });
};

export const construct = (allowance: Duration.Duration, countdown: Countdown.Countdown): DeviceUptimeTracker => {
  return Branded(BRAND, {
    allowance,
    countdown,
  });
};

export const synchronize = (me: DeviceUptimeTracker, now: DateTime.DateTime) => {
  Countdown.synchronize(me.countdown, now);
};

export const isAllowanceUp = (me: DeviceUptimeTracker): boolean => {
  return Countdown.isFinished(me.countdown);
};

export const toJson = (me: DeviceUptimeTracker): string => {
  return JSON.stringify([
    Duration.milliseconds(me.allowance),
    Duration.milliseconds(me.countdown.remainingDuration),
    DateTime.timestamp(me.countdown.previousSynchronizationTime),
  ]);
};

export const fromJson = (json: string): Tried.Tried<DeviceUptimeTracker, Error> => {
  let parsed: unknown;
  try {
    parsed = JSON.parse(json);
  } catch (error) {
    return Tried.Failure(new Error("Creating DeviceUptimeTracker from json: Failed to parse json text", { cause: error }));
  }

  if (
    !isArray(parsed) 
    || 
    parsed.length < 3
    ||
    !isInteger(parsed[0])
    ||
    !isInteger(parsed[1])
    ||
    !isInteger(parsed[2])
  ) {
    return Tried.Failure(new Error("Creating DeviceUpTimeTracker from json: Sanitization failed"));
  }

  const allowance = Duration.fromMilliseconds(parsed[0]);
  const countdownRemainingDuration = Duration.fromMilliseconds(parsed[1]);
  const countdownPreviousSynchronizationTime = DateTime.fromTimestamp(parsed[2]);
  if (Tried.isFailure(allowance)) {
    return Tried.Failure(new Error("Creating DeviceUpTimeTracker from json: Sanitization failed: Failed to santize the 'allowance' field", {
      cause: Tried.error(allowance)
    }));
  }
  if (Tried.isFailure(countdownRemainingDuration)) {
    return Tried.Failure(new Error("Creating DeviceUpTimeTracker from json: Sanitization failed: Failed to santize the 'countdownRemainingDuration' field", {
      cause: Tried.error(countdownRemainingDuration)
    }));
  }
  if (Tried.isFailure(countdownPreviousSynchronizationTime)) {
    return Tried.Failure(new Error("Creating DeviceUpTimeTracker from json: Sanitization failed: Failed to santize the 'countdownPreviousSynchronizationTime' field", {
      cause: Tried.error(countdownPreviousSynchronizationTime)
    }));
  }

  return Tried.Success(construct(
    Tried.value(allowance), 
    Countdown.construct(
      Tried.value(countdownRemainingDuration),
      Tried.value(countdownPreviousSynchronizationTime),
    )
  ));
};

const isArray = (me: unknown): me is unknown[] => {
  return Array.isArray(me);
};
const isInteger = (me: unknown): me is number => {
  return Number.isInteger(me);
};