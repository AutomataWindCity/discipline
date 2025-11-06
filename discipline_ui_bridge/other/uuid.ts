import * as Uuid from "npm:uuid";
import { Branded } from "../mod.ts";

const BRAND = Symbol();

export type UuidV4 = Branded<typeof BRAND, Uuid.UUIDTypes>;

export const generate = (): UuidV4 => {
  return Branded(BRAND, Uuid.v4());
};