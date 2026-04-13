package com.example.app.database

import com.example.app.database.*
import com.example.app.*
import android.database.Cursor

enum class OptionVariant {
  None,
  Some;

  companion object {
    fun fromNumberOrThrow(number: Int): OptionVariant {
      return when (number) {
        0 -> None
        1 -> Some
        else -> throw TextualError.create("Unknown OptionVariant number: $number")
      }
    }
  }
}

fun Buffer.none() {
  code("NULL")
}

fun Buffer.comma() {
  code(", ")
}

fun <T> Buffer.orderedOption(value: T?, orderedWrite: Buffer.(T) -> Unit) {
  value
    ?.let {
      int(1)
      orderedWrite(it)
    }
    ?: {
      int(0)
    }
}

fun Buffer.named(name: String, namedValue: Buffer.() -> Unit) {
  code("$name = ")
  namedValue()
}

// -------------------------------
fun Buffer.int(value: Int) {
  code(value.toString())
}

fun Buffer.long(value: Long) {
  code(value.toString())
}

fun Buffer.string(value: String) {
  code(value.toString())
}

fun Buffer.boolean(value: Boolean) {
  code(value.toString())
}

fun Buffer.uuidV4(value: UuidV4) {
  code(value.toString())
}

fun Buffer.time(value: Time) {
  int(value.toTimestamp())
}

fun Buffer.duration(value: Duration) {
  long(value.toTotalMilliseconds())
}

fun Buffer.instant(value: Instant) {
  duration(value.toElapsedTime())
}

fun Buffer.ruleEnablerVariant(value: RuleEnabler.Variant) {
  int(value.toNumber())
}

fun Buffer.orderedTimeRange(value: TimeRange) {
  int(value.fromTimestamp)
  comma()
  int(value.tillTimestamp)
}

fun Buffer.orderedCountdown(value: Countdown) {
  instant(value.from)
  comma()
  duration(value.duration)
}

fun Buffer.orderedNullCountdown() {
  code("NULL, NULL")
}

fun Buffer.orderedCountdownConditional(value: CountdownConditional) {
  duration(value.duration)
  orderedOption(value.countdown) { orderedCountdown(it) }
}

fun Buffer.orderedCountdownAfterPleaConditional(value: CountdownAfterPleaConditional) {
  duration(value.intervalFromPleaTillDeactivation)
  orderedOption(value.countdownTillDeactivation) { orderedCountdown(it) }
}

fun Buffer.orderedRuleEnabler(value: RuleEnabler) {
  when (value) {
    is RuleEnabler.Countdown -> {
      ruleEnablerVariant(RuleEnabler.Variant.Countdown)
      comma()
      orderedCountdownConditional(value.it)
    }
    is RuleEnabler.CountdownAfterPlea -> {
      ruleEnablerVariant(RuleEnabler.Variant.CountdownAfterPlea)
      orderedCountdownAfterPleaConditional(value.it)
    }
  }
}

fun Buffer.orderedAlwaysRule(value: AlwaysRule) {
  orderedRuleEnabler(value.enabler)
}

fun Buffer.alwaysRuleId(value: AlwaysRuleId) {
  long(value.asNumber())
}

fun Buffer.alwaysRuleGroupId(value: AlwaysRuleGroupId) {
  long(value.value)
}

fun Buffer.timeRangeRuleGroupId(value: TimeRangeRuleGroupId) {
  long(value.value)
}

fun Buffer.timeAllowanceRuleGroupId(value: TimeAllowanceRuleGroupId) {
  long(value.value)
}

fun Buffer.orderedTimeRangeRule(value: TimeRangeRule) {
  orderedRuleEnabler(value.enabler)
  comma()
  orderedTimeRange(value.condition)
}

fun Buffer.timeRangeRuleId(value: TimeRangeRuleId) {
  long(value.asNumber())
}

fun Buffer.orderedTimeAllowanceRule(value: TimeAllowanceRule) {
  orderedRuleEnabler(value.enabler)
  comma()
  duration(value.allowance)
}

fun Buffer.timeAllowanceRuleId(value: TimeAllowanceRuleId) {
  long(value.asNumber())
}

fun Buffer.applicationRegulationId(value: ApplicationRegulationId) {
  long(value.toNumber())
}
fun Buffer.applicationName(value: ApplicationName) {
  string(value.toString())
}

data class OptionNames<SomeNames>(
  val tag: String,
  val some: SomeNames,
) 

fun <SomeNames> Buffer.setOptionToNone(
  names: OptionNames<SomeNames>
) {}

fun <SomeNames> Buffer.setOptionToSome(
  names: OptionNames<SomeNames>,
  namedWriteSome: Buffer.(SomeNames) -> Unit
) {}

// {
//   fun writeNone(
//     destination: NamedWriteDestination, 
//     valueNamedWriteNull: SomeNames.() -> Unit,
//   ) {
//     0.namedWrite(tag, destination)
//     valueNamedWriteNull(some)
//   }

//   fun writeSome(
//     destination: NamedWriteDestination, 
//     valueNamedWrite: SomeNames.() -> Unit
//   ) {
//     1.namedWrite(tag, destination)
//     valueNamedWrite(some)
//   }
// }

data class TimeRangeNames(
  val from: String,
  val till: String,
)

fun Buffer.namedTimeRange(
  names: TimeRangeNames,
  value: TimeRange,
) {
  named(names.from) { int(value.fromTimestamp) }
  comma()
  named(names.till) { int(value.tillTimestamp) }
}

data class CountdownNames(
  val from: String,
  val duration: String,
)

fun Buffer.namedCountdown(
  names: CountdownNames,
  value: Countdown,
) {
  named(names.from) { instant(value.from) }
  comma()
  named(names.duration) { duration(value.duration) }
}

data class CountdownConditionalNames(
  val duration: String,
  val countdown: OptionNames<CountdownNames>,
) 

fun Buffer.reactivateCountdownConditional(
  names: CountdownConditionalNames,
  reactivateState: CountdownConditional.ReactivateState,
) {
  setOptionToSome(names.countdown) { namedCountdown(it, reactivateState.countdown) }
}

// {
//   fun writeDuration(destination: NamedWriteDestination, value: Duration) {
//     value.namedWrite(duration, destination)
//   }

//   fun writeCountdownNone(destination: NamedWriteDestination) {
//     countdown.writeNone(destination) { writeNull(destination) }
//   }

//   fun writeCountdownSome(destination: NamedWriteDestination, value: Countdown) {
//     countdown.writeSome(destination) { write(destination, value) }
//   }

//   fun writeCountdown(destination: NamedWriteDestination, value: Countdown?) {
//     value 
//       ?.let {
//         writeCountdownSome(destination, it)
//       } 
//       ?: run {
//         writeCountdownNone(destination)
//       }
//   }

//   fun write(destination: NamedWriteDestination, value: CountdownConditional) {
//     writeDuration(destination, value.duration)
//     writeCountdown(destination, value.countdown)
//   }

//   fun reactivate(
//     destination: NamedWriteDestination,
//     reactivateState: CountdownConditional.ReactivateState,
//   ) {
//     writeCountdownSome(destination, reactivateState.countdown)
//   }
// }

fun Buffer.namedCountdownConditional(
  names: CountdownConditionalNames,
  value: CountdownConditional,
) {
  named(names.duration) { duration(value.duration) }
  comma()
  // TODO
}

data class CountdownAfterPleaConditionalNames(
  val intervalFromPleaTillDeactivation: String,
  val countdownTillDeactivation: OptionNames<CountdownNames>,
) 
// {
//   fun writeDuration(destination: NamedWriteDestination, value: Duration) {
//     value.namedWrite(duration, destination)
//   }

//   fun writeCountdownNone(destination: NamedWriteDestination) {
//     countdown.writeNone(destination) { writeNull(destination) }
//   }

//   fun writeCountdownSome(destination: NamedWriteDestination, value: Countdown) {
//     countdown.writeSome(destination) { write(destination, value) }
//   }

//   fun writeCountdown(destination: NamedWriteDestination, value: Countdown?) {
//     value 
//       ?.let {
//         writeCountdownSome(destination, it)
//       } 
//       ?: run {
//         writeCountdownNone(destination)
//       }
//   }

//   fun write(destination: NamedWriteDestination, value: CountdownAfterPleaConditional) {
//     writeDuration(destination, value.intervalFromPleaTillDeactivation)
//     writeCountdown(destination, value.countdownTillDeactivation)
//   }

//   fun reactivate(
//     destination: NamedWriteDestination,
//   ) {
//     writeCountdownNone(destination)
//   }

//   fun reDeactivate(
//     destination: NamedWriteDestination,
//     reDeactivateState: CountdownAfterPleaConditional.ReDeactivateState,
//   ) {
//     writeCountdownSome(destination, reDeactivateState.countdown)
//   }
// }

fun Buffer.namedCountdownAfterPleaConditional(
  names: CountdownAfterPleaConditionalNames,
  value: CountdownAfterPleaConditional,
) {
  named(names.intervalFromPleaTillDeactivation) { duration(value.intervalFromPleaTillDeactivation) }
  comma()
  // TODO
}


fun Buffer.reactivateCountdownAfterPleaConditional(
  names: CountdownAfterPleaConditionalNames,
) {
  setOptionToNone(names.countdownTillDeactivation)
}

fun Buffer.reDeactivateCountdownAfterPleaConditional(
  names: CountdownAfterPleaConditionalNames,
  reDeactivateState: CountdownAfterPleaConditional.ReDeactivateState,
) {
  setOptionToSome(names.countdownTillDeactivation) { namedCountdown(it, reDeactivateState.countdown) }
}

data class RuleEnablerNames(
  val variant: String,
  val countdownConditional: CountdownConditionalNames,
  val countdownAfterPleaConditional: CountdownAfterPleaConditionalNames,
) 
// {
//   fun writeCountdown(destination: NamedWriteDestination, value: CountdownConditional) {
//     RuleEnabler.Variant.Countdown.namedWrite(variant, destination)
//     countdownConditional.write(destination, value)
//   }

//   fun writeCountdownAfterPlea(destination: NamedWriteDestination, value: CountdownAfterPleaConditional) {
//     RuleEnabler.Variant.CountdownAfterPlea.namedWrite(variant, destination)
//     countdownAfterPleaConditional.write(destination, value)
//   }

//   fun write(destination: NamedWriteDestination, value: RuleEnabler) {
//     when (value) {
//       is RuleEnabler.Countdown -> {
//         writeCountdown(destination, value.it)
//       }
//       is RuleEnabler.CountdownAfterPlea -> {
//         writeCountdownAfterPlea(destination, value.it)
//       }
//     }
//   }
// }

fun Buffer.namedRuleEnabler(
  names: RuleEnablerNames,
  value: RuleEnabler,
) {
  when (value) {
    is RuleEnabler.Countdown -> {
      ruleEnablerVariant(RuleEnabler.Variant.Countdown)
      comma()
      namedCountdownConditional(names.countdownConditional, value.it)
    }

    is RuleEnabler.CountdownAfterPlea -> {
      ruleEnablerVariant(RuleEnabler.Variant.CountdownAfterPlea)
      comma()
      namedCountdownAfterPleaConditional(names.countdownAfterPleaConditional, value.it)
    }
  }
}

data class AlwaysRuleNames(
  val enabler: RuleEnablerNames,
)

fun Buffer.namedAlwaysRule(
  names: AlwaysRuleNames,
  value: AlwaysRule,
) {
  namedRuleEnabler(names.enabler, value.enabler)
}

data class TimeRangeRuleNames(
  val enabler: RuleEnablerNames,
  val condition: TimeRangeNames,
)

fun Buffer.namedTimeRange(
  names: TimeRangeRuleNames,
  value: TimeRangeRule,
) {
  namedRuleEnabler(names.enabler, value.enabler)
  comma()
  namedTimeRange(names.condition, value.condition)
}

data class TimeAllowanceRuleNames(
  val enabler: RuleEnablerNames,
  val allowance: String,
)

fun Buffer.namedTimeAllowanceRule(
  names: TimeAllowanceRuleNames,
  value: TimeAllowanceRule,
) {
  namedRuleEnabler(names.enabler, value.enabler)
  comma()
  named(names.allowance) { duration(value.allowance) }
}

// ============ Base Read Functions ============

fun Cursor.isInteger(index: Int): Boolean {
  return getType(index) == Cursor.FIELD_TYPE_INTEGER
}

fun Cursor.readInt(index: Int): Int {
  val fieldType = try {
    getType(index)
  } catch (exception: Throwable) {
    throw TextualError
      .create("Getting field type")
      .addErrorAttachment("Exception", exception)
      .changeContext("Reading an Int")
  }

  when (fieldType) {
    Cursor.FIELD_TYPE_NULL -> {
      throw TextualError
        .create("Reading an Int")
        .addMessage("SQLite value was NULL")
    }
    Cursor.FIELD_TYPE_INTEGER -> {
      try {
        return getInt(index)
      } catch (exception: Throwable) {
        throw TextualError
          .create("Reading an Int")
          .addErrorAttachment("Exception", exception)
      }
    }
    Cursor.FIELD_TYPE_FLOAT -> {
      throw TextualError
        .create("Reading an Int")
        .addMessage("SQLite value was FLOAT")
    }
    Cursor.FIELD_TYPE_STRING -> {
      throw TextualError
        .create("Reading an Int")
        .addMessage("SQLite value was STRING")
    }
    Cursor.FIELD_TYPE_BLOB -> {
      throw TextualError
        .create("Reading an Int")
        .addMessage("SQLite value was BLOB")
    }
    else -> {
      throw TextualError
        .create("Reading an Int")
        .addMessage("Unknown SQLite value type")
    }
  }
}

fun Cursor.readIntOrNull(index: Int): Int? {
  val fieldType = try {
    getType(index)
  } catch (exception: Throwable) {
    throw TextualError
      .create("Getting field type")
      .addErrorAttachment("Exception", exception)
      .changeContext("Reading an Int?")
  }

  when (fieldType) {
    Cursor.FIELD_TYPE_NULL -> {
      return null
    }
    Cursor.FIELD_TYPE_INTEGER -> {
      try {
        return getInt(index)
      } catch (exception: Throwable) {
        throw TextualError
          .create("Reading an Int?")
          .addErrorAttachment("Exception", exception)
      }
    }
    Cursor.FIELD_TYPE_FLOAT -> {
      throw TextualError
        .create("Reading an Int?")
        .addMessage("SQLite value was FLOAT")
    }
    Cursor.FIELD_TYPE_STRING -> {
      throw TextualError
        .create("Reading an Int?")
        .addMessage("SQLite value was STRING")
    }
    Cursor.FIELD_TYPE_BLOB -> {
      throw TextualError
        .create("Reading an Int?")
        .addMessage("SQLite value was BLOB")
    }
    else -> {
      throw TextualError
        .create("Reading an Int?")
        .addMessage("Unknown SQLite value type")
    }
  }
}

fun Cursor.readLong(index: Int): Long {
  val fieldType = try {
    getType(index)
  } catch (exception: Throwable) {
    throw TextualError
      .create("Getting field type")
      .addErrorAttachment("Exception", exception)
      .changeContext("Reading a Long")
  }

  when (fieldType) {
    Cursor.FIELD_TYPE_NULL -> {
      throw TextualError
        .create("Reading a Long")
        .addMessage("SQLite value was NULL")
    }
    Cursor.FIELD_TYPE_INTEGER -> {
      try {
        return getLong(index)
      } catch (exception: Throwable) {
        throw TextualError
          .create("Reading a Long")
          .addErrorAttachment("Exception", exception)
      }
    }
    Cursor.FIELD_TYPE_FLOAT -> {
      throw TextualError
        .create("Reading a Long")
        .addMessage("SQLite value was FLOAT")
    }
    Cursor.FIELD_TYPE_STRING -> {
      throw TextualError
        .create("Reading a Long")
        .addMessage("SQLite value was STRING")
    }
    Cursor.FIELD_TYPE_BLOB -> {
      throw TextualError
        .create("Reading a Long")
        .addMessage("SQLite value was BLOB")
    }
    else -> {
      throw TextualError
        .create("Reading a Long")
        .addMessage("Unknown SQLite value type")
    }
  }
}

fun Cursor.readLongOrNull(index: Int): Long? {
  val fieldType = try {
    getType(index)
  } catch (exception: Throwable) {
    throw TextualError
      .create("Getting field type")
      .addErrorAttachment("Exception", exception)
      .changeContext("Reading a Long?")
  }

  when (fieldType) {
    Cursor.FIELD_TYPE_NULL -> {
      return null
    }
    Cursor.FIELD_TYPE_INTEGER -> {
      try {
        return getLong(index)
      } catch (exception: Throwable) {
        throw TextualError
          .create("Reading a Long?")
          .addErrorAttachment("Exception", exception)
      }
    }
    Cursor.FIELD_TYPE_FLOAT -> {
      throw TextualError
        .create("Reading a Long?")
        .addMessage("SQLite value was FLOAT")
    }
    Cursor.FIELD_TYPE_STRING -> {
      throw TextualError
        .create("Reading a Long?")
        .addMessage("SQLite value was STRING")
    }
    Cursor.FIELD_TYPE_BLOB -> {
      throw TextualError
        .create("Reading a Long?")
        .addMessage("SQLite value was BLOB")
    }
    else -> {
      throw TextualError
        .create("Reading a Long?")
        .addMessage("Unknown SQLite value type")
    }
  }
}

fun Cursor.readFloat(index: Int): Float {
  val fieldType = try {
    getType(index)
  } catch (exception: Throwable) {
    throw TextualError
      .create("Getting field type")
      .addErrorAttachment("Exception", exception)
      .changeContext("Reading a Float")
  }

  when (fieldType) {
    Cursor.FIELD_TYPE_NULL -> {
      throw TextualError
        .create("Reading a Float")
        .addMessage("SQLite value was NULL")
    }
    Cursor.FIELD_TYPE_INTEGER -> {
      throw TextualError
        .create("Reading a Float")
        .addMessage("SQLite value was INTEGER")
    }
    Cursor.FIELD_TYPE_FLOAT -> {
      try {
        return getFloat(index)
      } catch (exception: Throwable) {
        throw TextualError
          .create("Reading a Float")
          .addErrorAttachment("Exception", exception)
      }
    }
    Cursor.FIELD_TYPE_STRING -> {
      throw TextualError
        .create("Reading a Float")
        .addMessage("SQLite value was STRING")
    }
    Cursor.FIELD_TYPE_BLOB -> {
      throw TextualError
        .create("Reading a Float")
        .addMessage("SQLite value was BLOB")
    }
    else -> {
      throw TextualError
        .create("Reading a Float")
        .addMessage("Unknown SQLite value type")
    }
  }
}

fun Cursor.readFloatOrNull(index: Int): Float? {
  val fieldType = try {
    getType(index)
  } catch (exception: Throwable) {
    throw TextualError
      .create("Getting field type")
      .addErrorAttachment("Exception", exception)
      .changeContext("Reading a Float?")
  }

  when (fieldType) {
    Cursor.FIELD_TYPE_NULL -> {
      return null
    }
    Cursor.FIELD_TYPE_INTEGER -> {
      throw TextualError
        .create("Reading a Float?")
        .addMessage("SQLite value was INTEGER")
    }
    Cursor.FIELD_TYPE_FLOAT -> {
      try {
        return getFloat(index)
      } catch (exception: Throwable) {
        throw TextualError
          .create("Reading a Float?")
          .addErrorAttachment("Exception", exception)
      }
    }
    Cursor.FIELD_TYPE_STRING -> {
      throw TextualError
        .create("Reading a Float?")
        .addMessage("SQLite value was STRING")
    }
    Cursor.FIELD_TYPE_BLOB -> {
      throw TextualError
        .create("Reading a Float?")
        .addMessage("SQLite value was BLOB")
    }
    else -> {
      throw TextualError
        .create("Reading a Float?")
        .addMessage("Unknown SQLite value type")
    }
  }
}

fun Cursor.readDouble(index: Int): Double {
  val fieldType = try {
    getType(index)
  } catch (exception: Throwable) {
    throw TextualError
      .create("Getting field type")
      .addErrorAttachment("Exception", exception)
      .changeContext("Reading a Double")
  }

  when (fieldType) {
    Cursor.FIELD_TYPE_NULL -> {
      throw TextualError
        .create("Reading a Double")
        .addMessage("SQLite value was NULL")
    }
    Cursor.FIELD_TYPE_INTEGER -> {
      throw TextualError
        .create("Reading a Double")
        .addMessage("SQLite value was INTEGER")
    }
    Cursor.FIELD_TYPE_FLOAT -> {
      try {
        return getDouble(index)
      } catch (exception: Throwable) {
        throw TextualError
          .create("Reading a Double")
          .addErrorAttachment("Exception", exception)
      }
    }
    Cursor.FIELD_TYPE_STRING -> {
      throw TextualError
        .create("Reading a Double")
        .addMessage("SQLite value was STRING")
    }
    Cursor.FIELD_TYPE_BLOB -> {
      throw TextualError
        .create("Reading a Double")
        .addMessage("SQLite value was BLOB")
    }
    else -> {
      throw TextualError
        .create("Reading a Double")
        .addMessage("Unknown SQLite value type")
    }
  }
}

fun Cursor.readDoubleOrNull(index: Int): Double? {
  val fieldType = try {
    getType(index)
  } catch (exception: Throwable) {
    throw TextualError
      .create("Getting field type")
      .addErrorAttachment("Exception", exception)
      .changeContext("Reading a Double?")
  }

  when (fieldType) {
    Cursor.FIELD_TYPE_NULL -> {
      return null
    }
    Cursor.FIELD_TYPE_INTEGER -> {
      throw TextualError
        .create("Reading a Double?")
        .addMessage("SQLite value was INTEGER")
    }
    Cursor.FIELD_TYPE_FLOAT -> {
      try {
        return getDouble(index)
      } catch (exception: Throwable) {
        throw TextualError
          .create("Reading a Double?")
          .addErrorAttachment("Exception", exception)
      }
    }
    Cursor.FIELD_TYPE_STRING -> {
      throw TextualError
        .create("Reading a Double?")
        .addMessage("SQLite value was STRING")
    }
    Cursor.FIELD_TYPE_BLOB -> {
      throw TextualError
        .create("Reading a Double?")
        .addMessage("SQLite value was BLOB")
    }
    else -> {
      throw TextualError
        .create("Reading a Double?")
        .addMessage("Unknown SQLite value type")
    }
  }
}

fun Cursor.readString(index: Int): String {
  val fieldType = try {
    getType(index)
  } catch (exception: Throwable) {
    throw TextualError
      .create("Getting field type")
      .addErrorAttachment("Exception", exception)
      .changeContext("Reading a String")
  }

  when (fieldType) {
    Cursor.FIELD_TYPE_NULL -> {
      throw TextualError
        .create("Reading a String")
        .addMessage("SQLite value was NULL")
    }
    Cursor.FIELD_TYPE_INTEGER -> {
      throw TextualError
        .create("Reading a String")
        .addMessage("SQLite value was INTEGER")
    }
    Cursor.FIELD_TYPE_FLOAT -> {
      throw TextualError
        .create("Reading a String")
        .addMessage("SQLite value was FLOAT")
    }
    Cursor.FIELD_TYPE_STRING -> {
      try {
        return getString(index)
      } catch (exception: Throwable) {
        throw TextualError
          .create("Reading a String")
          .addErrorAttachment("Exception", exception)
      }
    }
    Cursor.FIELD_TYPE_BLOB -> {
      throw TextualError
        .create("Reading a String")
        .addMessage("SQLite value was BLOB")
    }
    else -> {
      throw TextualError
        .create("Reading a String")
        .addMessage("Unknown SQLite value type")
    }
  }
}

fun Cursor.readStringOrNull(index: Int): String? {
  val fieldType = try {
    getType(index)
  } catch (exception: Throwable) {
    throw TextualError
      .create("Getting field type")
      .addErrorAttachment("Exception", exception)
      .changeContext("Reading a String?")
  }

  when (fieldType) {
    Cursor.FIELD_TYPE_NULL -> {
      return null
    }
    Cursor.FIELD_TYPE_INTEGER -> {
      throw TextualError
        .create("Reading a String?")
        .addMessage("SQLite value was INTEGER")
    }
    Cursor.FIELD_TYPE_FLOAT -> {
      throw TextualError
        .create("Reading a String?")
        .addMessage("SQLite value was FLOAT")
    }
    Cursor.FIELD_TYPE_STRING -> {
      try {
        return getString(index)
      } catch (exception: Throwable) {
        throw TextualError
          .create("Reading a String?")
          .addErrorAttachment("Exception", exception)
      }
    }
    Cursor.FIELD_TYPE_BLOB -> {
      throw TextualError
        .create("Reading a String?")
        .addMessage("SQLite value was BLOB")
    }
    else -> {
      throw TextualError
        .create("Reading a String?")
        .addMessage("Unknown SQLite value type")
    }
  }
}

fun Cursor.readBlob(index: Int): ByteArray {
  val fieldType = try {
    getType(index)
  } catch (exception: Throwable) {
    throw TextualError
      .create("Getting field type")
      .addErrorAttachment("Exception", exception)
      .changeContext("Reading a Blob")
  }

  when (fieldType) {
    Cursor.FIELD_TYPE_NULL -> {
      throw TextualError
        .create("Reading a Blob")
        .addMessage("SQLite value was NULL")
    }
    Cursor.FIELD_TYPE_INTEGER -> {
      throw TextualError
        .create("Reading a Blob")
        .addMessage("SQLite value was INTEGER")
    }
    Cursor.FIELD_TYPE_FLOAT -> {
      throw TextualError
        .create("Reading a Blob")
        .addMessage("SQLite value was FLOAT")
    }
    Cursor.FIELD_TYPE_STRING -> {
      throw TextualError
        .create("Reading a Blob")
        .addMessage("SQLite value was STRING")
    }
    Cursor.FIELD_TYPE_BLOB -> {
      try {
        return getBlob(index)
      } catch (exception: Throwable) {
        throw TextualError
          .create("Reading a Blob")
          .addErrorAttachment("Exception", exception)
      }
    }
    else -> {
      throw TextualError
        .create("Reading a Blob")
        .addMessage("Unknown SQLite value type")
    }
  }
}

fun Cursor.readBlobOrNull(index: Int): ByteArray? {
  val fieldType = try {
    getType(index)
  } catch (exception: Throwable) {
    throw TextualError
      .create("Getting field type")
      .addErrorAttachment("Exception", exception)
      .changeContext("Reading a Blob?")
  }

  when (fieldType) {
    Cursor.FIELD_TYPE_NULL -> {
      return null
    }
    Cursor.FIELD_TYPE_INTEGER -> {
      throw TextualError
        .create("Reading a Blob?")
        .addMessage("SQLite value was INTEGER")
    }
    Cursor.FIELD_TYPE_FLOAT -> {
      throw TextualError
        .create("Reading a Blob?")
        .addMessage("SQLite value was FLOAT")
    }
    Cursor.FIELD_TYPE_STRING -> {
      throw TextualError
        .create("Reading a Blob?")
        .addMessage("SQLite value was STRING")
    }
    Cursor.FIELD_TYPE_BLOB -> {
      try {
        return getBlob(index)
      } catch (exception: Throwable) {
        throw TextualError
          .create("Reading a Blob?")
          .addErrorAttachment("Exception", exception)
      }
    }
    else -> {
      throw TextualError
        .create("Reading a Blob?")
        .addMessage("Unknown SQLite value type")
    }
  }
}

fun Cursor.readShort(index: Int): Short {
  val fieldType = try {
    getType(index)
  } catch (exception: Throwable) {
    throw TextualError
      .create("Getting field type")
      .addErrorAttachment("Exception", exception)
      .changeContext("Reading a Short")
  }

  when (fieldType) {
    Cursor.FIELD_TYPE_NULL -> {
      throw TextualError
        .create("Reading a Short")
        .addMessage("SQLite value was NULL")
    }
    Cursor.FIELD_TYPE_INTEGER -> {
      try {
        return getShort(index)
      } catch (exception: Throwable) {
        throw TextualError
          .create("Reading a Short")
          .addErrorAttachment("Exception", exception)
      }
    }
    Cursor.FIELD_TYPE_FLOAT -> {
      throw TextualError
        .create("Reading a Short")
        .addMessage("SQLite value was FLOAT")
    }
    Cursor.FIELD_TYPE_STRING -> {
      throw TextualError
        .create("Reading a Short")
        .addMessage("SQLite value was STRING")
    }
    Cursor.FIELD_TYPE_BLOB -> {
      throw TextualError
        .create("Reading a Short")
        .addMessage("SQLite value was BLOB")
    }
    else -> {
      throw TextualError
        .create("Reading a Short")
        .addMessage("Unknown SQLite value type")
    }
  }
}

fun Cursor.readShortOrNull(index: Int): Short? {
  val fieldType = try {
    getType(index)
  } catch (exception: Throwable) {
    throw TextualError
      .create("Getting field type")
      .addErrorAttachment("Exception", exception)
      .changeContext("Reading a Short?")
  }

  when (fieldType) {
    Cursor.FIELD_TYPE_NULL -> {
      return null
    }
    Cursor.FIELD_TYPE_INTEGER -> {
      try {
        return getShort(index)
      } catch (exception: Throwable) {
        throw TextualError
          .create("Reading a Short?")
          .addErrorAttachment("Exception", exception)
      }
    }
    Cursor.FIELD_TYPE_FLOAT -> {
      throw TextualError
        .create("Reading a Short?")
        .addMessage("SQLite value was FLOAT")
    }
    Cursor.FIELD_TYPE_STRING -> {
      throw TextualError
        .create("Reading a Short?")
        .addMessage("SQLite value was STRING")
    }
    Cursor.FIELD_TYPE_BLOB -> {
      throw TextualError
        .create("Reading a Short?")
        .addMessage("SQLite value was BLOB")
    }
    else -> {
      throw TextualError
        .create("Reading a Short?")
        .addMessage("Unknown SQLite value type")
    }
  }
}

fun Cursor.readBoolean(index: Int): Boolean {
  val fieldType = try {
    getType(index)
  } catch (exception: Throwable) {
    throw TextualError
      .create("Getting field type")
      .addErrorAttachment("Exception", exception)
      .changeContext("Reading a Boolean")
  }

  when (fieldType) {
    Cursor.FIELD_TYPE_NULL -> {
      throw TextualError
        .create("Reading a Boolean")
        .addMessage("SQLite value was NULL")
    }
    Cursor.FIELD_TYPE_INTEGER -> {
      try {
        return getInt(index) == 1
      } catch (exception: Throwable) {
        throw TextualError
          .create("Reading a Boolean")
          .addErrorAttachment("Exception", exception)
      }
    }
    Cursor.FIELD_TYPE_FLOAT -> {
      throw TextualError
        .create("Reading a Boolean")
        .addMessage("SQLite value was FLOAT")
    }
    Cursor.FIELD_TYPE_STRING -> {
      throw TextualError
        .create("Reading a Boolean")
        .addMessage("SQLite value was STRING")
    }
    Cursor.FIELD_TYPE_BLOB -> {
      throw TextualError
        .create("Reading a Boolean")
        .addMessage("SQLite value was BLOB")
    }
    else -> {
      throw TextualError
        .create("Reading a Boolean")
        .addMessage("Unknown SQLite value type")
    }
  }
}

fun Cursor.readBooleanOrNull(index: Int): Boolean? {
  val fieldType = try {
    getType(index)
  } catch (exception: Throwable) {
    throw TextualError
      .create("Getting field type")
      .addErrorAttachment("Exception", exception)
      .changeContext("Reading a Boolean?")
  }

  when (fieldType) {
    Cursor.FIELD_TYPE_NULL -> {
      return null
    }
    Cursor.FIELD_TYPE_INTEGER -> {
      try {
        return getInt(index) == 1
      } catch (exception: Throwable) {
        throw TextualError
          .create("Reading a Boolean?")
          .addErrorAttachment("Exception", exception)
      }
    }
    Cursor.FIELD_TYPE_FLOAT -> {
      throw TextualError
        .create("Reading a Boolean?")
        .addMessage("SQLite value was FLOAT")
    }
    Cursor.FIELD_TYPE_STRING -> {
      throw TextualError
        .create("Reading a Boolean?")
        .addMessage("SQLite value was STRING")
    }
    Cursor.FIELD_TYPE_BLOB -> {
      throw TextualError
        .create("Reading a Boolean?")
        .addMessage("SQLite value was BLOB")
    }
    else -> {
      throw TextualError
        .create("Reading a Boolean?")
        .addMessage("Unknown SQLite value type")
    }
  }
}

// ============ Indexes Data Classes ============
data class OptionIndexes<ValueIndexes>(
  val variant: Int,
  val value: ValueIndexes,
)

data class TimeRangeIndexes(
  val from: Int,
  val till: Int,
)

data class CountdownIndexes(
  val from: Int,
  val duration: Int,
)

data class CountdownConditionalIndexes(
  val duration: Int,
  val countdown: OptionIndexes<CountdownIndexes>,
)

data class CountdownAfterPleaConditionalIndexes(
  val duration: Int,
  val countdown: OptionIndexes<CountdownIndexes>,
)

data class RuleEnablerIndexes(
  val variant: Int,
  val countdownConditional: CountdownConditionalIndexes,
  val countdownAfterPleaConditional: CountdownAfterPleaConditionalIndexes,
)

data class AlwaysRuleIndexes(
  val enabler: RuleEnablerIndexes,
)

data class TimeRangeRuleIndexes(
  val enabler: RuleEnablerIndexes,
  val condition: TimeRangeIndexes,
)

data class TimeAllowanceRuleIndexes(
  val enabler: RuleEnablerIndexes,
  val allowance: Int,
)

fun Cursor.readOptionalVariant(index: Int): OptionVariant {
  return OptionVariant.fromNumberOrThrow(readInt(index))
}

fun <Value, ValueIndexes> Cursor.readOptional(
  indexes: OptionIndexes<ValueIndexes>,
  indexedReadValue: (ValueIndexes) -> Value,
): Value? {
  return when (readOptionalVariant(indexes.variant)) {
    OptionVariant.None -> null
    OptionVariant.Some -> indexedReadValue(indexes.value)
  }
}

fun Cursor.readTime(index: Int): Time {
  try {
    return Time.fromTimestampOrThrow(readInt(index))
  } catch (error: TextualError) {
    throw error.changeContext("Reading a Time")
  }
}

fun Cursor.readTimeRange(indexes: TimeRangeIndexes): TimeRange {
  try {
    return TimeRange.fromTimes(
      readTime(indexes.from),
      readTime(indexes.till),
    )
  } catch (error: TextualError) {
    throw error.changeContext("Reading a TimeRange")
  }
}

fun Cursor.readDuration(index: Int): Duration {
  try {
    return Duration.fromMillisecondsOrThrow(readLong(index))
  } catch (error: TextualError) {
    throw error.changeContext("Reading a Duration")
  }
}

fun Cursor.readInstant(index: Int): Instant {
  try {
    return Instant.fromElapsedTime(readDuration(index))
  } catch (error: TextualError) {
    throw error.changeContext("Reading an Instant")
  }
}

fun Cursor.readCountdown(indexes: CountdownIndexes): Countdown {
  try {
    return Countdown.construct(
      readInstant(indexes.from),
      readDuration(indexes.duration),
    )
  } catch (error: TextualError) {
    throw error.changeContext("Reading a Countdown")
  }
}

fun Cursor.readCountdownConditional(indexes: CountdownConditionalIndexes): CountdownConditional {
  try {
    return CountdownConditional.construct(
      readDuration(indexes.duration),
      readOptional(indexes.countdown) { readCountdown(it) }
    )
  } catch (error: TextualError) {
    throw error.changeContext("Reading a CountdownConditional")
  }
}

fun Cursor.readCountdownAfterPleaConditional(indexes: CountdownAfterPleaConditionalIndexes): CountdownAfterPleaConditional {
  try {
    return CountdownAfterPleaConditional.construct(
      readDuration(indexes.duration),
      readOptional(indexes.countdown) { readCountdown(it) }
    )
  } catch (error: TextualError) {
    throw error.changeContext("Reading a CountdownAfterPleaConditional")
  }
}

fun Cursor.readRuleEnablerVariant(index: Int): RuleEnabler.Variant {
  try {
    return RuleEnabler.Variant.fromNumberOrThrow(readInt(index))
  } catch (error: TextualError) {
    throw error.changeContext("Reading a RuleEnablerVariant")
  }
}

fun Cursor.readRuleEnabler(indexes: RuleEnablerIndexes): RuleEnabler {
  try {
    return when (readRuleEnablerVariant(indexes.variant)) {
      RuleEnabler.Variant.Countdown -> {
        RuleEnabler.Countdown(readCountdownConditional(indexes.countdownConditional))
      }
      RuleEnabler.Variant.CountdownAfterPlea -> {
        RuleEnabler.CountdownAfterPlea(readCountdownAfterPleaConditional(indexes.countdownAfterPleaConditional))
      }
    }
  } catch (error: TextualError) {
    throw error.changeContext("Reading a RuleEnabler")
  }
}

fun Cursor.readAlwaysRule(indexes: AlwaysRuleIndexes): AlwaysRule {
  try {
    return AlwaysRule.construct(readRuleEnabler(indexes.enabler))
  } catch (error: TextualError) {
    throw error.changeContext("Reading an AlwaysRule")
  }
}

fun Cursor.readTimeRangeRule(indexes: TimeRangeRuleIndexes): TimeRangeRule {
  try {
    return TimeRangeRule.construct(
      readRuleEnabler(indexes.enabler),
      readTimeRange(indexes.condition),
    )
  } catch (error: TextualError) {
    throw error.changeContext("Reading a TimeRangeRule")
  }
}

fun Cursor.readTimeAllowanceRule(indexes: TimeAllowanceRuleIndexes): TimeAllowanceRule {
  try {
    return TimeAllowanceRule.construct(
      readRuleEnabler(indexes.enabler),
      readDuration(indexes.allowance),
    )
  } catch (error: TextualError) {
    throw error.changeContext("Reading a TimeAllowanceRule")
  }
}

fun Cursor.readAlwaysRuleId(index: Int): AlwaysRuleId {
  try {
    return AlwaysRuleId(readLong(index))
  } catch (error: TextualError) {
    throw error.changeContext("Reading an AlwaysRuleId")
  }
}

fun Cursor.readTimeRangeRuleId(index: Int): TimeRangeRuleId {
  try {
    return TimeRangeRuleId(readLong(index))
  } catch (error: TextualError) {
    throw error.changeContext("Reading an TimeRangeRuleId")
  }
}

fun Cursor.readTimeAllowanceRuleId(index: Int): TimeAllowanceRuleId {
  try {
    return TimeAllowanceRuleId(readLong(index))
  } catch (error: TextualError) {
    throw error.changeContext("Reading an TimeAllowanceRuleId")
  }
}