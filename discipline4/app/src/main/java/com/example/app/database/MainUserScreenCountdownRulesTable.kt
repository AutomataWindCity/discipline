package com.example.app.database

import android.database.Cursor
import android.database.sqlite.SQLiteDatabase
import com.example.app.database.*
import com.example.app.*

public class MainUserScreenCountdownRulesTable {
  companion object {
    const val TABLE = "MainUserScreenCountdownRulesTable"

    const val ID = "id"
    // const val ENABLED = "enabled"
    const val COUNTDOWN_FROM = "countdown_from"
    const val COUNTDOWN_DURATION = "countdown_duration"

    const val ID_INDEX = 0
    // const val ENABLED_INDEX = 1
    const val COUNTDOWN_FROM_INDEX = 1
    const val COUNTDOWN_DURATION_INDEX = 2
  }

  fun writeCreateTable(
    buffer: Buffer,
  ) {
    buffer.write("""
      CREATE TABLE IF NOT EXISTS $TABLE (
        $ID INTEGER PRIMARY KEY,
        $COUNTDOWN_FROM INTEGER NOT NULL,
        $COUNTDOWN_DURATION INTEGER NOT NULL
      ) STRICT, WITHOUT ROWID;
    """)
  }

  fun writeInsertRule(
    buffer: Buffer,
    id: UuidV4,
    rule: CountdownRule,
  ) {
    buffer.write("""
      INSERT INTO $TABLE (
        ${id.toSql()},
        ${rule.countdown.from.toSql()},
        ${rule.countdown.duration.toSql()},
      );
    """)
  }

  fun insertRuleOrThrow(
    database: SQLiteDatabase,
    id: UuidV4,
    rule: CountdownRule,
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

  fun readRuleOrThrow(cursor: Cursor): CountdownRule {
    // val rawEnabled = cursor.ourGetIntOrThrow(ENABLED_INDEX)
    val rawCountdownFrom = cursor.ourGetLongOrThrow(COUNTDOWN_FROM_INDEX)
    val rawCountdownDuration = cursor.ourGetLongOrThrow(COUNTDOWN_DURATION_INDEX)
    
    // val enabled = createBooleanFromSqlOrThrow(rawEnabled)
    val countdownFrom = createInstantFromSqlOrThrow(rawCountdownFrom)
    val countdownDuration = createDurationFromSqlOrThrow(rawCountdownDuration)

    return CountdownRule.construct(
      // isEnabled = enabled, 
      countdown = Countdown.create(
        countdownFrom, 
        countdownDuration,
      ),
    )
  }

  fun selectRuleOrThrow(
    database: SQLiteDatabase, 
    id: UuidV4,
  ): CountdownRule? {
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

  fun selectAllRulesOrThrow(database: SQLiteDatabase): MutableMap<UuidV4, CountdownRule> {
    val buffer = Buffer()
    writeSelectAllRules(buffer)

    val rules = mutableMapOf<UuidV4, CountdownRule>()
    val cursor = database.ourQuery(buffer.string())
    while (cursor.moveToNext()) {
      val id = createUuidFromSqlOrThrow(cursor.ourGetStringOrThrow(ID_INDEX))
      val rule = readRuleOrThrow(cursor)
      rules.set(id, rule)
    }

    return rules
  }
}