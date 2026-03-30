import { DateTime, Duration, Instant } from "./x.ts";

export type AndroidMonitoringServiceState = {
  previousAppUsageStatsQueryInstant: Instant,
  previousAppUsageStatsQueryDateTime: DateTime,
  previousAppUsageStatsQueryTime: Date,
  appUsageStatsQueryInterval: Duration,
};

export const AndroidMonitoringServiceState = {

};