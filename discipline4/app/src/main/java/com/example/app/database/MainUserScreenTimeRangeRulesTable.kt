package com.example.app.database

import android.database.Cursor
import android.database.sqlite.SQLiteDatabase
import com.example.app.database.*
import com.example.app.*

public class MainUserScreenTimeRangeRulesTable {
  companion object {
    const val TABLE = "MainUserScreenTimeRangeRulesTable"

    const val ID = "id"
    const val LIFETIME_FROM = "lifetime_from"
    const val LIFETIME_DURATION = "lifetime_duration"
    const val TIME_RANGE_FROM = "time_range_from"
    const val TIME_RANGE_TILL = "time_range_till"

    const val ID_INDEX = 0
    const val LIFETIME_FROM_INDEX = 1
    const val LIFETIME_DURATION_INDEX = 2
    const val TIME_RANGE_FROM_INDEX = 3
    const val TIME_RANGE_TILL_INDEX = 4
  }

  fun writeCreateTable(
    buffer: Buffer,
  ) {
    buffer.write("""
      CREATE TABLE IF NOT EXISTS $TABLE (
        $ID INTEGER PRIMARY KEY,
        $LIFETIME_FROM INTEGER NOT NULL,
        $LIFETIME_DURATION INTEGER NOT NULL,
        $TIME_RANGE_FROM INTEGER NOT NULL,
        $TIME_RANGE_TILL INTEGER NOT NULL
      );
    """)
  }

  fun writeInsertRule(
    buffer: Buffer,
    id: UuidV4,
    rule: TimeRangeRule,
  ) {
    buffer.write("""
      INSERT INTO $TABLE (
        ${id.toSql()},
        ${rule.lifetime.from.toSql()},
        ${rule.lifetime.duration.toSql()},
        ${rule.condition.fromTimestamp.toSql()},
        ${rule.condition.tillTimestamp.toSql()},
      );
    """)
  }

  fun insertRuleOrThrow(
    database: SQLiteDatabase,
    id: UuidV4,
    rule: TimeRangeRule,
  ) {
    val buffer = Buffer()
    writeInsertRule(buffer, id, rule)
    database.execSQL(buffer.string())
  }

  fun writeDeleteRule(
    buffer: Buffer,
    id: UuidV4,
  ) {
    buffer.write("""
      DELETE FROM $TABLE WHERE $ID = ${id.toSql()};
    """)
  }

  fun deleteRuleOrThrow(
    database: SQLiteDatabase,
    id: UuidV4,
  ) {
    val buffer = Buffer()
    writeDeleteRule(buffer, id)
    database.execSQL(buffer.string())
  }

  fun writeSelectRule(
    buffer: Buffer,
    id: UuidV4,
  ) {
    buffer.write("""
      SELECT * FROM $TABLE WHERE ${id.toSql()};
    """)
  }

  fun readRuleOrThrow(cursor: Cursor): TimeRangeRule {
    // val rawEnabled = cursor.ourGetIntOrThrow(ENABLED_INDEX)
    val rawLifetimeFrom = cursor.ourGetLongOrThrow(LIFETIME_FROM_INDEX)
    val rawLifetimeDuration = cursor.ourGetLongOrThrow(LIFETIME_DURATION_INDEX)
    val rawConditionFrom = cursor.ourGetIntOrThrow(TIME_RANGE_FROM_INDEX)
    val rawConditionTill = cursor.ourGetIntOrThrow(TIME_RANGE_TILL_INDEX)
    
    // val enabled = createBooleanFromSqlOrThrow(rawEnabled)
    val lifetimeFrom = createInstantFromSqlOrThrow(rawLifetimeFrom)
    val lifetimeDuration = createDurationFromSqlOrThrow(rawLifetimeDuration)

    return TimeRangeRule.create(
      TimeRange.fromTimestampsOrThrow(
        rawConditionFrom, 
        rawConditionTill,
      ), 
      Countdown.create(
        lifetimeFrom,
        lifetimeDuration,
      ),
    )
  }

  fun selectRuleOrThrow(
    database: SQLiteDatabase, 
    id: UuidV4,
  ): TimeRangeRule? {
    val buffer = Buffer()
    writeSelectRule(buffer, id)

    val cursor = database.ourQuery(buffer.string())
    if (cursor.moveToNext()) {
      return readRuleOrThrow(cursor)
    }

    return null
  }

  fun writeSelectAllRules(
    buffer: Buffer,
  ) {
    buffer.write("""
      SELECT * FROM $TABLE;
    """)
  }

  fun selectAllRulesOrThrow(database: SQLiteDatabase): MutableMap<UuidV4, TimeRangeRule> {
    val buffer = Buffer()
    writeSelectAllRules(buffer)

    val rules = mutableMapOf<UuidV4, TimeRangeRule>()
    val cursor = database.ourQuery(buffer.string())
    while (cursor.moveToNext()) {
      val id = createUuidFromSqlOrThrow(cursor.ourGetStringOrThrow(ID_INDEX))
      val rule = readRuleOrThrow(cursor)
      rules.set(id, rule)
    }

    return rules
  }
}