import { TypeId, Unique } from "../mod.ts"

export class UserName implements Unique {
  readonly typeId = TypeId.OperatingSystemLinuxUserName;

  private constructor(private readonly inner: string) {}
}

export class User implements Unique {
  readonly typeId = TypeId.OperatingSystemLinuxUser;

  private constructor(
    private readonly _name: UserName,
    // private readonly _
  ) {}
}

export class UserGroup {
  readonly typeId = TypeId.UserGroup;
}