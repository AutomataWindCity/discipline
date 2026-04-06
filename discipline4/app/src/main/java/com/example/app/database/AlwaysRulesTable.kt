package com.example.app

import android.database.Cursor
import android.database.sqlite.SQLiteDatabase
import com.example.app.database.*
import com.example.app.*

class AlwaysRulesTable {
  companion object {
    const val TABLE = "AlwaysRules"

    const val ID = "id"
    const val ENABLER_TYPE = "enabler_type"
    const val ENABLED_DATA_1 = "enabler_data_1"
    const val ENABLED_DATA_2 = "enabler_data_2"
    const val ENABLED_DATA_3 = "enabler_data_3"

    const val ENABLER_COUNTDOWN_DURATION = ENABLED_DATA_1
    const val ENABLER_COUNTDOWN_COUNTDOWN_FROM = ENABLED_DATA_2
    const val ENABLER_COUNTDOWN_COUNTDOWN_DURATION = ENABLED_DATA_3

    const val ENABLER_COUNTDOWN_AFTER_PLEA_DURATION = ENABLED_DATA_1
    const val ENABLER_COUNTDOWN_AFTER_PLEA_COUNTDOWN_FROM = ENABLED_DATA_2
    const val ENABLER_COUNTDOWN_AFTER_PLEA_COUNTDOWN_DURATION = ENABLED_DATA_3

    const val ID_INDEX = 0
    const val ENABLER_TYPE_INDEX = 1
    const val ENABLED_DATA_1_INDEX = 2
    const val ENABLED_DATA_2_INDEX = 3
    const val ENABLED_DATA_3_INDEX = 4

    const val ENABLER_COUNTDOWN_DURATION_INDEX = ENABLED_DATA_1_INDEX
    const val ENABLER_COUNTDOWN_COUNTDOWN_FROM_INDEX = ENABLED_DATA_2_INDEX
    const val ENABLER_COUNTDOWN_COUNTDOWN_DURATION_INDEX = ENABLED_DATA_3_INDEX

    const val ENABLER_COUNTDOWN_AFTER_PLEA_DURATION_INDEX = ENABLED_DATA_1_INDEX
    const val ENABLER_COUNTDOWN_AFTER_PLEA_COUNTDOWN_FROM_INDEX = ENABLED_DATA_2_INDEX
    const val ENABLER_COUNTDOWN_AFTER_PLEA_COUNTDOWN_DURATION_INDEX = ENABLED_DATA_3_INDEX
  }

  fun writeCreateTable(code: Buffer) {
    code.write(""" 
      CREATE TABLE IF NOT EXISTS $TABLE (
        $ID INTEGER PRIMARY KEY AUTOINCREMENT,
        $ENABLER_TYPE INTEGER NOT NULL,
        $ENABLER_DATA_1 INTEGER NOT NULL,
        $ENABLER_DATA_2 INTEGER,
        $ENABLER_DATA_3 INTEGER
      ) STRICT;
    """)
  }

  fun writeInsertRule(code: Buffer, rule: AlwaysRule) {
    code.write("INSERT INTO $TABLE VALUES (")
    when (rule.enabler) {
      is RuleEnabler.Countdown -> {
        code.write(rule.enabler.it.duration.toSql())
        code.write(", ")
        when rule.enabler.it.countdown {
          is null -> {
            code.write("NULL")
            code.write(", ")
            code.write("NULL")
          }
          else -> {
            code.write(rule.enabler.it.countdown.from.toSql())
            code.write(", ")
            code.write(rule.enabler.it.countdown.duration.toSql())
          }
        }
      }
      is RuleEnabler
    }
  }
}

interface Index {}

interface WriteAllNull {}

interface CompoundWriter {
  fun writeNull(index: Index): Unit
  fun writeInt(index: Index, value: Int): Unit
  fun writeNullableInt(index: Index, value: Int?): Unit
  fun writeLong(index: Index, value: Long): Unit
  fun writeNullableLong(index: Index, value: Long?): Unit
  fun writeBoolean(index: Index, value: Boolean): Unit
  fun writeNullableBoolean(index: Index, value: Boolean?): Unit
  fun writeNullableLong(index: Index, value: String?): Unit
  fun <Value> writeScalar(index: Index, value: Value): Unit
  fun <Value> writeNullableScalar(index: Index, value: Value?, write: ScalarWrite): Unit
  fun <Value, Schema> writeCompound(schema: Schema, value: Value, write: ScalarWrite): Unit
  fun <Value, Schema : WriteAllNull> writeCompound(schema: Schema, value: Value): Unit
}

interface CompoundWrite<Value, Schema> {
  fun write(value: Value, schema: Schema, writer: CompoundWriter): Unit
}

interface CountdownSchema {
  val from: Index
  val duration: Index
}

object CountdownWrite<Countdown, CountdownSchema> {
  fun write(value: Countdown, schema: CountdownSchema, writer: CompoundWriter): Unit {
    writer.writeScalar(schema.from, value.from, InstantWrite)
    writer.writeScalar(schema.duration, value.duration, DurationWrite)
  }
}

object CountdownRead<Countdown, CountdownSchema> {
  fun readOrThrow(schema: CountdownSchema, reader: CountdownReader): Unit {
    return Countdown.create(
      reader.readScalarOrThrow(schema.from, InstantRead),
      reader.readScalarOrThrow(schema.from, DurationRead),
    )
  }
}