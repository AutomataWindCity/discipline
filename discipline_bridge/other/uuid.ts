import * as Uuid from "npm:uuid";
import { TypeId } from "../mod.ts";

export class UuidV4 {
  readonly typeId = TypeId.UuidV4;
  
  private constructor(private readonly inner: Uuid.UUIDTypes) {}

  
}