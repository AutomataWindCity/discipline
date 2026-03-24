import { DateTime, Duration, Instant } from "./x.ts";

export type AndroidMonitoringServiceState = {
  previousSynchronizationInstant: Instant,
  previousSynchronizationDateTime: DateTime,
  previousSynchronizationTime: Date,
  synchronizationInterval: Duration,
};