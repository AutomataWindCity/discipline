// import { Serialization, Countdown, Duration, DateTime, Tried, StateT, TimeRange, Time, UptimeAllowanceConditional, VaultT, Vault, VaultName, VaultData, Option, State } from "../x.ts"

import { Reflect, Tried } from "../x.ts";

// export const Duration_serialize = Serialization.Serialize_implement<Duration.DurationT>({
//   write(value, destination) {
//     Serialization.Destination_writeNumber(destination, Duration.toTotalMilliseconds(value));
//   },
// });

// export const Duration_deserialize = Serialization.Deserialize_implement<Duration.DurationT>({
//   read(source) {
//     const totalMilliseconds = Serialization.Source_readInteger(source);
//     if (totalMilliseconds) {
//       return Serialization.Failure();
//     }
    
//     const durationOrError = Duration.fromMilliseconds(totalMilliseconds);
//     if (Tried.isFailure(durationOrError)) {
//       return Serialization.Failure();
//     }

//     return Tried.value(durationOrError);
//   },
// });

// export const DateTime_serialize = Serialization.Serialize_implement<DateTime.DateTimeT>({
//   write(value, destination) {
//     Serialization.Destination_writeNumber(destination, DateTime.timestamp(value));
//   },
// });

// export const DateTime_deserialize = Serialization.Deserialize_implement<DateTime.DateTimeT>({
//   read(source) {
//     const timestamp = Serialization.Source_readInteger(source);
//     if (timestamp === Serialization.Failure()) {
//       return Serialization.Failure();
//     }

//     const datetimeOrError = DateTime.fromTimestamp(timestamp);
//     if (Tried.isFailure(datetimeOrError)) {
//       return Serialization.Failure();
//     }

//     return Tried.value(datetimeOrError);
//   },
// });

// export const Countdown_serialize = Serialization.Serialize_implement<Countdown.CountdownT>({
//   write(value, destination) {
//     Serialization.Destination_writeValue(destination, value.remainingDuration, Duration_serialize);
//     Serialization.Destination_writeValue(destination, value.previousSynchronizationTime, DateTime_serialize);
//   },
// });

// export const Countdown_deserialize = Serialization.Deserialize_implement<Countdown.CountdownT>({
//   read(source) {
//     const remainingDuration = Serialization.Source_readValue(source, Duration_deserialize);
//     if (remainingDuration === Serialization.Failure()) {
//       return Serialization.Failure();
//     }

//     const previousSynchronizationTime = Serialization.Source_readValue(source, DateTime_deserialize);
//     if (previousSynchronizationTime === Serialization.Failure()) {
//       return Serialization.Failure();
//     }

//     return Countdown.construct(
//       remainingDuration,
//       previousSynchronizationTime,
//     );
//   },
// });

// export const Time_serialize = Serialization.Serialize_implement<Time.Time>({
//   write(value, destination) {
//     Serialization.Destination_writeNumber(destination, Time.timestamp(value));    
//   },
// });

// export const Time_deserialize = Serialization.Deserialize_implement<Time.Time>({
//   read(source) {
//     const timestamp = Serialization.Source_readInteger(source);
//     if (timestamp === Serialization.Failure()) {
//       return Serialization.Failure();
//     }

//     const time = Time.fromTimestampOrNone(timestamp);
//     if (Option.isNone(time)) {
//       return Serialization.Failure();
//     }

//     return Option.value(time);
//   },
// })

// export const TimeRange_serialize = Serialization.Serialize_implement<TimeRange.TimeRange>({
//   write(value, destination) {
//     Serialization.Destination_writeValue(destination, TimeRange.getFrom(value), Time_serialize);
//     Serialization.Destination_writeValue(destination, TimeRange.getTill(value), Time_serialize);

//     // Serialization.Destination_writeNumber(destination, TimeRange.getFromTimestamp(value));
//     // Serialization.Destination_writeNumber(destination, TimeRange.getTillTimestamp(value));
//   },
// });

// export const TimeRange_deserialize = Serialization.Deserialize_implement<TimeRange.TimeRange>({
//   read(source) {
//     const from = Serialization.Source_readValue(source, Time_deserialize);
//     if (from === Serialization.Failure()) {
//       return Serialization.Failure();
//     }

//     const till = Serialization.Source_readValue(source, Time_deserialize);
//     if (till === Serialization.Failure()) {
//       return Serialization.Failure();
//     }

//     return TimeRange.fromTimes(
//       from,
//       till,
//     );
//   },
// });

// export const UptimeAllowance_serialize = Serialization.Serialize_implement<UptimeAllowance.UptimeAllowance>({
//   write(value, destination) {
//     Serialization.Destination_writeValue(destination, UptimeAllowanceConditional.getAllowance(value), Duration_serialize);
//     Serialization.Destination_writeValue(destination, UptimeAllowanceConditional.getCountdown(value), Countdown_serialize);
//   }, 
// });

// export const UptimeAllowance_deserialize = Serialization.Deserialize_implement<UptimeAllowance.UptimeAllowance>({
//   read(source) {
//     const duration = Serialization.Source_readValue(source, Duration_deserialize);
//     if (duration === Serialization.Failure()) {
//       return Serialization.Failure();
//     }

//     const countdown = Serialization.Source_readValue(source, Countdown_deserialize);
//     if (countdown === Serialization.Failure()) {
//       return Serialization.Failure();
//     }

//     return UptimeAllowanceConditional.construct(
//       duration,
//       countdown,
//     );
//   },
// });

// export const Vault_serialize = Serialization.Serialize_implement<VaultT>({
//   write(value, destination) {
//     Serialization.Destination_writeString(destination, VaultName.toString(Vault.getName(value)));
//     Serialization.Destination_writeString(destination, VaultData.toString(Vault.getData(value)));
//     Serialization.Destination_writeValue(destination, Vault.getProtection(value), Countdown_serialize);
//   },
// });

// export const Vault_deserialize = Serialization.Deserialize_implement<VaultT>({
//   read(source) {
//     const nameAsString = Serialization.Source_readString(source);
//     if (nameAsString === Serialization.Failure()) {
//       return Serialization.Failure();
//     }

//     const name = VaultName.create(nameAsString);
//     if (Tried.isFailure(name)) {
//       return Serialization.Failure();
//     }

//     const dataAsString = Serialization.Source_readString(source);
//     if (dataAsString === Serialization.Failure()) {
//       return Serialization.Failure();
//     }

//     const data = VaultData.create(dataAsString);
//     if (Tried.isFailure(data)) {
//       return Serialization.Failure();
//     }

//     const protection = Serialization.Source_readValue(source, Countdown_deserialize);
//     if (protection === Serialization.Failure()) {
//       return Serialization.Failure();
//     }

//     return Vault.construct(
//       Tried.value(name),
//       Tried.value(data),
//       protection,
//     );
//   },  
// });

// export const State_serialize = Serialization.Serialize_implement<StateT>({
//   write(value, destination) {
//     Serialization.Destination_writeValue(destination, value.denyDeviceAccessCountdown, Countdown_serialize);
//     Serialization.Destination_writeValue(destination, value.denyDeviceAccessTimeRange, TimeRange_serialize);
//     Serialization.Destination_writeValue(destination, value.denyDeviceAccessUptimeAllowance, UptimeAllowance_serialize);
//     Serialization.Destination_writeValue(destination, value.denyDeviceAccessUptimeAllowance, UptimeAllowance_serialize);
//     Serialization.Destination_writeArray(destination, value.vaults, Vault_serialize);
//   },
// });


// export const State_deserialize = Serialization.Deserialize_implement<StateT>({
//   read(source) {
//     const denyDeviceAccessCountdown = Serialization.Source_readValue(source, Countdown_deserialize);
//     if (denyDeviceAccessCountdown === Serialization.Failure()) {
//       return Serialization.Failure();
//     }

//     const denyDeviceAccessTimeRange = Serialization.Source_readValue(source, TimeRange_deserialize);
//     if (denyDeviceAccessTimeRange === Serialization.Failure()) {
//       return Serialization.Failure();
//     }

//     const denyDeviceAccessUptimeAllowance = Serialization.Source_readValue(source, UptimeAllowance_deserialize);
//     if (denyDeviceAccessUptimeAllowance === Serialization.Failure()) {
//       return Serialization.Failure();
//     }

//     const vaults = Serialization.Source_readArray(source, Vault_deserialize);
//     if (vaults === Serialization.Failure()) {
//       return Serialization.Failure();
//     }

//     return State.construct(
//       denyDeviceAccessTimeRange,
//       denyDeviceAccessCountdown,
//       denyDeviceAccessUptimeAllowance,
//       vaults,
//     );
//   },
// });


const serialize = <Value>(
  value: Value, 
  reflection: Reflection<Value>,
) => {
  if (Reflection.isNull(reflection)) {}
  if (Reflection.isNumber(reflection)) {}
  if (Reflection.isArray(reflection)) {
    const itemReflection = ArrayReflection.getItemReflection(reflection);
    if (Reflection.isNull()) {}
  }
  if (Reflection.isObject(reflection)) {
    const properties = ObjectReflection.getProperties(reflection);
    for (const [ name, valueReflection ] of properties) {

    }
  }
  if (Reflection.isWrapper(reflection)) {
    
  }

  // if (reflection.brand === Reflect.Brand.Null) {
  //   if ()
  //   return null;
  // }
  // if (reflection.brand === Reflect.Brand.Undefined) {
  //   throw new Error("");
  // }
  // if (reflection.brand === Reflect.Brand.Number) {
  //   if (Number.isFinite(value)) {
  //     return value;
  //   } else {
  //     throw new Error("");
  //   }
  // }
  // if (reflection.brand === Reflect.Brand.Integer) {
  //   if (Number.isSafeInteger(value)) {
  //     return value;
  //   } else {
  //     throw new Error("");
  //   }
  // }
  // if (reflection.brand === Reflect.Brand.Boolean) {
  //   if (typeof )
  // }

  if (Reflect.Reflect.isNull(reflection)) {
    return null;
  }
  // Reflect.Reflect.match2(reflection, value, {
  //   Null: (_, value) => value,
  //   Undefined: () => { throw new Error(""); },
  //   Number: (_, value) => value,
  // });
};

// const durationReflection = Wrapper.createFromNamedArguments<Duration, number>({
//   name: "Duration",
//   innerGetter: it => it.toTotalMilliseconds(),
//   innerReflection: Number.create(),
// });

// const instantReflection = Wrapper.createFromNamedArguments<Instant, Duration>({
//   name: "Instant",
//   innerGetter: it => it.toElapsedTime(),
//   innerReflection: durationReflection,
// });

// const countdownReflection = Class.createFromNamedArguments({
//   name: "Countdown",
//   reflection: pipe(
//     Object.createEmpty<Countdown>(),
//     it => Object.withProperty(it, "from", it => it.getFrom(), instantReflection),
//     it => Object.withProperty(it, "till", it => it.getTill(), instantReflection),
//   ),
// });
