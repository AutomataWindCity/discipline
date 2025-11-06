import { TypeId, Unique } from "../mod.ts";

export class Duration implements Unique {
  readonly typeId = TypeId.Duration;

  private constructor(private readonly inner: number) {}

  static readonly MAXIMUM_MILLISECONDS = Number.MAX_SAFE_INTEGER;
  static readonly MAXIMUM_SECONDS = Duration.MAXIMUM_MILLISECONDS / 1000;
  static readonly MAXIMUM_MINUTES = Duration.MAXIMUM_SECONDS / 60;
  static readonly MAXIMUM_HOURS = Duration.MAXIMUM_MINUTES / 60;

  static readonly MILLISECONDS_PER_SECOND = 1000;
  static readonly MILLISECONDS_PER_MINUTE = Duration.MILLISECONDS_PER_SECOND * 60;
  static readonly MILLISECONDS_PER_HOUR = Duration.MILLISECONDS_PER_MINUTE * 60;
  static readonly MILLISECONDS_PER_DAY = Duration.MILLISECONDS_PER_HOUR * 24;
  static readonly MILLISECONDS_PER_WEEK = Duration.MILLISECONDS_PER_DAY * 7;

  static fromMillisecondsOrThrow(milliseconds: number): Duration {
    if (Number.isInteger(milliseconds)) {
      throw new Error("Creating Duration from milliseconds: Argument 'milliseconds' is integer");
    }
    if (milliseconds < 0) {
      throw new Error("Creating Duration from milliseconds: Argument 'milliseconds' is less than minium value which is zero");
    }
    if (milliseconds > Duration.MAXIMUM_MILLISECONDS) {
      throw new Error(`Creating Duration from milliseconds: Argument 'milliseconds' is greater than maximum value which is ${Duration.MAXIMUM_MILLISECONDS}`);
    }
    return new Duration(milliseconds);
  }

  static fromHoursOrThrow(hours: number): Duration {
    if (Number.isInteger(hours)) {
      throw new Error("Creating Duration from hours: Argument 'hours' is integer");
    }
    if (hours < 0) {
      throw new Error("Creating Duration from hours: Argument 'hours' is less than minium value which is zero");
    }
    if (hours > Duration.MAXIMUM_HOURS) {
      throw new Error(`Creating Duration from hours: Argument 'hours' is greater than maximum value which is ${Duration.MAXIMUM_HOURS}`);
    }
    return new Duration(hours * Duration.MILLISECONDS_PER_HOUR);
  }

  static zero(): Duration {
    return new Duration(0);
  }

  milliseconds(): number {
    return this.inner;
  }

  minusOrZero(rhs: Duration): Duration {
  if (this.inner > rhs.inner) {
    return new Duration(this.inner - rhs.inner);
  } else {
    return Duration.zero();
  }
}

  isZero(): boolean {
    return this.inner === 0;
  }
}