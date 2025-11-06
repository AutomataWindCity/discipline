import { Branded, UserName } from "../mod.ts"

const BRAND = Symbol();

export type User = Branded<typeof BRAND, {
  name: UserName.UserName,
  regulationInfo: null,
  operatingSystemInfo: null,
}>;
