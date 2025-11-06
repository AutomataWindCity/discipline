import { Countdown, DateTime, Duration, isSome, None, Option, TypeId, Unique } from "../mod.ts";

export class CountdownConditional implements Unique {
  readonly typeId = TypeId.CountdownConditional;

  private constructor(
    private _duration: Duration,
    private _countdown: Option<Countdown>,
  ) {}

  synchronize(now: DateTime) {
    if (isSome(this._countdown)) {
      this._countdown.value().synchronize(now);
      if (this._countdown.value().isFinished()) {
        this._countdown = None.new();
      }
    }
  }

  isEffective(): boolean {
    return isSome(this._countdown);
  }
}