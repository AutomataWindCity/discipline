import { Countdown, DateTime, Duration, withVirtualKey } from "../mod.ts";

const STATUS_TYPE_ACTIVATED = 0;
const STATUS_TYPE_DEACTIVATING = 1;
const STATUS_TYPE_DEACTIVATED = 2;

type StatusActivated = {
  readonly type: typeof STATUS_TYPE_ACTIVATED,
};
type StatusDeactivating = {
  readonly type: typeof STATUS_TYPE_DEACTIVATING,
  readonly countdown: Countdown.Countdown,
};
type StatusDeactivated = {
  readonly type: typeof STATUS_TYPE_DEACTIVATED,
};

type Status = (
  | StatusActivated
  | StatusDeactivating 
  | StatusDeactivated
);

const Status_Activated = (): StatusActivated => {
  return { 
    type: STATUS_TYPE_ACTIVATED,
  };
};
const Status_Deactivating = (countdown: Countdown.Countdown): StatusDeactivating => {
  return {
    type: STATUS_TYPE_DEACTIVATING,
    countdown,
  };
};
const Status_Deactivated = (): StatusDeactivated => {
  return {
    type: STATUS_TYPE_DEACTIVATED,
  }
};

const Status_isActivated = (status: Status): status is StatusActivated => {
  return status.type === STATUS_TYPE_ACTIVATED;
};
const Status_isDeactivating = (status: Status): status is StatusDeactivating => {
  return status.type === STATUS_TYPE_DEACTIVATING;
};
const Status_isDeactivated = (status: Status): status is StatusDeactivated => {
  return status.type === STATUS_TYPE_DEACTIVATED;
};

type StatusMatchCases<A, B, C> = {
  Activated: () => A,
  Deactivating: (countdown: Countdown.Countdown) => B,
  Deactivated: () => C,
};

const Status_match = <A, B, C>(me: Status, cases: StatusMatchCases<A, B, C>) => {
  switch (me.type) {
    case STATUS_TYPE_ACTIVATED: {
      return cases.Activated();
    }
    case STATUS_TYPE_DEACTIVATING: {
      return cases.Deactivating(me.countdown);
    }
    case STATUS_TYPE_DEACTIVATED: {
      return cases.Deactivated();
    }
    default: {
      throw new Error("Matching against CountdownAfterPleaCondition Status variant: Unknown variant");
    }
  }
};

const BRAND = Symbol();

export type CountdownAfterPleaConditional = {
  duration: Duration.Duration,
  status: Status,
  readonly [BRAND]: "CountdownAfterPleaConditional",
};

const construct = (duration: Duration.Duration, status: Status): CountdownAfterPleaConditional => {
  return withVirtualKey(BRAND, {
    duration,
    status,
  });
};

export const create = (duration: Duration.Duration): CountdownAfterPleaConditional => {
  return construct(duration, Status_Deactivated());
};

export const activate = (me: CountdownAfterPleaConditional) => {
  me.status = Status_Activated();
};

export const deactivate = (me: CountdownAfterPleaConditional, now: DateTime.DateTime) => {
  if (Status_isActivated(me.status)) {
    me.status = Status_Deactivating(
      Countdown.create(me.duration, now)
    );
  }
};

export const synchronize = (me: CountdownAfterPleaConditional, now: DateTime.DateTime) => {
  if (Status_isDeactivating(me.status)) {
    Countdown.synchronize(me.status.countdown, now);
    if (Countdown.isFinished(me.status.countdown)) {
      me.status = Status_Deactivated();
    }
  }
};

export const isActivateOrDeactivating = (me: CountdownAfterPleaConditional): boolean => {
  return !Status_isDeactivated(me.status);
};