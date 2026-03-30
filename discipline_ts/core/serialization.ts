import { 
  Countdown, Data1, DateTime, TextualError, Duration, 
  VaultName, isInteger, 
  UptimeAllowanceConditional, Vault, VaultData, 
  Instant, UserRegulation, Conditional,
  ConditionalTag,
  TimeRange,
  Time,
  TimeRangeConditional,
  CountdownConditional,
  AnyConditionalTag,
  ConditionalTypeCountdown,
  ConditionalTypeTimeRange,
  ConditionalTypeUptimeAllowance,
  Data2,
  UptimeClock,
  MonotonicClock,
} from "./x.ts";

const isString = (it: unknown): it is string => {
  return typeof it === "string";
};

const isBoolean = (it: unknown): it is boolean => {
  return typeof it === "boolean";
};

const FAILURE = Symbol();
type Failure = typeof FAILURE;

const SUCCESS = Symbol();
type Success = typeof SUCCESS;

type JsonPrimitive = null | number | boolean | string;
type JsonArray = JsonValue[];
type JsonObject = { [key: string]: JsonValue }; 
type JsonValue = JsonPrimitive | JsonArray | JsonObject;

class Writer {
  private constructor(readonly data: JsonPrimitive[]) {}

  static create() {
    return new Writer([]);
  }

  writeInteger(value: number): Success | TextualError {
    if (Number.isInteger(value)) {
      this.data.push(value);
      return SUCCESS;
    }

    return TextualError
      .create("Writer writing an integer")
      .addMessage("Value is not an integer")
      .addNumberAttachment("Value", value);
  }

  writeString(value: string): Success | TextualError {
    this.data.push(value);
    return SUCCESS;
  }

  writeBoolean(value: boolean): Success | TextualError {
    this.data.push(value);
    return SUCCESS;
  }

  writeArray<Item>(array: Item[], itemSerialize: Serialize<Item>): Success | TextualError {
    const error = this.writeInteger(array.length);
    if (error !== SUCCESS) {
      return error
        .changeContext("Writer writing the array length")
        .changeContext("Writer writing an array");
    }

    for (const item of array) {
      const error = itemSerialize.write(item, this);
      if (error !== SUCCESS) {
        return error
          .changeContext("Writer writing an array item")
          .changeContext("Writer writing an array");
      }
    }

    return SUCCESS;
  }

  write<Value>(value: Value, valueSerialize: Serialize<Value>): Success | TextualError {
    return valueSerialize.write(value, this);
  }
}

class Reader {
  private constructor(
    private data: JsonPrimitive[],
    private index: number,
  ) {}

  static create(data: JsonPrimitive[]) {
    return new Reader(data, 0);
  }

  readInteger(): number | TextualError {
    if (this.index >= this.data.length) {
      return TextualError
        .create("Reader reading an integer")
        .addMessage("There are no more values to read");
    }

    const item = this.data[this.index];
    if (!isInteger(item)) {
      return TextualError
        .create("Reader reading an integer")
        .addMessage("Value is not an integer")
        .addPrimitiveAttachment("Found value", item);
    }

    this.index += 1;
    return item;
  }

  readString(): string | TextualError {
    if (this.index >= this.data.length) {
      return TextualError
        .create("Reader reading a string")
        .addMessage("There are no more values to read");
    }

    const item = this.data[this.index];
    if (!isString(item)) {
      return TextualError
        .create("Reader reading a string")
        .addMessage("Value is not a string")
        .addPrimitiveAttachment("Found value", item);
    }

    this.index += 1;
    return item;
  }

  readBoolean(): boolean | TextualError {
    if (this.index >= this.data.length) {
      return TextualError
        .create("Reader reading a boolean")
        .addMessage("There are no more values to read");
    }

    const item = this.data[this.index];
    if (!isBoolean(item)) {
      return TextualError
        .create("Reader reading a boolean")
        .addMessage("Value is not a boolean")
        .addPrimitiveAttachment("Found value", item);
    }

    this.index += 1;
    return item;
  }

  readArray<Item>(read: Deserialize<Item>): Item[] | TextualError {
    const lengthOrError = this.readInteger();
    if (TextualError.is(lengthOrError)) {
      return lengthOrError
        .changeContext("Reader reading the array length")
        .changeContext("Reader reading an array");
    }

    if (lengthOrError < 0) {
      return TextualError
        .create("Reader reading the array length")
        .addMessage("Array length is negative")
        .changeContext("Reader reading an array");
    }
   
    const array: Item[] = [];
    let remainingItems = lengthOrError;
    while (remainingItems > 0) {
      const itemOrError = read.read(this);
      if (TextualError.is(itemOrError)) {
        return itemOrError
          .changeContext("Reader reading an array item")
          .changeContext("Reader reading an array");
      }

      array.push(itemOrError);
      remainingItems -= 1;
    }
    
    return array;
  }

  read<Value>(deserialize: Deserialize<Value>) {
    return deserialize.read(this);
  }
}

export class Serialize<Value> {
  private constructor(
    readonly write: (value: Value, writer: Writer) => Success | TextualError,
  ) {}

  static implement<Value>({
    write,
  }: {
    write: (value: Value, writer: Writer) => Success | TextualError,
  }) {
    return new Serialize(write);
  }

  serialize(value: Value): string | TextualError {
    const writer = Writer.create();
    this.write(value, writer);

    try {
      return JSON.stringify(writer.data);
    } catch (error) {
      return TextualError 
        .create("Serializing a value")
        .addMessage("An exception was thrown by 'JSON.stringify'")
        .addStringAttachment("Exception", String(error));
    }
  }
}

export class Deserialize<Value> {
  private constructor(
    readonly read: (reader: Reader) => Value | TextualError,
  ) {}

  static implement<T>({
    read,
  }: {
    read: (reader: Reader) => T | TextualError
  }) {
    return new Deserialize(read);
  }

  deserialize(jsonText: string): Value | TextualError {
    let data;

    try {
      data = JSON.parse(jsonText);
    } catch (error) {
      return TextualError 
        .create("Deserializing a value")
        .addMessage("An exception was thrown by 'JSON.parse'")
        .addStringAttachment("Exception", String(error))
        .addStringAttachment("Json text", jsonText);
    }

    if (!Array.isArray(data)) {
      return TextualError
        .create("Deserializing a value")
        .addMessage("Data is not an array")
        .addStringAttachment("Json text", jsonText);
    }

    return this.read(Reader.create(data));
  }
}

export const DateTimeSerialize = Serialize.implement<DateTime>({
  write(value, writer) {
    const error = writer.writeInteger(value.toTimestamp());
    if (error !== SUCCESS) {
      return error.changeContext("Serializing 'DateTime' as a millisecond timestamp")
    } else {
      return SUCCESS;
    }
  },
});

export const DateTimeDeserialize = Deserialize.implement<DateTime>({
  read(reader) {
    const timestamp = reader.readInteger();
    if (TextualError.is(timestamp)) {
      return timestamp.changeContext("Deserializing 'DateTime', which is serialized as a millisecond timestamp")
    }

    const datetime = DateTime.fromTimestamp(timestamp);
    if (datetime instanceof Error) {
      return TextualError 
        .create("Constructing 'DateTime' from deserialized millisecond timestamp integer")
        .addErrorAttachment("Error", datetime)
        .changeContext("Deserializing 'DateTime', which is serialized as a millisecond timestamp");
    }

    return datetime;
  },
});

export const DurationSerialize = Serialize.implement<Duration>({
  write(value, writer) {
    const error = writer.writeInteger(value.toTotalMilliseconds());
    if (error !== SUCCESS) {
      return error.changeContext("Serializing 'Duration' as a number of total milliseconds")
    } else {
      return SUCCESS;
    }
  },
});

export const DurationDeserialize = Deserialize.implement<Duration>({
  read(reader) {
    const milliseconds = reader.readInteger();
    if (TextualError.is(milliseconds)) {
      return milliseconds
        .changeContext("Deserializing 'Duration', which is serialized as a total milliseconds integer")
        .changeContext("Deserializing 'Duration'");
    }

    const duration = Duration.fromMilliseconds(milliseconds);
    if (duration instanceof Error) {
      return TextualError
        .create("Constructing a 'Duration' from milliseconds")
        .addErrorAttachment("Error", duration)
        .changeContext("Deserializing 'Duration'");
    }

    return duration;
  },
});

export const InstantSerialize = Serialize.implement<Instant>({
  write(value, writer) {
    const error = writer.write(value.toElapsedTime(), DurationSerialize);
    if (error === SUCCESS) {
      return SUCCESS;
    } else {
      return error.changeContext("Serializing 'elapsedTime' field of 'Instant'");
    }
  },
});

export const InstantDeserialize = Deserialize.implement<Instant>({
  read(reader) {
    const elapsedTime = reader.read(DurationDeserialize);
    if (TextualError.is(elapsedTime)) {
      return TextualError
        .create("Deserializing 'elapsedTime' field")
        .changeContext("Deserializing 'Instant'");
    }
    
    return Instant.fromElapsedTime(elapsedTime);
  },
});

export const CountdownSerialize = Serialize.implement<Countdown>({
  write(value, writer) {
    let it;

    it = writer.write(value.getFrom(), InstantSerialize);
    if (TextualError.is(it)) {
      return it.changeContext("Serializing 'from' field of 'Countdown'");
    }
    
    it = writer.write(value.getTotalDuration(), DurationSerialize);
    if (TextualError.is(it)) {
      return it.changeContext("Serializing 'totalDuration' field of 'Countdown'");
    }

    return SUCCESS;
  },
});

export const CountdownDeserialize = Deserialize.implement<Countdown>({
  read(reader) {
    const from = reader.read(InstantDeserialize);
    if (TextualError.is(from)) {
      return from.changeContext("Deserializing 'from' field of 'Countdown'");
    }

    const totalDuration = reader.read(DurationDeserialize);
    if (TextualError.is(totalDuration)) {
      return totalDuration.changeContext("Deserializing 'totalDuration' field of 'Countdown'");
    }

    return Countdown.construct(from, totalDuration);
  },
});

export const TimeSerialize = Serialize.implement<Time>({
  write(value, writer) {
    const it = writer.writeInteger(value.getTimestamp());
    if (it !== SUCCESS) {
      return it.changeContext("Serializing 'timestamp' field of 'Time'");
    }

    return SUCCESS;
  },
});

export const TimeDeserialize = Deserialize.implement<Time>({
  read(reader) {
    const timestamp = reader.readInteger();
    if (TextualError.is(timestamp)) {
      return timestamp.changeContext("Deserializing 'timestamp' field of 'Time'");
    }

    const time = Time.fromTimestamp(timestamp);
    if (TextualError.is(time)) {
      return time.changeContext("Constructing 'Time' from deserialized fields");
    }

    return time;
  },
});

export const TimeRangeSerialize = Serialize.implement<TimeRange>({
  write(value, writer) {
    let it;

    it = writer.write(value.getFrom(), TimeSerialize);
    if (it !== SUCCESS) {
      return it.changeContext("Serializing 'from' field of 'TimeRange'");
    }

    it = writer.write(value.getTill(), TimeSerialize);
    if (it !== SUCCESS) {
      return it.changeContext("Serializing 'till' field of 'TimeRange'");
    }
    
    return SUCCESS;
  },
});

export const TimeRangeDeserialize = Deserialize.implement<TimeRange>({
  read(reader) {
    const from = reader.read(TimeDeserialize);
    if (TextualError.is(from)) {
      return from.changeContext("Deserializing 'from' field of 'TimeRange'");
    }

    const till = reader.read(TimeDeserialize);
    if (TextualError.is(till)) {
      return till.changeContext("Deserializing 'till' field of 'TimeRange'");
    }

    return TimeRange.fromTimes(from, till);
  },
});

export const CountdownConditionalSerialize = Serialize.implement<CountdownConditional>({
  write(value, writer) {
    const error = writer.write(value.countdown, CountdownSerialize);
    if (error === SUCCESS) {
      return SUCCESS;
    } else {
      return error.changeContext("Serializing 'countdown' field of 'CountdownConditional'");
    }
  },
});

export const CountdownConditionalDeserialize = Deserialize.implement<CountdownConditional>({
  read(reader) {
    const countdown = reader.read(CountdownDeserialize);
    if (TextualError.is(countdown)) {
      return countdown.changeContext("Deserializing 'countdown' field of 'CountdownConditional'");
    }

    return CountdownConditional.construct(countdown);
  },
});

export const TimeRangeConditionalSerialize = Serialize.implement<TimeRangeConditional>({
  write(value, writer) {
    let it;

    it = writer.write(value.timeRange, TimeRangeSerialize);
    if (it !== SUCCESS) {
      return it.changeContext("Serializing 'timeRange' field of 'TimeRangeConditional'");
    }

    it = writer.write(value.lifetime, CountdownSerialize);
    if (it !== SUCCESS) {
      return it.changeContext("Serializing 'lifetime' field of 'TimeRangeConditional'");
    }

    return SUCCESS;
  },
});

export const TimeRangeConditionalDeserialize = Deserialize.implement<TimeRangeConditional>({
  read(reader) {
    const timeRange = reader.read(TimeRangeDeserialize);
    if (TextualError.is(timeRange)) {
      return timeRange.changeContext("Deserializing 'timeRange' field of 'TimeRangeConditional'");
    }
    
    const lifetime = reader.read(CountdownDeserialize);
    if (TextualError.is(lifetime)) {
      return lifetime.changeContext("Deserializing 'lifetime' field of 'TimeRangeConditional'");
    }

    return TimeRangeConditional.construct(timeRange, lifetime);
  },
});

export const UptimeAllowanceConditionalSerialize = Serialize.implement<UptimeAllowanceConditional>({
  write(value, writer) {
    let it;
    
    it = writer.write(value.getTotalAllowance(), DurationSerialize);
    if (it !== SUCCESS) {
      return it.changeContext("Serializing 'totalAllowance' field of 'UptimeAllowanceConditional'");
    }

    it = writer.write(value.getLifetime(), CountdownSerialize);
    if (it !== SUCCESS) {
      return it.changeContext("Serializing 'lifetime' field of 'UptimeAllowanceConditional'");
    }

    return SUCCESS;
  },
});

export const UptimeAllowanceConditionalDeserialize = Deserialize.implement<UptimeAllowanceConditional>({
  read(reader) {
    const totalAllowance = reader.read(DurationDeserialize);
    if (TextualError.is(totalAllowance)) {
      return totalAllowance.changeContext("Deserializing 'totalAllowance' field of 'UptimeAllowanceConditional'");
    }

    const lifetime = reader.read(CountdownDeserialize);
    if (TextualError.is(lifetime)) {
      return lifetime.changeContext("Deserializing 'lifetime' field of 'UptimeAllowanceConditional'");
    }

    return UptimeAllowanceConditional.construct(
      totalAllowance,
      lifetime,
    );
  },
});

export const ConditionalTypeSerialize = Serialize.implement<AnyConditionalTag>({
  write(value, writer) {
    const error = writer.writeInteger(value.asNumber);
    if (error === SUCCESS) {
      return SUCCESS;
    } else {
      return error
        .changeContext("Serializing 'ConditionalTag'")
        .addStringAttachment("Variant name", value.asString)
        .addNumberAttachment("Variant number", value.asNumber);
    }
  },
});

export const ConditionalTagDeserialize = Deserialize.implement<ConditionalTag>({
  read(reader) {
    const integer = reader.readInteger();
    if (TextualError.is(integer)) {
      return integer.changeContext("Deserialiing 'ConditionalTag'");
    }
    switch (integer) {
      case ConditionalTypeCountdown.it.asNumber: {
        return ConditionalTag.Countdown;
      }
      case ConditionalTypeTimeRange.it.asNumber: {
        return ConditionalTag.TimeRange;
      }
      case ConditionalTypeUptimeAllowance.it.asNumber: {
        return ConditionalTag.UptimeAllowance;
      }
      default: {
        return TextualError
          .create("Deserializing 'ConditionalTag'")
          .addMessage("Unrecognized varint number")
          .addNumberAttachment("Value", integer);
      }
    }
  },
});

export const ConditionalSerialize = Serialize.implement<Conditional>({
  write(value, writer) {
    let it;
    
    it = writer.write(value.type, ConditionalTypeSerialize);
    if (it !== SUCCESS) {
      return it.changeContext("Serializing 'type' field of 'Conditional'");
    }

    switch (value.tag) {
      case ConditionalTag.Countdown: {
        it = writer.write(value, CountdownConditionalSerialize);
        if (it === SUCCESS) {
          return SUCCESS;
        } else {
          return it.changeContext("Serializing 'Conditional'");
        }
      }
      case ConditionalTag.TimeRange: {
        it = writer.write(value, TimeRangeConditionalSerialize);
        if (it === SUCCESS) {
          return SUCCESS;
        } else {
          return it.changeContext("Serializing 'Conditional'");
        }
      }
      case ConditionalTag.UptimeAllowance: {
        it = writer.write(value, UptimeAllowanceConditionalSerialize);
        if (it === SUCCESS) {
          return SUCCESS;
        } else {
          return it.changeContext("Serializing 'Conditional'");
        }
      }
    }
  },
});

export const ConditionalDeserialize = Deserialize.implement<Conditional>({
  read(reader) {
    const tag = reader.read(ConditionalTagDeserialize);
    if (TextualError.is(tag)) {
      return tag.changeContext("Deserializing 'tag' field of 'Conditional'");
    }

    let it;
    switch (tag) {
      case ConditionalTag.Countdown: {
        it = reader.read(CountdownConditionalDeserialize);
        if (TextualError.is(it)) {
          return it.changeContext("Deserializing 'Countdown' variant of 'Conditional'");
        } else {
          return it;
        }
      }
      case ConditionalTag.TimeRange: {
        it = reader.read(TimeRangeConditionalDeserialize);
        if (TextualError.is(it)) {
          return it.changeContext("Deserializing 'TimeRange' variant of 'Conditional'");
        } else {
          return it;
        }
      }
      case ConditionalTag.UptimeAllowance: {
        it = reader.read(UptimeAllowanceConditionalDeserialize);
        if (TextualError.is(it)) {
          return it.changeContext("Deserializing 'UptimeAllowance' variant of 'Conditional'");
        } else {
          return it;
        }
      }
    }
  },
});

export const VaultNameSerialize = Serialize.implement<VaultName>({
  write(value, writer) {
    const error = writer.writeString(value.toString());
    if (error === SUCCESS) {
      return SUCCESS;
    } else {
      return error.changeContext("Serializing 'VaultName'");
    }
  },
});

export const VaultNameDeserialize = Deserialize.implement<VaultName>({
  read(reader) {
    const string = reader.readString();
    if (TextualError.is(string)) {
      return string.changeContext("Deserializing 'VaultName'");
    }
    
    const vaultName = VaultName.create(string);
    if (vaultName instanceof Error) {
      return TextualError
        .create("Constructing 'VaultName' from deserialized string")
        .addErrorAttachment("Error", vaultName)
        .changeContext("Deserializing 'VaultName'");
    }

    return vaultName;
  },
});

export const VaultDataSerialize = Serialize.implement<VaultData>({
  write(value, writer) {
    const error = writer.writeString(value.toString());
    if (error === SUCCESS) {
      return SUCCESS;
    } else {
      return error.changeContext("Serializing 'VaultData'");
    }
  },
});

export const VaultDataDeserialize = Deserialize.implement<VaultData>({
  read(reader) {
    const string = reader.readString();
    if (TextualError.is(string)) {
      return string.changeContext("Deserializing 'VaultData'");
    }
    
    const vaultData = VaultData.create(string);
    if (vaultData instanceof Error) {
      return TextualError
        .create("Constructing 'VaultData' from deserialized string")
        .addErrorAttachment("Error", vaultData)
        .changeContext("Deserializing 'VaultData'");
    }

    return vaultData;
  },
});

export const VaultSerialize = Serialize.implement<Vault>({
  write(value, writer) {
    let it;

    it = writer.write(value.getName(), VaultNameSerialize);
    if (it !== SUCCESS) {
      return it
        .changeContext("Serializing 'name' field of 'Vault'")
        .changeContext("Serializing 'Vault'");
    }

    it = writer.write(value.getData(), VaultDataSerialize);
    if (it !== SUCCESS) {
      return it
        .changeContext("Serializing 'data' field of 'Vault'")
        .changeContext("Serializing 'Vault'");
    }


    it = writer.write(value.getProtection(), CountdownSerialize);
    if (it !== SUCCESS) {
      return it
        .changeContext("Serializing 'protection' field of 'Vault'")
        .changeContext("Serializing 'Vault'");
    }

    return SUCCESS;
  },
});

export const VaultDeserialize = Deserialize.implement<Vault>({
  read(reader) {
    const name = reader.read(VaultNameDeserialize);
    if (TextualError.is(name)) {
      return name
        .changeContext("Deserializing 'name' field of 'Vault'")
        .changeContext("Deserializing 'Vault'");
    }

    const data = reader.read(VaultDataDeserialize);
    if (TextualError.is(data)) {
      return data
        .changeContext("Deserializing 'data' field of 'Vault'")
        .changeContext("Deserializing 'Vault'");
    }

    const protection = reader.read(CountdownDeserialize);
    if (TextualError.is(protection)) {
      return protection
        .changeContext("Deserializing 'protection' field of 'Vault'")
        .changeContext("Deserializing 'Vault'");
    }

    return Vault.construct(
      name,
      data,
      protection,
    );
  },
});

export const UserRegulationSerialize = Serialize.implement<UserRegulation>({
  write(value, writer) {
    let it;
    
    it = writer.writeArray(value.screen, ConditionalSerialize);
    if (it !== SUCCESS) {
      return it
        .changeContext("Serializing 'screen' field of 'UserRegulation'")
        .changeContext("Serializing 'UserRegulation'");
    }

    it = writer.writeArray(value.internet, ConditionalSerialize);
    if (it !== SUCCESS) {
      return it
        .changeContext("Serializing 'internet' field of 'UserRegulation'")
        .changeContext("Serializing 'UserRegulation'");
    }

    return SUCCESS;
  },
});

export const UserRegulationDeserialize = Deserialize.implement<UserRegulation>({
  read(reader) {
    const screen = reader.readArray(ConditionalDeserialize);
    if (TextualError.is(screen)) {
      return screen 
        .changeContext("Deserializing 'screen' field of 'UserRegulation'")
        .changeContext("Deserializing 'UserRegulation'");
    } 

    const internet = reader.readArray(ConditionalDeserialize);
    if (TextualError.is(internet)) {
      return internet 
        .changeContext("Deserializing 'internet' field of 'UserRegulation'")
        .changeContext("Deserializing 'UserRegulation'");
    } 

    return UserRegulation.construct(screen, internet);
  },
});

export const UptimeClockSerialize = Serialize.implement<UptimeClock>({
  write(value, writer) {
    let it;

    it = writer.write(value.getDailyUptime(), DurationSerialize);
    if (it !== SUCCESS) {
      return it
        .changeContext("Serializing 'dailyUptime' field of 'UptimeClock'")
        .changeContext("Serializing 'UptimeClock'");
    }

    it = writer.write(value.getPreviousSynchronizationTime(), DateTimeSerialize);
    if (it !== SUCCESS) {
      return it
        .changeContext("Serializing 'previousSynchronizationTime' field of 'UptimeClock'")
        .changeContext("Serializing 'UptimeClock'");
    }

    return SUCCESS;
  },
});

export const UptimeClockDeserialize = Deserialize.implement<UptimeClock>({
  read(reader) {
    const dailyUptime = reader.read(DurationDeserialize);
    if (TextualError.is(dailyUptime)) {
      return dailyUptime
        .changeContext("Deserializing 'dailyUptime' field of 'UptimeClock'")
        .changeContext("Deserializing 'UptimeClock'");
    }

    const previousSynchronizationTime = reader.read(DateTimeDeserialize);
    if (TextualError.is(previousSynchronizationTime)) {
      return previousSynchronizationTime
        .changeContext("Deserializing 'previousSynchronizationTime' field of 'UptimeClock'")
        .changeContext("Deserializing 'UptimeClock'");
    }

    return UptimeClock.construct(
      dailyUptime,
      previousSynchronizationTime,
    );
  },
});

export const MonotonicClockSerialize = Serialize.implement<MonotonicClock>({
  write(value, writer) {
    let it;

    it = writer.write(value.getElapsedTime(), DurationSerialize);
    if (it !== SUCCESS) {
      return it
        .changeContext("Serializing 'elapsedTime' field of 'MonotonicClock'")
        .changeContext("Serializing 'MonotonicClock'");
    }

    it = writer.write(value.getPreviousSynchronizationTime(), DateTimeSerialize);
    if (it !== SUCCESS) {
      return it
        .changeContext("Serializing 'previousSynchronizationTime' field of 'MonotonicClock'")
        .changeContext("Serializing 'MonotonicClock'");
    }

    return SUCCESS;
  },
});

export const MonotonicClockDeserialize = Deserialize.implement<MonotonicClock>({
  read(reader) {
    const elapsedTime = reader.read(DurationDeserialize);
    if (TextualError.is(elapsedTime)) {
      return elapsedTime
        .changeContext("Deserializing 'elapsedTime' field of 'MonotonicClock'")
        .changeContext("Deserializing 'MonotonicClock'");
    }

    const previousSynchronizationTime = reader.read(DateTimeDeserialize);
    if (TextualError.is(previousSynchronizationTime)) {
      return previousSynchronizationTime
        .changeContext("Deserializing 'previousSynchronizationTime' field of 'UptimeClock'")
        .changeContext("Deserializing 'UptimeClock'");
    }

    return MonotonicClock.construct(
      elapsedTime,
      previousSynchronizationTime,
    );
  },
});


export const Data1Serialize = Serialize.implement<Data1>({
  write(value, writer) {
    let it;
    
    it = writer.write(value.luny, UserRegulationSerialize);
    if (it !== SUCCESS) {
      return it 
        .changeContext("Serializing 'luny' field of 'Data1'")
        .changeContext("Serializing 'Data1'");
    }

    it = writer.write(value.ruru, UserRegulationSerialize);
    if (it !== SUCCESS) {
      return it 
        .changeContext("Serializing 'ruru' field of 'Data1'")
        .changeContext("Serializing 'Data1'");
    }

    it = writer.writeArray(value.vaults, VaultSerialize);
    if (it !== SUCCESS) {
      return it 
        .changeContext("Serializing 'vaults' field of 'Data1'")
        .changeContext("Serializing 'Data1'");
    }

    return SUCCESS;
  },
});

export const Data1Deserialize = Deserialize.implement<Data1>({
  read(reader) {
    const luny = reader.read(UserRegulationDeserialize);
    if (TextualError.is(luny)) {
      return luny 
        .changeContext("Deserializing 'luny' field of 'Data1'")
        .changeContext("Deserializing 'Data1'");
    }

    const ruru = reader.read(UserRegulationDeserialize);
    if (TextualError.is(ruru)) {
      return ruru 
        .changeContext("Deserializing 'ruru' field of 'Data1'")
        .changeContext("Deserializing 'Data1'");
    }

    const vaults = reader.readArray(VaultDeserialize);
    if (TextualError.is(vaults)) {
      return vaults 
        .changeContext("Deserializing 'vaults' field of 'Data1'")
        .changeContext("Deserializing 'Data1'");
    }

    return Data1.construct(
      luny,
      ruru, 
      vaults,
    );
  },
});

export const Data2Serialize = Serialize.implement<Data2>({
  write(value, writer) {
    let it;

    it = writer.write(value.uptimeClock, UptimeClockSerialize);
    if (it !== SUCCESS) {
      return it 
        .changeContext("Serializing 'uptimeClock' field of 'Data2'")
        .changeContext("Serializing 'Data2'");
    }

    it = writer.write(value.monotonicClock, MonotonicClockSerialize);
    if (it !== SUCCESS) {
      return it 
        .changeContext("Serializing 'monotonicClock' field of 'Data2'")
        .changeContext("Serializing 'Data2'");
    }

    return SUCCESS;
  },
});

export const Data2Deserialize = Deserialize.implement<Data2>({
  read(reader) {
    const uptimeClock = reader.read(UptimeClockDeserialize);
    if (TextualError.is(uptimeClock)) {
      return uptimeClock
        .changeContext("Deserializing 'uptimeClock' field of 'Data2'")
        .changeContext("Deserializing Data2'");
    }

    const monotonicClock = reader.read(MonotonicClockDeserialize);
    if (TextualError.is(monotonicClock)) {
      return monotonicClock
        .changeContext("Deserializing 'monotonicClock' field of 'Data2'")
        .changeContext("Deserializing Data2'");
    }

    return Data2.construct(
      monotonicClock,
      uptimeClock,
    );
  },
});