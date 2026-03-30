import { Duration, DateTime, Branded, Date } from "../x.ts";

const BRAND = Symbol();

export type UptimeClock = Branded<typeof BRAND, {
  dailyUptime: Duration,
  previousSynchronizationTime: DateTime,
}>;

const construct = (
  dailyUptime: Duration,
  previousSynchronizationTime: DateTime,
): UptimeClock => {
  return Branded(BRAND, {
    dailyUptime,
    previousSynchronizationTime,
  });
};

export const create = (now: DateTime): UptimeClock => {
  return construct(
    Duration.zero(),
    now,
  );
};

export const synchronize = (
  it: UptimeClock,
  now: DateTime,
  synchronizationInterval: Duration,
  didSynchronizeSinceDevicePowerUp: boolean,
) => {
  if (Date.isLaterThan(
    DateTime.getDate(now), 
    DateTime.getDate(it.previousSynchronizationTime),
  )) {
    it.dailyUptime = Duration.zero();
    it.previousSynchronizationTime = now;
    return;
  }

  if (!didSynchronizeSinceDevicePowerUp) {
    it.previousSynchronizationTime = now;
    return;
  }

  const timeSincePreviousSynchronization = Duration.max(
    DateTime.tillOrZero(it.previousSynchronizationTime, now),
    synchronizationInterval,
  );
  

  it.dailyUptime = Duration.plusOrMax(
    it.dailyUptime,
    timeSincePreviousSynchronization,
  );
    
  it.previousSynchronizationTime = now;
}

export const getDailyUptime = (it: UptimeClock) => {
  return it.dailyUptime;
};

export const getPreviousSynchronizationTime = (it: UptimeClock): DateTime => {
  return it.previousSynchronizationTime;
};

export const UptimeClock = {
  create,
  synchronize,
  getDailyUptime,
  getPreviousSynchronizationTime,
};