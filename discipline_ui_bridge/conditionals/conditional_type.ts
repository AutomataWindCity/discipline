const VARIANT_TIME = Symbol();
const VARIANT_ALWAYS = Symbol();
const VARIANT_COUNTDOWN = Symbol();
const VARIANT_COUNTDOWN_AFTER_PLEA = Symbol();

export type Time = typeof VARIANT_TIME;
export type Always = typeof VARIANT_ALWAYS;
export type Countdown = typeof VARIANT_COUNTDOWN;
export type CountdownAfterPlea = typeof VARIANT_COUNTDOWN_AFTER_PLEA;

export type ConditionalType = (
  | Time
  | Always
  | Countdown
  | CountdownAfterPlea
);

export const Time = (): Time => {
  return VARIANT_TIME;
};
export const Always = (): Always => {
  return VARIANT_ALWAYS;
};
export const Countdown = (): Countdown => {
  return VARIANT_COUNTDOWN;
};
export const CountdownAfterPlea = (): CountdownAfterPlea => {
  return VARIANT_COUNTDOWN_AFTER_PLEA;
};

export type ConditionalTypeMatchCases<A, B, C, D> = {
  readonly Always: () => A,
  readonly Time: () => B,
  readonly Countdown: () => C,
  readonly CountdownAfterPlea: () => D,
};

export const match = <A, B, C, D>(
  me: ConditionalType, 
  cases: ConditionalTypeMatchCases<A, B, C, D>,
) => {
  switch (me) {
    case VARIANT_TIME: {
      return cases.Time();
    }
    case VARIANT_ALWAYS: {
      return cases.Always();
    }
    case VARIANT_COUNTDOWN: {
      return cases.Countdown();
    }
    case VARIANT_COUNTDOWN_AFTER_PLEA: {
      return cases.CountdownAfterPlea();
    }
    default: {
      throw new Error("Matching against ConditionalType: Unknown variant");
    }
  }
};


// const ALWAYS_NUMBER = 0;
// const TIME_NUMBER = 1;
// const COUNTDOWN_NUMBER = 2;
// const COUNTDOWN_AFTER_PLEA_NUMBER = 3;
