import { Branded, User, UuidV4 } from "../mod.ts";

const BRAND = Symbol();

export type UserGroup = Branded<typeof BRAND, {
  users: Map<UuidV4, User.User>,
}>;