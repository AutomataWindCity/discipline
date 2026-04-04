package com.example.app.database

import android.database.Cursor
import android.database.sqlite.SQLiteDatabase
import com.example.app.database.*
import com.example.app.*

public class MainUserScreenTimeAllowanceRules {
  companion object {
    const val TABLE = "MainUserScreenTimeAllowanceRules"

    const val ID = "id"
    const val ALLOWANCE = "allowance"
    const val LIFETIME_FROM = "lifetime_from"
    const val LIFETIME_DURATION = "lifetime_duration"

    const val ID_INDEX = 0
    const val ALLOWANCE_INDEX = 1
    const val LIFETIME_FROM_INDEX = 2
    const val LIFETIME_DURATION_INDEX = 3
  }

  fun writeCreateTable(
    buffer: Buffer,
  ) {
    buffer.write("""
      CREATE TABLE IF NOT EXISTS $TABLE (
        $ID INTEGER PRIMARY KEY,
        $ALLOWANCE INTEGER NOT NULL,
        $LIFETIME_FROM INTEGER NOT NULL,
        $LIFETIME_DURATION INTEGER NOT NULL
      ) STRICT, WITHOUT ROWID;
    """)
  }

  fun writeInsertRule(
    buffer: Buffer,
    id: UuidV4,
    rule: TimeAllowanceRule,
  ) {
    buffer.write("""
      INSERT INTO $TABLE (
        ${id.toSql()},
        ${rule.allowance.toSql()},
        ${rule.lifetime.from.toSql()},
        ${rule.lifetime.duration.toSql()},
      );
    """)
  }

  fun insertRuleOrThrow(
    database: SQLiteDatabase,
    id: UuidV4,
    rule: TimeAllowanceRule,
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

  fun deleteRule(
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

  fun readRuleOrThrow(cursor: Cursor): TimeAllowanceRule {
    val rawAllowance = cursor.ourGetLongOrThrow(ALLOWANCE_INDEX)
    val rawLifetimeFrom = cursor.ourGetLongOrThrow(LIFETIME_FROM_INDEX)
    val rawLifetimeDuration = cursor.ourGetLongOrThrow(LIFETIME_DURATION_INDEX)

    val allowance = createDurationFromSqlOrThrow(rawAllowance)
    val lifetimeFrom = createInstantFromSqlOrThrow(rawLifetimeFrom)
    val lifetimeDuration = createDurationFromSqlOrThrow(rawLifetimeDuration)

    return TimeAllowanceRule.createOrThrow(
      allowance = allowance,
      lifetime = Countdown.create(
        lifetimeFrom,
        lifetimeDuration,
      ),
    )
  }

  fun selectRuleOrThrow(
    database: SQLiteDatabase,
    id: UuidV4,
  ): TimeAllowanceRule? {
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

  fun selectAllRulesOrThrow(database: SQLiteDatabase): MutableMap<UuidV4, TimeAllowanceRule> {
    val buffer = Buffer()
    writeSelectAllRules(buffer)

    val rules = mutableMapOf<UuidV4, TimeAllowanceRule>()
    val cursor = database.ourQuery(buffer.string())
    while (cursor.moveToNext()) {
      val id = createUuidFromSqlOrThrow(cursor.ourGetStringOrThrow(ID_INDEX))
      val rule = readRuleOrThrow(cursor)
      rules.set(id, rule)
    }

    return rules
  }
}
