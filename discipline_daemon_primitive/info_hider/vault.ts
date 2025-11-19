import { Branded, Countdown } from "../../discipline_ui_bridge/mod.ts";
import * as VaultName from "./vault_name.ts"
import * as VaultData from "./vault_data.ts"

const BRAND = Symbol();

export type Vault = Branded<typeof BRAND, {
  readonly name: VaultName.VaultName,
  readonly data: VaultData.VaultData,
  readonly protection: Countdown.Countdown
}>;
