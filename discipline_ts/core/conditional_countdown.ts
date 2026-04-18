import { ConditionalVariantEnum, Countdown, Duration, Instant, Nullable, Unique } from "../x.ts";

const BRAND = Symbol();

type RawCountdownConditional = {
  readonly variant: ConditionalVariantEnum.Countdown,
  duration: Duration,
  countdown: Nullable<Countdown>,
};

export type CountdownConditional = Unique<typeof BRAND, "CountdownConditional", RawCountdownConditional>;

const construct = (
  duration: Duration,
  countdown: Nullable<Countdown>,
): CountdownConditional => {
  return {
    variant: ConditionalVariantEnum.Countdown,
    duration,
    countdown,
  } satisfies RawCountdownConditional as CountdownConditional;
};

const reconstruct = construct;

const create = (duration: Duration): CountdownConditional => {
  return construct(duration, null);
};

const isActive = (it: CountdownConditional, now: Instant): boolean => {
  return it.countdown !== null && Countdown.isRunning(it.countdown, now);
};

const reactivate = (it: CountdownConditional, now: Instant) => {
  it.countdown = Countdown.create(now, it.duration);
};

export type CountdownConditionalReactivateState = Countdown;

const createReactivateState = (it: CountdownConditional, now: Instant): CountdownConditionalReactivateState => {
  return Countdown.create(now, it.duration);
};

const reactivateFromState = (it: CountdownConditional, reactivateState: CountdownConditionalReactivateState) => {
  it.countdown = reactivateState;
};

export const CountdownConditional = {
  reconstruct,
  create,
  isActive,
  reactivate,
  createReactivateState,
  reactivateFromState,
};