import { Branded, ScreenRegulation, ApplicationRegulations, UptimeClock, VaultsStats } from "../x.ts";

const BRAND = Symbol();

export type UserProfile = Branded<typeof BRAND, {
  screenRegulation: ScreenRegulation,
  applicationRegulations: ApplicationRegulations,
  uptimeClock: UptimeClock,
  vaultsStats: VaultsStats,
}>;

export const UserProfile = {

};