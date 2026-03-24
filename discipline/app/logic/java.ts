export const Timer = java.util.Timer;
export type Timer = java.util.Timer;

export const TimerTask = java.util.TimerTask;
export type TimerTask = java.util.TimerTask;

export const TimeUnit = java.util.concurrent.TimeUnit;
export type TimeUnit = java.util.concurrent.TimeUnit;

export const System = java.lang.System;
export type System = java.lang.System;

@NativeClass()
export class Runnable extends java.lang.Runnable {
  private action: () => void;

  private constructor(action: () => void) {
    super();
    this.action = action;
    return global.__native(this);
  }

  static create(action: () => void) {
    return new Runnable(action);
  }

  override run(): void {
    this.action();
  }
}
