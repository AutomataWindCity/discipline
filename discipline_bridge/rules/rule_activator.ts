import { AlwaysConditional, Time, TimeConditional, TypeId, Unique, Weekday } from "../mod.ts";

type Inner = (
  | AlwaysConditional
  | TimeConditional
)

export type RuleActivatorMatchCases<A, B> = {
  readonly Always: (conditional: AlwaysConditional) => A,
  readonly Time: (conditional: TimeConditional) => B,
}

export class RuleActivator implements Unique {
  readonly typeId = TypeId.RuleActivator;

  private constructor(private readonly inner: Inner) {}

  match<A, B>(cases: RuleActivatorMatchCases<A, B>) {
    switch (this.inner.typeId) {
      case TypeId.AlwaysConditional: {
        return cases.Always(this.inner);
      }
      case TypeId.TimeConditional: {
        return cases.Time(this.inner);
      }
    }
  }

  isEffective(weekday: Weekday, time: Time) {
    return this.match({
      Time: conditional => conditional.isEffective(weekday, time),
      Always: conditional => conditional.isEffective(),
    })
  }
}