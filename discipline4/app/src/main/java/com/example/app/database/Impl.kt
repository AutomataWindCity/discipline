package com.example.app.database

import com.example.app.*

interface ScalarWriter {
  fun writeNull(): Unit
  fun writeInt(value: Int): Unit
  fun writeNullableInt(value: Int?): Unit
  fun writeLong(value: Long): Unit
  fun writeNullableLong(value: Long?): Unit
  fun writeBoolean(value: Boolean): Unit
  fun writeNullableBoolean(value: Boolean?): Unit
  fun writeNullableLong(value: String?): Unit
  fun <Value> writeScalar(value: Value, write: ScalarWrite<Value>): Unit
  fun <Value> writeNullableScalar(value: Value?write: ScalarWrite<Value>): Unit
}

interface ScalarWrite<Value> {
  fun write(value: Value, writer: ScalarWriter): Unit
}

interface ScalarSanitize<Value> {

}

interface IntSanitize<Value> : ScalarSanitize<Value> {
  fun sanitizeOrThrow(value: Int): Value

  fun internalSantizeOrThrow() {}
}

interface LongSanitize<Value> : ScalarSanitize<Value> {
  fun sanitizeOrThrow(value: Int): Value

  fun internalSantizeOrThrow() {}
}

interface Index {}

interface Writer {
  fun writeNull(index: Index): Unit
  fun writeInt(index: Index, value: Int): Unit
  fun writeNullableInt(index: Index, value: Int?): Unit
  fun writeLong(index: Index, value: Long): Unit
  fun writeNullableLong(index: Index, value: Long?): Unit
  fun writeBoolean(index: Index, value: Boolean): Unit
  fun writeNullableBoolean(index: Index, value: Boolean?): Unit
  fun writeNullableLong(index: Index, value: String?): Unit
  fun <Value, Schema> write(schema: Schema, value: Value, write: Write<Value, Schema>): Unit
  fun <Value, Schema> writeOptional(optionVariantIndex: Index, valueSchema: Schema, value: Value?, valueWrite: Write<Value, Schema>)
}

interface Reader {
  fun readNullOrThrow(index: Index): null
  fun readIntOrThrow(index: Index): Int
  fun readNullableIntOrThrow(index: Index): Int?
  fun readLongOrThrow(index: Index): Long
  fun readNullableLongOrThrow(index: Index): Long?
  fun readBooleanOrThrow(index: Index): Boolean
  fun readNullableBooleanOrThrow(index: Index): Boolean?
  fun <Value, Schema> readOrThrow(schema: Schema, read: Read<Value, Schema>): Value
  fun <Value, Schema> readOptionalOrThrow(optionVariantIndex: Index, schema: Schema, read: Read<Value, Schema>): Value?
}

interface Write<Value, Schema> {
  fun write(value: Value, schema: Schema, writer: Writer): Unit
}

interface Read<Value, Schema> {
  fun readOrThrow(schema: Schema, reader: Reader): Value
}

@JvmInline
value class TimeSchema(
  val timestamp: Index,
)

object TimeWrite : Write<Time, TimeSchema> {
  override fun write(value: Time, schema: TimeSchema, writer: Writer): Unit {
    writer.writeInt(schema.timestamp, value.toTimestamp())
  }
}

object TimeRead : Read<Time, TimeSchema> {
  override fun readOrThrow(schema: TimeSchema, reader: Reader): Time {
    return Time.fromTimestampOrThrow(
      reader.readIntOrThrow(schema.timestamp)
    )
  }
}

class TimeRangeSchema(
  val from: TimeSchema,
  val till: TimeSchema,
)

object TtimeRangeWrite : Write<TimeRange, TimeRangeSchema> {

}

class MonotonicClockSchema(
  var elapsedTime: DurationSchema,
  var previousSynchronizationTime: InstantSchema,
  var synchronizationInterval: DurationSchema,
)

object MonotonicClockWrite : Write<MonotonicClock, MonotonicClockSchema> {
  override fun write(value: MonotonicClock, schema: MonotonicClockSchema, writer: Writer): Unit {
    writer.write(schema.elapsedTime, value.elapsedTime, DurationWrite)
    writer.write(schema.previousSynchronizationTime, value.previousSynchronizationTime, InstantSchema)
    writer.write(schema.synchronizationInterval, value.synchronizationInterval, DurationWrite)
  }
}

@JvmInline
value class DurationSchema(
  val milliseconds: Index,
)

object DurationWrite : Write<Duration, DurationSchema> {
  override fun write(value: Duration, schema: DurationSchema, writer: Writer): Unit {
    writer.writeLong(schema.milliseconds, value.toTotalMilliseconds())
  }
}

object DurationRead : Read<Duration, DurationSchema> {
  override fun readOrThrow(schema: DurationSchema, reader: Reader): Duration {
    return Duration.fromMillisecondsOrThrow(
      reader.readLongOrThrow(schema.milliseconds)
    )
  }
}

@JvmInline
value class InstantSchema(
  val duration: DurationSchema,
)

object InstantWrite : Write<Instant, InstantSchema> {
  override fun write(value: Instant, schema: InstantSchema, writer: Writer): Unit {
    writer.write(schema.duration, value.toElapsedTime(), DurationWrite)
  }
}

object InstantRead : Read<Instant, InstantSchema> {
  override fun readOrThrow(schema: InstantSchema, reader: Reader): Instant {
    return Instant.fromElapsedTime(
      reader.readOrThrow(schema.duration, DurationRead)
    )
  }
}

class CountdownSchema(
  val from: InstantSchema,
  val duration: DurationSchema,
)

object CountdownWrite : Write<Countdown, CountdownSchema> {
  override fun write(value: Countdown, schema: CountdownSchema, writer: Writer): Unit {
    writer.write(schema.from, value.from, InstantWrite)
    writer.write(schema.duration, value.duration, DurationWrite)
  }
}

object CountdownRead : Read<Countdown, CountdownSchema> {
  override fun readOrThrow(schema: CountdownSchema, reader: Reader): Countdown {
    return Countdown.create(
      reader.readOrThrow(schema.from, InstantRead),
      reader.readOrThrow(schema.duration, DurationRead),
    )
  }
}

class CountdownConditionalSchema(
  val duration: DurationSchema,
  val countdownOptionVariant: Index,
  val countdown: CountdownSchema,
)

object CountdownConditionalWrite : Write<CountdownConditional, CountdownConditionalSchema> {
  override fun write(value: CountdownConditional, schema: CountdownConditionalSchema, writer: Writer): Unit {
    writer.write(schema.duration, value.duration, DurationWrite)
    writer.writeOptional(schema.countdownOptionVariant, schema.countdown, value.countdown, CountdownWrite)
  }
}

object CountdownConditionalRead : Read<CountdownConditional, CountdownConditionalSchema> {
  override fun readOrThrow(schema: CountdownConditionalSchema, reader: Reader): CountdownConditional {
    return CountdownConditional.construct(
      reader.readOrThrow(schema.duration, DurationRead), 
      reader.readOptionalOrThrow(schema.countdownOptionVariant, schema.countdown, CountdownRead)
    )
  }
}


class CountdownAfterPleaConditionalSchema(
  val intervalFromPleaTillDeactivation: DurationSchema,
  val countdownTillDeactivationOptionVariant: Index,
  var countdownTillDeactivation: CountdownSchema,
)

object CountdownAfterPleaConditionalWrite : Write<CountdownAfterPleaConditional, CountdownAfterPleaConditionalSchema> {
  override fun write(value: CountdownAfterPleaConditional, schema: CountdownAfterPleaConditionalSchema, writer: Writer): Unit {
    writer.write(
      schema.intervalFromPleaTillDeactivation, 
      value.intervalFromPleaTillDeactivation, 
      DurationWrite,
    )

    writer.writeOptional(
      schema.countdownTillDeactivationOptionVariant, 
      schema.countdownTillDeactivation, 
      value.countdownTillDeactivation, 
      CountdownWrite,
    )
  }
}

object CountdownAfterPleaConditionalRead : Read<CountdownAfterPleaConditional, CountdownAfterPleaConditionalSchema> {
  override fun readOrThrow(schema: CountdownAfterPleaConditionalSchema, reader: Reader): CountdownAfterPleaConditional {
    return CountdownAfterPleaConditional.construct(
      reader.readOrThrow(
        schema.intervalFromPleaTillDeactivation, 
        DurationRead,
      ), 
      reader.readOptionalOrThrow(
        schema.countdownTillDeactivationOptionVariant, 
        schema.countdownTillDeactivation, 
        CountdownRead,
      )
    )
  }
}

class RuleEnablerVariantSchema(
  val variant: Index,
)

object RuleEnablerVariantWrite : Write<RuleEnabler.Variant, RuleEnablerVariantSchema> {
  override fun write(value: RuleEnabler.Variant, schema: RuleEnablerVariantSchema, writer: Writer): Unit {
    when (value) {
      RuleEnabler.Variant.Countdown -> {
        writer.writeInt(schema.variant, 0)
      }
      RuleEnabler.Variant.CountdownAfterPlea -> {
        writer.writeInt(schema.variant, 1)
      }
    }
  }
}

object RuleEnablerVariantRead : Read<RuleEnabler.Variant, RuleEnablerVariantSchema> {
  override fun readOrThrow(schema: RuleEnablerVariantSchema, reader: Reader): RuleEnabler.Variant {
    return when (reader.readIntOrThrow(schema.variant)) {
      0 -> {
        RuleEnabler.Variant.Countdown
      }
      1 -> {
        RuleEnabler.Variant.CountdownAfterPlea
      }
      else -> {
        // TODO
        throw TextualError.create("")
      }
    }
  }
}

class RuleEnablerSchema(
  val variant: RuleEnablerVariantSchema,
  val countdownVariant: CountdownConditionalSchema,
  val countdownAfterPleaVariant: CountdownAfterPleaConditionalSchema,
)

object RuleEnablerWrite : Write<RuleEnabler, RuleEnablerSchema> {
  override fun write(value: RuleEnabler, schema: RuleEnablerSchema, writer: Writer): Unit {
    when (value) {
      is RuleEnabler.Countdown {
        writer.write(
          schema.variant, 
          RuleEnabler.Variant.Countdown,
          RuleEnablerVariantWrite,
        )

        writer.write(
          schema.countdownVariant, 
          value.it, 
          CountdownConditionalWrite,
        )
      }
      is RuleEnabler.CountdownAfterPlea -> {
        writer.write(
          schema.variant, 
          RuleEnabler.Variant.CountdownAfterPlea, 
          RuleEnablerVariantWrite,
        )

        writer.write(
          schema.countdownAfterPleaVariant, 
          value.it, 
          CountdownAfterPleaConditionalWrite,
        )
      }
    }
  }
}
