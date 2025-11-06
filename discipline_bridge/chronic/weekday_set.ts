import { Weekday } from "../mod.ts";

const MON = 0b0000001;
const TUE = 0b0000010;
const WED = 0b0000100;
const THU = 0b0001000;
const FRI = 0b0010000;
const SAT = 0b0100000;
const SUN = 0b1000000;

export class WeekdaySet {
  private mask: number = 0; // bits 0–6 used

  // Add a day
  add(weekday: Weekday): void {
    this.mask |= weekday.match({
      Mon: () => MON,
      Tue: () => TUE,
      Wed: () => WED,
      Fri: () => FRI,
      Sat: () => SAT,
      Thu: () => THU,
      Sun: () => SUN,
    });
  }

  remove(weekday: Weekday): void {
    this.mask &= ~weekday.match({
      Mon: () => MON,
      Tue: () => TUE,
      Wed: () => WED,
      Fri: () => FRI,
      Sat: () => SAT,
      Thu: () => THU,
      Sun: () => SUN,
    });
  }

  // Check if a day is included
  contains(weekday: Weekday): boolean {
    return !!(this.mask & weekday.match({
      Mon: () => MON,
      Tue: () => TUE,
      Wed: () => WED,
      Fri: () => FRI,
      Sat: () => SAT,
      Thu: () => THU,
      Sun: () => SUN,
    }));
  }

  // Toggle a day
  toggle(weekday: Weekday): void {
    this.mask ^= weekday.match({
      Mon: () => MON,
      Tue: () => TUE,
      Wed: () => WED,
      Fri: () => FRI,
      Sat: () => SAT,
      Thu: () => THU,
      Sun: () => SUN,
    });
  }

  // Clear all
  clear(): void {
    this.mask = 0;
  }

  // Set all weekdays
  setAll(): void {
    this.mask = 0b1111111;
  }
}
