package com.example.app.database

import com.example.app.*

object TimeRangeRuleGroupsTable {
  const val TABLE = "TimeRangeRuleGroupsTable"

  const val ID = "id"

  fun writeCreateTable(
    buffer: Buffer,
  ) {
    buffer.code("""
      CREATE TABLE IF NOT EXISTS $TABLE (
        $ID INTEGER PRIMARY KEY
      ) STRICT;
    """)
  }

  fun writeInsert(
    buffer: Buffer,
  ) {
    buffer.code("INSERT INTO $TABLE VALUES (NULL);")
  }

  fun insertOrThrow(database: DatabaseConnection): TimeRangeRuleGroupId {
    val buffer = Buffer()
    writeInsert(buffer) 
    return TimeRangeRuleGroupId(database.insertOrThrow(buffer.string()))
  }

  fun writeDelete(
    buffer: Buffer,
    ruleGroupId: TimeRangeRuleGroupId,
  ) {
    buffer.apply {
      code("DELETE FROM $TABLE WHERE $ID = ")
      timeRangeRuleGroupId(ruleGroupId)
      code(";")
    }
  }

  fun deleteOrThrow(
    database: DatabaseConnection,
    ruleGroupId: TimeRangeRuleGroupId,
  ) {
    val buffer = Buffer()
    writeDelete(buffer, ruleGroupId)
    database.execSqlOrThrow(buffer.string())
  }
}