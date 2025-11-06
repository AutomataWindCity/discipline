import { Countdown, DateTime, Duration, TypeId, Unique } from "../mod.ts";

type Status = 
  | StatusEffective 
  | StatusGoingIneffective 
  | StatusIneffective;

type StatusEffective = {
  readonly typeId: TypeId.CountdownAfterPleaConditionalStatusTypeEffective;
};
type StatusGoingIneffective = {
  readonly typeId: TypeId.CountdownAfterPleaConditionalStatusTypeGoingIneffective;
  readonly countdown: Countdown,
};
type StatusIneffective = {
  readonly typeId: TypeId.CountdownAfterPleaConditionalStatusTypeIneffective,
};

const StatusEffective = (): StatusEffective => {
  return {
    typeId: TypeId.CountdownAfterPleaConditionalStatusTypeEffective,
  }
};
const StatusGoingIneffective = (countdown: Countdown): StatusGoingIneffective => {
  return {
    typeId: TypeId.CountdownAfterPleaConditionalStatusTypeGoingIneffective,
    countdown,
  }
};
const StatusIneffective = (): StatusIneffective => {
  return {
    typeId: TypeId.CountdownAfterPleaConditionalStatusTypeIneffective,
  }
};

interface StatusMatchCases<A, B, C> {
  readonly Effective: () => A,
  readonly GoingIneffective: (countdown: Countdown) => B,
  readonly Ineffective: () => C,
}

const Status_match = <A, B, C>(status: Status, cases: StatusMatchCases<A, B, C>) => {
  switch (status.typeId) {
    case TypeId.CountdownAfterPleaConditionalStatusTypeEffective: {
      return cases.Effective();
    }
    case TypeId.CountdownAfterPleaConditionalStatusTypeGoingIneffective: {
      return cases.GoingIneffective(status.countdown);
    }
    case TypeId.CountdownAfterPleaConditionalStatusTypeIneffective: {
      return cases.Ineffective();
    }
  }
};

const Status_isEffective = (status: Status): status is StatusEffective => {
  return status.typeId === TypeId.CountdownAfterPleaConditionalStatusTypeEffective;
};
const Status_isGoingIneffective = (status: Status): status is StatusGoingIneffective => {
  return status.typeId === TypeId.CountdownAfterPleaConditionalStatusTypeGoingIneffective;
};
const Status_isIneffective = (status: Status): status is StatusIneffective => {
  return status.typeId === TypeId.CountdownAfterPleaConditionalStatusTypeIneffective;
};

export class CountdownAfterPleaConditional implements Unique {
  readonly typeId = TypeId.CountdownAfterPleaConditional;

  private constructor(
    private readonly _duration: Duration,
    private _status: Status,
  ) {}

  static new(duration: Duration) {
    return new CountdownAfterPleaConditional(duration, StatusIneffective());
  }

  makeEffective() {
    this._status = StatusEffective();
  }

  makeIneffective(now: DateTime) {
    if (Status_isEffective(this._status)) {
      this._status = StatusGoingIneffective(Countdown.new(this._duration, now));
    }
  }

  synchronize(now: DateTime) {
    if (this._status.typeId === TypeId.CountdownAfterPleaConditionalStatusTypeGoingIneffective) {
      this._status.countdown.synchronize(now);
      if (this._status.countdown.isFinished()) {
        this._status = StatusIneffective();
      }
    }
  }

  isEffective(): boolean {
    return !Status_isIneffective(this._status);
  }
}