import { Branded, Countdown, DateTime, Duration, Tried } from "../x.ts";

const BRAND = Symbol();

export type UptimeAllowance = Branded<typeof BRAND, {
  readonly allowance: Duration.DurationT,
  readonly countdown: Countdown.CountdownT,
}>;

export const create = (allowance: Duration.DurationT, now: DateTime.DateTimeT): UptimeAllowance => {
  return Branded(BRAND, {
    allowance,
    countdown: Countdown.create(allowance, now),
  });
};

export const construct = (allowance: Duration.DurationT, countdown: Countdown.CountdownT): UptimeAllowance => {
  return Branded(BRAND, {
    allowance,
    countdown,
  });
};

export const synchronize = (me: UptimeAllowance, now: DateTime.DateTimeT) => {
  Countdown.synchronize(me.countdown, now);
};

export const isAllowanceUp = (me: UptimeAllowance): boolean => {
  return Countdown.isFinished(me.countdown);
};

export const setRemainingDuration = (me: UptimeAllowance, newValue: Duration.DurationT) => {
  return Countdown.setRemaniningDuration(me.countdown, newValue);
};

export const getRemainingDuration = (me: UptimeAllowance): Duration.DurationT => {
  return Countdown.getRemainingDuration(me.countdown);
};

export const toJson = (me: UptimeAllowance): string => {
  return JSON.stringify([
    Duration.toTotalMilliseconds(me.allowance),
    Duration.toTotalMilliseconds(me.countdown.remainingDuration),
    DateTime.timestamp(me.countdown.previousSynchronizationTime),
  ]);
};

export const fromJson = (json: string): Tried.Tried<UptimeAllowance, Error> => {
  let parsed: unknown;
  try {
    parsed = JSON.parse(json);
  } catch (error) {
    return Tried.Failure(new Error("Creating UptimAllowance from json: Failed to parse json text", { cause: error }));
  }

  if (!isArray(parsed)) {
    return Tried.Failure(new Error(`Creating UptimAllowance from json: Json value is not array. Json: ${json}`));
  }

  const allowanceRaw = parsed[0];
  const countdownRemainingDurationRaw = parsed[1];
  const countdownPreviousSynchronizationTimeRaw = parsed[2];
  
  if (!isInteger(allowanceRaw)) {
    return Tried.Failure(new Error(`Creating UptimAllowance from json: Value for "allowance" is not number. Json: ${json}`));
  }
  if (!isInteger(countdownRemainingDurationRaw)) {
    return Tried.Failure(new Error(`Creating UptimAllowance from json: Value for "countdownRemainingDuration" is not number. Json: ${json}`));
  }
  if (!isInteger(countdownPreviousSynchronizationTimeRaw)) {
    return Tried.Failure(new Error(`Creating UptimAllowance from json: Value for "countdownPreviousSynchronizationTime" is not number. Json: ${json}`));
  }

  const allowance = Duration.fromMilliseconds(allowanceRaw);
  if (Tried.isFailure(allowance)) {
    return Tried.Failure(new Error("Creating UptimAllowance from json: Sanitization failed: Failed to santize the 'allowance' field", {
      cause: Tried.error(allowance)
    }));
  }

  const countdownRemainingDuration = Duration.fromMilliseconds(countdownRemainingDurationRaw);
  if (Tried.isFailure(countdownRemainingDuration)) {
    return Tried.Failure(new Error("Creating UptimAllowance from json: Sanitization failed: Failed to santize the 'countdownRemainingDuration' field", {
      cause: Tried.error(countdownRemainingDuration)
    }));
  }
  
  const countdownPreviousSynchronizationTime = DateTime.fromTimestamp(countdownPreviousSynchronizationTimeRaw);
  if (Tried.isFailure(countdownPreviousSynchronizationTime)) {
    return Tried.Failure(new Error("Creating UptimAllowance from json: Sanitization failed: Failed to santize the 'countdownPreviousSynchronizationTime' field", {
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

export const getAllowance = (me: UptimeAllowance): Duration.DurationT => {
  return me.allowance;
};

export const getCountdown = (me: UptimeAllowance): Countdown.CountdownT => {
  return me.countdown;
};