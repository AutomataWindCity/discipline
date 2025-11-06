import { Equality, Time, Duration, TypeId, Unique } from "../mod.ts";

export class DateTime implements Unique {
  readonly typeId = TypeId.DateTime;

  private constructor(private readonly inner: Date) {}

  static now(): DateTime {
    return new DateTime(new Date());
  }

  time(): Time {
    const hour = this.inner.getUTCHours();
    const minute = this.inner.getUTCMinutes();
    return Time.fromHourAndMinuteOrThrow(hour, minute);
  }

  tillOrZero(rhs: DateTime): Duration {
    const lhsTimestamp = this.inner.getTime();
    const rhsTimestamp = rhs.inner.getTime();

    if (lhsTimestamp < rhsTimestamp) {
      return Duration.fromMillisecondsOrThrow(rhsTimestamp - lhsTimestamp);
    } else {
      return Duration.zero();
    }
  };

  sinceOrZero(rhs: DateTime): Duration {
    const lhsTimestamp = this.inner.getTime();
    const rhsTimestamp = rhs.inner.getTime();

    if (lhsTimestamp > rhsTimestamp) {
      return Duration.fromMillisecondsOrThrow(lhsTimestamp - rhsTimestamp);
    } else {
      return Duration.zero();
    }
  };

  static equality = Equality.implement<DateTime>({
    isEqualTo(lhs, rhs) {
      return lhs.inner.getTime() === rhs.inner.getTime();
    },
  });
}