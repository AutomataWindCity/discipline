import { TypeId, UuidV4, Rule } from "../mod.ts";

export class RuleGroup {
  readonly typeId = TypeId.RuleGroup;

  private constructor(
    private readonly _rules: Map<UuidV4, Rule>,
    private readonly _maximumRuleNumber: number,
  ) {}
}