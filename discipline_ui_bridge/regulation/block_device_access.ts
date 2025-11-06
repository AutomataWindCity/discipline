import { RuleGroup, TypeId } from "../mod.ts";

export class BlockUserAccess {
  readonly typeId = TypeId.RegulationBlockUserAccess;

  private constructor(private readonly _rules: RuleGroup) {}
}