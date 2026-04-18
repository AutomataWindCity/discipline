import { ConditionalVariantEnum, Countdown, CountdownStatus, Duration, Instant, Nullable, Unique } from "../x"

export const enum CountdownAfterPleaConditionalStatusEnum {
  Active, 
  Deactivating,
  Deactivated,
}

const Status = {
  
};

const BRAND = Symbol();

type RawCountdownAfterPleaConditional = {
  readonly vraiant: ConditionalVariantEnum.CountdownAfterPlea,
  duration: Duration,
  countdown: Nullable<Countdown>,
};

type CountdownAfterPleaConditional = Unique<typeof BRAND, "CountdownAfterPleaConditional", RawCountdownAfterPleaConditional>;

const construct = (
  duration: Duration,
  countdown: Nullable<Countdown>,
): CountdownAfterPleaConditional => {
  return {
    vraiant: ConditionalVariantEnum.CountdownAfterPlea,
    duration,
    countdown,
  } satisfies RawCountdownAfterPleaConditional as CountdownAfterPleaConditional;
};

const reconstruct = construct;

const create = (
  duration: Duration,
): CountdownAfterPleaConditional => {
  return construct(
    duration,
    null,
  );
};

const getStatus = (
  it: CountdownAfterPleaConditional, 
  now: Instant,
): CountdownAfterPleaConditionalStatusEnum => {
  if (it.countdown === null) {
    return CountdownAfterPleaConditionalStatusEnum.Active;
  }

  switch (Countdown.getStatus(it.countdown, now)) {
    case CountdownStatus.Pending: {
      return CountdownAfterPleaConditionalStatusEnum.Active;
    }
    case CountdownStatus.Running: {
      return CountdownAfterPleaConditionalStatusEnum.Deactivating;
    }
    case CountdownStatus.Finished: {
      return CountdownAfterPleaConditionalStatusEnum.Deactivated;
    }
  }
};

const isActive = (
  it: CountdownAfterPleaConditional,
  now: Instant,
): boolean => {
  return getStatus(it, now) === CountdownAfterPleaConditionalStatusEnum.Active;
};

const isActiveOrDeactivating = (
  it: CountdownAfterPleaConditional,
  now: Instant,
): boolean => {
  return getStatus(it, now) !== CountdownAfterPleaConditionalStatusEnum.Deactivated;
};

const isDeactivating = (
  it: CountdownAfterPleaConditional,
  now: Instant,
): boolean => {
  return getStatus(it, now) === CountdownAfterPleaConditionalStatusEnum.Deactivating;
};

const isDeactivated = (
  it: CountdownAfterPleaConditional,
  now: Instant,
): boolean => {
  return getStatus(it, now) === CountdownAfterPleaConditionalStatusEnum.Deactivated;
};

const reactivate = (
  it: CountdownAfterPleaConditional,
) => {
  it.countdown = null;
};

const reDeactivate = (
  it: CountdownAfterPleaConditional,
  now: Instant,
) => {
  it.countdown = Countdown.create(now, it.duration);
};

type CountdownAfterPleaConditionalReDeactivateState = Countdown;

const createReDeactivateState = (
  it: CountdownAfterPleaConditional,
  now: Instant,
): CountdownAfterPleaConditionalReDeactivateState => {
  return Countdown.create(now, it.duration);
};

const reDeactivateFromState = (
  it: CountdownAfterPleaConditional,
  reDeactivateState: CountdownAfterPleaConditionalReDeactivateState,
) => {
  it.countdown = reDeactivateState;
};

export const CountdownAfterPleaConditional = {
  reconstruct,
  create,
  getStatus,
  isActive,
  isActiveOrDeactivating,
  isDeactivating,
  isDeactivated,
  reactivate,
  reDeactivate,
  createReDeactivateState,
  reDeactivateFromState,
};