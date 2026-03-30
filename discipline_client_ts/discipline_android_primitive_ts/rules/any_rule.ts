import { ConditionalTag, CountdownConditional, DateTime, Duration, Instant, TimeRangeConditional, UptimeAllowanceConditional } from "../x.ts";

export type Conditional = (
  | CountdownConditional
  | TimeRangeConditional
  | UptimeAllowanceConditional
);

export const Conditional = {
  isAlive: (it: Conditional, now: Instant): boolean => {
    switch (it.tag) {
      case ConditionalTag.UptimeAllowance: {
        return !it.getLifetime().isFinished(now);
      }
      case ConditionalTag.Countdown: {
        return !it.countdown.isFinished(now);
      }
      case ConditionalTag.TimeRange: {
        return !it.lifetime.isFinished(now);
      }
    }
  },

  isActive: (
    it: Conditional, 
    instant: Instant,
    datetime: DateTime,
    dailyUptime: Duration,
  ): boolean => {
    switch (it.tag) {
      case ConditionalTag.TimeRange: {
        return (
          it.lifetime.isRunning(instant) 
          && 
          it.timeRange.contains(datetime.getTime())
        );
      }
      case ConditionalTag.Countdown: {
        return (
          it.countdown.isRunning(instant)
        );
      }
      case ConditionalTag.UptimeAllowance: {
        return (
          it.lifetime.isRunning(instant)
          &&
          it.isAllowanceUp(dailyUptime)
        );
      }
    }
  }
};

export const Conditionals = {
  removeDead: (conditionals: Conditional[], now: Instant) => {
    let length = conditionals.length;
    let index = 0;

    while (true) {
      if (index >= length) {
        break;
      }

      const conditional = conditionals[index];
      console.log("index", index)
      console.log("index", conditional);
      if (!Conditional.isAlive(conditional, now)) {
        conditionals.splice(index, 1);
        length -= 1;
        continue;
      }

      index += 1;
      continue;
    }
  },
};
