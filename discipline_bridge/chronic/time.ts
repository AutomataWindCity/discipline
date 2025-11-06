import { Option, Number, Some, Unique, None, TypeId } from "../mod.ts";

export class Time implements Unique {
  readonly typeId = TypeId.Time;

  private constructor(private readonly inner: number) {}

  static readonly minimumTimestamp = 0;
  static readonly maximumTimestamp = 1000 * 60 * 60 * 24 - 1;

  static readonly millisecondsPerMinute = 1000 * 60;
  static readonly millisecondsPerHour = 1000 * 60 * 60;

  static fromTimestampOrNone(timestamp: number): Option<Time> {
    if (
      Number.isInteger(timestamp) 
      && 
      timestamp >= Time.minimumTimestamp
      &&
      timestamp <= Time.maximumTimestamp
    ) {
      return Some.new(new Time(timestamp));
    }

    return None.new();
  };

  static fromHourAndMinuteAmOrThrow(hour: number, minute: number): Time {
    if (!Number.isInteger(hour)) {
      throw new Error("Creating Time from hour (AM) and minute: Hour is not integer");
    }
    if (hour < 0) {
      throw new Error("Creating Time from hour (AM) and minute: Hour is less than zero");
    }
    if (hour > 11) {
      throw new Error("Creating Time from hour (AM) and minute: Hour is greater than 11");
    }
    if (!Number.isInteger(minute)) {
      throw new Error("Creating Time from hour (AM) and minute: Minute is not integer");
    }
    if (minute < 0) {
      throw new Error("Creating Time from hour (AM) and minute: Minute less than zero");
    }
    if (minute > 59) {
      throw new Error("Creating Time from hour (AM) and minute: Minute is greater than 59");
    }

    return new Time(
      hour * Time.millisecondsPerHour
      +
      minute * Time.millisecondsPerMinute
    )
  };

  static fromHourAndMinutePmOrThrow(hour: number, minute: number): Time {
    if (!Number.isInteger(hour)) {
      throw new Error("Creating Time from hour (PM) and minute: Hour is not integer");
    }
    if (hour < 0) {
      throw new Error("Creating Time from hour (PM) and minute: Hour is less than zero");
    }
    if (hour > 11) {
      throw new Error("Creating Time from hour (PM) and minute: Hour is greater than 11");
    }
    if (!Number.isInteger(minute)) {
      throw new Error("Creating Time from hour (PM) and minute: Minute is not integer");
    }
    if (minute < 0) {
      throw new Error("Creating Time from hour (PM) and minute: Minute less than zero");
    }
    if (minute > 59) {
      throw new Error("Creating Time from hour (PM) and minute: Minute is greater than 59");
    }

    return new Time(
      (12 + hour) * Time.millisecondsPerHour
      +
      minute * Time.millisecondsPerMinute
    );
  };

  static fromHourAndMinuteOrThrow(hour: number, minute: number): Time {
    if (!Number.isInteger(hour)) {
      throw new Error("Creating Time from hour and minute: Hour is not integer");
    }
    if (hour < 0) {
      throw new Error("Creating Time from hour and minute: Hour is less than zero");
    }
    if (hour > 23) {
      throw new Error("Creating Time from hour and minute: Hour is greater than 23");
    }
    if (!Number.isInteger(minute)) {
      throw new Error("Creating Time from hour and minute: Minute is not integer");
    }
    if (minute < 0) {
      throw new Error("Creating Time from hour and minute: Minute less than zero");
    }
    if (minute > 59) {
      throw new Error("Creating Time from hour and minute: Minute is greater than 59");
    }

    return new Time(
      hour * Time.millisecondsPerHour
      +
      minute * Time.millisecondsPerMinute
    );
  };

  timestamp(): number {
    return this.inner;
  }
}
