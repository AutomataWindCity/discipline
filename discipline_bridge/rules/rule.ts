import { RuleActivator, RuleProtector, Time, TypeId, Weekday } from "../mod.ts";

export class Rule {
  readonly typeId = TypeId.Rule;

  private constructor(
    private readonly _activator: RuleActivator,
    private readonly _protector: RuleProtector,
  ) {}

  isEffective(weekday: Weekday, time: Time) {
    return (
      this._protector.isEffective() 
      &&
      this._activator.isEffective(weekday, time)
    );
  }

  isProtected() {
    return this._protector.isEffective();
  }
}