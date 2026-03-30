import { Branded, MonotonicClock, UserProfile } from "./x.ts";

const BRAND = Symbol();

export type State = Branded<typeof BRAND, {
  monotonicClock: MonotonicClock,
  mainUserProfile: UserProfile,
}>;

export const State = {

};