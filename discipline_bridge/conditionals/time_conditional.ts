import { TypeId, Weekday, Time, WeekdaySet, TimeRange, Unique } from "../mod.ts";

export class TimeConditional implements Unique {
  readonly typeId = TypeId.TimeConditional;

  private constructor(
    private readonly _weekdays: WeekdaySet,
    private readonly _timeRange: TimeRange
  ) {}

  isEffective(weekday: Weekday, time: Time): boolean {
    return this._weekdays.contains(weekday) && this._timeRange.contains(time)
  }
}