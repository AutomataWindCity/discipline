import { Number, Some, None, Unique, Time, TypeId, Option, Duration } from "../mod.ts";

const minimumFromTimestamp = 0;
const maximumFromTimestamp = 1000 * 60 * 60 * 24 - 1;

const minimumTillTimestamp = 0;
const maximumTillTimestamp = 1000 * 60 * 60 * 24 * 2 - 1;

const millisecondsPerDay = 1000 * 60 * 60 * 24;

export class TimeRange implements Unique {
  readonly typeId = TypeId.TimeRange;

  private constructor(
    private readonly _from: number,
    private readonly _till: number,
  ) {}


  static fromHour12AndMinuteOrNone(from: number, till: number): Option<TimeRange> {
    if (
      Number.isInteger(from)
      &&
      from >= minimumFromTimestamp
      &&
      from <= maximumFromTimestamp
      &&
      Number.isInteger(from)
      &&
      till >= minimumTillTimestamp
      &&
      till <= maximumTillTimestamp
      &&
      from <= till
      &&
      till - from <= millisecondsPerDay
    ) {
      return Some.new(new TimeRange(from, till));
    }

    return None.new();
  };

  static fromTimes(from: Time, till: Time): TimeRange {
    const fromTimestamp = from.timestamp();
    const tillTimestamp = till.timestamp();

    if (fromTimestamp <= tillTimestamp) {
      return new TimeRange(fromTimestamp, tillTimestamp);
    } else {
      return new TimeRange(fromTimestamp, tillTimestamp + Duration.MILLISECONDS_PER_DAY);
    }
  }

  contains(time: Time): boolean {
    return time.timestamp() >= this._from && time.timestamp() <= this._till;
  }
}