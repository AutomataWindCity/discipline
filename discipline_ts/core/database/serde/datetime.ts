import { Countdown, CountdownRule, Date, DateTime, Duration, FAILURE, Instant, MonotonicClock, SUCCESS, Time, TimeRange, TimeRangeRule, Vault, VaultData, VaultName } from "../../x.ts";
import { IntegerDescriptor, CustomScalarDescriptor, Column, CompoundValueReadSource, CompoundValueReadWrite, ObjectDescriptor, ObjectScalarPropertyDescriptor, StringDescriptor, ObjectCompoundPropertyDescriptor, BooleanDescriptor } from "../mod.ts";

export const DateTimeDescriptor = CustomScalarDescriptor.create<DateTime, number>({
  name: "DateTime",
  
  descriptor: IntegerDescriptor.create(),

  fromInner: (value, textualError) => {
    return DateTime.fromTimestampOrErrorCode(value, textualError);
  },

  intoInner: (value) => {
    return DateTime.toTimestamp(value);
  }
});

export const DurationDescriptor = CustomScalarDescriptor.create<Duration, number>({
  name: "Duration",
  descriptor: IntegerDescriptor.create(),
  fromInner: (value, textualError) => {
    return Duration.fromMillisecondsOrErrorCode(value, textualError);
  },
  intoInner: (value) => {
    return Duration.toTotalMilliseconds(value);
  },
});

export const DateDescriptor = CustomScalarDescriptor.create<Date, number>({
  name: "Date",
  descriptor: IntegerDescriptor.create(),
  fromInner: (value, textualError) => {
    return Date.fromTimestampOrError(value, textualError);
  },
  intoInner: (value) => {
    return Date.getTimestamp(value);
  },
});

export const TimeDescriptor = CustomScalarDescriptor.create<Time, number>({
  name: "Time",
  descriptor: IntegerDescriptor.create(),
  fromInner: (value, textualError) => {
    return Time.fromTimestampOrError(value, textualError);
  },
  intoInner: (value) => {
    return Time.getTimestamp(value);
  },
});

export const InstantDescriptor = CustomScalarDescriptor.create<Instant, Duration>({
  name: "Instant",
  descriptor: DurationDescriptor,
  fromInner: (value, textualError) => {
    return Instant.fromElapsedTime(value);
  },
  intoInner: (value) => {
    return Instant.toElapsedTime(value);
  },
});

export interface TimeRangeColumns {
  readonly from: Column,
  readonly till: Column,
}

export const TimeRangeColumns = {
  create: (from: Column, till: Column): TimeRangeColumns => {
    return { from, till };
  },
};

export const TimeRangeDescriptor = CompoundValueReadWrite.create<TimeRange, TimeRangeColumns>({
  name: "TimeRange",

  read: (source, read, Columns, textualError)=> {
    const from = read.readScalarValue(source, Columns.from, TimeDescriptor, textualError);
    if (from === FAILURE) {
      return FAILURE;
    }

    const till = read.readScalarValue(source, Columns.till, TimeDescriptor, textualError);
    if (till === FAILURE) {
      return FAILURE;
    }

    return TimeRange.fromTimes(from, till);
  },

  write: (value, destination, write, Columns, textualError) => {
    let it;
    it = write.writeScalarValue(destination, Columns.from, TimeRange.getFrom(value), TimeDescriptor, textualError);
    if (it === FAILURE) {
      return FAILURE;
    }

    it = write.writeScalarValue(destination, Columns.till, TimeRange.getTill(value), TimeDescriptor, textualError);
    if (it === FAILURE) {
      return FAILURE;
    }

    return SUCCESS;
  },
});

export interface CountdownColumns {
  readonly from: Column,
  readonly duration: Column,
}

export const CountdownColumns = {
  create: (from: Column, duration: Column): CountdownColumns => {
    return {
      from,
      duration,
    };
  },
};

// export const CountdownDescriptor = CompoundValueReadWrite.create<Countdown, CountdownColumns>({
//   name: "Countdown",
  
//   read: (source, read, Columns, textualError) => {
//     const from = read.readScalarValue(source, Columns.from, InstantDescriptor, textualError);
//     if (from === FAILURE) {
//       return FAILURE;
//     }

//     const duration = read.readScalarValue(source, Columns.duration, DurationDescriptor, textualError);
//     if (duration === FAILURE) {
//       return FAILURE;
//     }

//     return Countdown.construct(from, duration);
//   },

//   write: (value, destination, write, Columns, textualError) => {
//     write.writeScalarValue(destination, Countdown.getTotalDuration(value), InstantDescriptor, Columns.from, textualError);
  
//   },
// });

type CountdownProperties = [
  from: Instant,
  duration: Duration,
];

export const CountdownDescriptor = ObjectDescriptor.create<
  Countdown,
  CountdownProperties,
  CountdownColumns
>({
  name: "Countdown",
  
  construct: Countdown.construct,

  properties: [
    ObjectScalarPropertyDescriptor.create({
      name: "from",
      getter: Countdown.getFrom,
      column: it => it.from,
      descriptor: InstantDescriptor,
    }),
    ObjectScalarPropertyDescriptor.create({
      name: "duration",
      getter: Countdown.getTotalDuration,
      column: it => it.duration,
      descriptor: DurationDescriptor,
    }),
  ],
});

type MonotonicClockProperties = [
  elapsedTime: Duration,
  previousSynchronizationTime: DateTime,
];

export type MonotonicClockColumns = {
  readonly elapsedTime: Column,
  readonly previousSynchronizationTime: Column,
};

export const MonotonicClockColumns = {
  create: (initializre: MonotonicClockColumns): MonotonicClockColumns => {
    return initializre;
  },
};

export const MonotonicClockDescriptor = ObjectDescriptor.create<
  MonotonicClock,
  MonotonicClockProperties,
  MonotonicClockColumns
>({
  name: "MonotonicClock",
  
  construct: MonotonicClock.construct,

  properties: [
    ObjectScalarPropertyDescriptor.create({
      name: "elapsedTime",
      getter: MonotonicClock.getElapsedTime,
      column: it => it.elapsedTime,
      descriptor: DurationDescriptor,
    }),
    ObjectScalarPropertyDescriptor.create({
      name: "previousSynchronizationTime",
      getter: MonotonicClock.getPreviousSynchronizationTime,
      column: it => it.previousSynchronizationTime,
      descriptor: DateTimeDescriptor,
    }),
  ],
});

export const VaultNameDescriptor = CustomScalarDescriptor.create<VaultName, string>({
  name: "VaultName",
  descriptor: StringDescriptor.create(),
  intoInner: VaultName.toString,
  fromInner: VaultName.createOrErrorCode,
});

export const VaultDataDescriptor = CustomScalarDescriptor.create<VaultData, string>({
  name: "VaultData",
  descriptor: StringDescriptor.create(),
  intoInner: VaultData.toString,
  fromInner: VaultData.createOrErrorCode,
});

export type VaultProperties = [
  name: VaultName,
  data: VaultData,
  protection: Countdown,
];

export type VaultColumns = {
  name: Column,
  data: Column,
  protection: CountdownColumns,
};

export const VaultColumns = {
  create: (initializre: VaultColumns): VaultColumns => {
    return initializre;
  },
};

export const VaultDescriptor = ObjectDescriptor.create<
  Vault,
  VaultProperties,
  VaultColumns
>({
  name: "Vault",
  construct: Vault.construct,
  properties: [
    ObjectScalarPropertyDescriptor.create({
      name: "name",
      getter: Vault.getName,
      column: columns => columns.name,
      descriptor: VaultNameDescriptor,
    }),
    ObjectScalarPropertyDescriptor.create({
      name: "data",
      getter: Vault.getData,
      column: columns => columns.data,
      descriptor: VaultDataDescriptor,
    }),
    ObjectCompoundPropertyDescriptor.create({
      name: "protection",
      getter: Vault.getProtection,
      descriptor: CountdownDescriptor,
    }),
  ],
});

export type CountdownRuleProperties = [
  isEnabled: boolean,
  countdown: Countdown,
];

export type CountdownRuleColumns = {
  readonly isEnabled: Column,
  readonly countdown: Column,
};

export const CountdownRuleColumns = {
  create: (initializre: CountdownRuleColumns): CountdownRuleColumns => {
    return initializre;
  },
};

export const CountdownRuleDescriptor = ObjectDescriptor.create<
  CountdownRule,
  CountdownRuleProperties,
  CountdownRuleColumns
>({
  name: "CountdownRule",
  construct: CountdownRule.construct,
  properties: [
    ObjectScalarPropertyDescriptor.create({
      name: "isEnabled",
      getter: CountdownRule.getIsEnabled,
      column: columns => columns.isEnabled,
      descriptor: BooleanDescriptor.create(),
    }),
    ObjectCompoundPropertyDescriptor.create({
      name: "countdown",
      getter: CountdownRule.getCountdown,
      descriptor: CountdownDescriptor,
    }),
  ],
});

export type TimeRangeRuleProperties = [
  timeRange: TimeRange,
  lifetime: Countdown,
];

export type TimeRangeRuleColumns = {
  readonly timeRange: TimeRangeColumns,
  readonly lifetime: CountdownColumns,
};

export const TimeRangeRuleColumns = {
  create: (initializre: TimeRangeRuleColumns): TimeRangeRuleColumns => {
    return initializre;
  },
}

export const TimeRangeRuleDescriptor = ObjectDescriptor.create<
  TimeRangeRule,
  TimeRangeRuleProperties,
  TimeRangeColumns
>({
  name: "TimeRangeRule",
  construct: TimeRangeRule.construct,
  properties: [
    ObjectCompoundPropertyDescriptor.create({
      name: "timeRange",
      getter: TimeRangeRule.getCondition,
      descriptor: TimeRangeDescriptor,
    }),
    ObjectCompoundPropertyDescriptor.create({
      name: "lifetime",
      getter: TimeRangeRule.getLifetime,
      descriptor: CountdownDescriptor,
    }),
  ],
});