import { Branded, RuleActivator, RuleProtector, Time, Weekday } from "../mod.ts";

const BRAND = Symbol();

export type Rule = Branded<typeof BRAND, {
  activator: RuleActivator.RuleActivator,
  protector: RuleProtector.RuleProtector,
}>;

export const isEffective = (
  me: Rule, 
  time: Time.Time,
  weekday: Weekday.Weekday, 
): boolean => {
  return (
    RuleProtector.isRuleProtected(me.protector)
    && 
    RuleActivator.isRuleActivated(me.activator, time, weekday)
  );
};

export const isProtected = (me: Rule): boolean => {
  return RuleProtector.isRuleProtected(me.protector);
};