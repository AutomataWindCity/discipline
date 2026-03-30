export const enum ConditionalTag {
  UptimeAllowance,
  Countdown,
  TimeRange,
}

export type AnyConditionalTag = (
  | ConditionalTypeCountdown
  | ConditionalTypeTimeRange
  | ConditionalTypeUptimeAllowance
);

export class ConditionalType {

}

export class ConditionalTypeCountdown extends ConditionalType {
  readonly asNumber = 0;
  readonly asString = "Countdown";

  static it = new ConditionalTypeCountdown();
}

export class ConditionalTypeTimeRange extends ConditionalType {
  readonly asNumber = 1;
  readonly asString = "TimeRange";

  static it = new ConditionalTypeTimeRange();
}

export class ConditionalTypeUptimeAllowance extends ConditionalType {
  readonly asNumber = 2;
  readonly asString = "UptimeAllowance";

  static it = new ConditionalTypeUptimeAllowance();
}
