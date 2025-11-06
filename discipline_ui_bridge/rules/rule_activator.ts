import { AlwaysConditional, Branded, curry1, Time, TimeConditional, Weekday } from "../mod.ts";

const VARIANT_TIME = 0;
const VARIANT_ALWAYS = 1;

const BRAND = Symbol();

export type RuleActivator = Branded<typeof BRAND, {
  readonly type: typeof VARIANT_TIME,
  readonly conditional: TimeConditional.TimeConditional,
} | {
  readonly type: typeof VARIANT_ALWAYS,
  readonly conditional: AlwaysConditional.AlwaysConditional,
}>;

export const fromTimeConditional = (conditional: TimeConditional.TimeConditional): RuleActivator => {
  return Branded(BRAND, {
    type: VARIANT_TIME,
    conditional,
  });
};

export const fromAlwaysConditional = (conditional: AlwaysConditional.AlwaysConditional): RuleActivator => {
  return Branded(BRAND, {
    type: VARIANT_ALWAYS,
    conditional,
  });
};

export type RuleActivatorMatchCases<A, B> = {
  readonly Time: (conditional: TimeConditional.TimeConditional) => B,
  readonly Always: (conditional: AlwaysConditional.AlwaysConditional) => A,
}

export const match = <A, B>(
  me: RuleActivator, 
  cases: RuleActivatorMatchCases<A, B>,
): A | B => {
  switch (me.type) {
    case VARIANT_TIME: return cases.Time(me.conditional);
    case VARIANT_ALWAYS: return cases.Always(me.conditional);
  }
};

export const isRuleActivated = (
  me: RuleActivator,
  time: Time.Time,
  weekday: Weekday.Weekday, 
): boolean => {
  return match(me, {
    Time: curry1(TimeConditional.contains, weekday, time),
    Always: AlwaysConditional.isEffective,
  });
};
