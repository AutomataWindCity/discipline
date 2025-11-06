import { CountdownAfterPleaConditional, CountdownConditional, DateTime, TypeId } from "../mod.ts";

type Inner = (
  | CountdownConditional
  | CountdownAfterPleaConditional
);

export type RuleProtectorMatchCases<A, B> = {
  readonly Countdown: (conditional: CountdownConditional) => A,
  readonly CountdownAfterPlea: (conditional: CountdownAfterPleaConditional) => B,
};

export class RuleProtector {
  readonly typeId = TypeId.RuleProtector;

  private constructor(private readonly inner: Inner) {}

  match<A, B>(cases: RuleProtectorMatchCases<A, B>) {
    switch (this.inner.typeId) {
      case TypeId.CountdownConditional: {
        return cases.Countdown(this.inner);
      }
      case TypeId.CountdownAfterPleaConditional: {
        return cases.CountdownAfterPlea(this.inner);
      }
    }
  }

  isEffective() {
    return this.match({
      Countdown: conditional => conditional.isEffective(),
      CountdownAfterPlea: conditional => conditional.isEffective(),
    });
  }

  synchronize(now: DateTime) {
    this.match({
      Countdown: conditional => conditional.synchronize(now),
      CountdownAfterPlea: conditional => conditional.synchronize(now),
    });
  }

  conditional(): Inner {
    return this.inner;
  }
}