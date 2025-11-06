import { TypeId } from "../mod.ts";

export class AlwaysConditional {
  readonly typeId = TypeId.AlwaysConditional;

  private constructor() {}

  static new() {
    return new AlwaysConditional();
  }

  isEffective() {
    return true;
  }
}