import { Branded, CountdownAfterPleaConditional, CountdownConditional, curry1, DateTime } from "../mod.ts";

const VARIANT_COUNTDOWN = 0;
const VARIANT_COUNTDOWN_AFTER_PLEA = 1;

const BRAND = Symbol();

export type RuleProtector = Branded<typeof BRAND, {
  readonly type: typeof  VARIANT_COUNTDOWN,
  readonly value: CountdownConditional.CountdownConditional,
} | {
  readonly type: typeof VARIANT_COUNTDOWN_AFTER_PLEA,
  readonly value: CountdownAfterPleaConditional.CountdownAfterPleaConditional,
}>;

export const fromCountdownConditional = (conditional: CountdownConditional.CountdownConditional): RuleProtector => {
  return Branded(BRAND, {
    type: VARIANT_COUNTDOWN,
    value: conditional,
  });
};

export const fromCountdownAfterPleaConditional = (conditional: CountdownAfterPleaConditional.CountdownAfterPleaConditional): RuleProtector => {
  return Branded(BRAND, {
    type: VARIANT_COUNTDOWN_AFTER_PLEA,
    value: conditional,
  });
};

export type RuleProtectorMatchCases<A, B> = {
  readonly Countdown: (conditional: CountdownConditional.CountdownConditional) => A,
  readonly CountdownAfterPlea: (conditional: CountdownAfterPleaConditional.CountdownAfterPleaConditional) => B,
};

export const match = <A, B>(
  me: RuleProtector, 
  cases: RuleProtectorMatchCases<A, B>,
): A | B => {
  switch (me.type) {
    case 0: return cases.Countdown(me.value);
    case 1: return cases.CountdownAfterPlea(me.value);
  }
}

export const isRuleProtected = (me: RuleProtector): boolean => {
  return match(me, {
    Countdown: CountdownConditional.isCountdownRunning,
    CountdownAfterPlea: CountdownAfterPleaConditional.isActivateOrDeactivating,
  });
};

export const synchronize = (me: RuleProtector, now: DateTime.DateTime) => {
  match(me, {
    Countdown: curry1(CountdownConditional.synchronize, now),
    CountdownAfterPlea: curry1(CountdownAfterPleaConditional.synchronize, now),
  });
};