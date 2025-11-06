import { Duration, DateTime, TypeId, Unique } from "../mod.ts";

export class Countdown implements Unique {
  readonly typeId = TypeId.Countdown;

  private constructor(
    private _remainingDuration: Duration,
    private _previouSynchronizationTime: DateTime,
  ) {}

  static new(duration: Duration, now: DateTime) {
    return new Countdown(duration, now);
  }

  synchronize(now: DateTime) {
    this._remainingDuration = this
      ._previouSynchronizationTime
      .tillOrZero(now)
      .minusOrZero(this._remainingDuration);

    this._previouSynchronizationTime = now;
  }

  isFinished(): boolean {
    return this._remainingDuration.isZero();
  }
}
