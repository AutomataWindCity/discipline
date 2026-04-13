package com.example.app.database

import com.example.app.*

object TimeAllowanceRuleGroupsTable {
  const val TABLE = "TimeAllowanceRuleGroupsTable"

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

  fun insertOrThrow(database: DatabaseConnection): TimeAllowanceRuleGroupId {
    val buffer = Buffer()
    writeInsert(buffer) 
    return TimeAllowanceRuleGroupId(database.insertOrThrow(buffer.string()))
  }

  fun writeDelete(
    buffer: Buffer,
    ruleGroupId: TimeAllowanceRuleGroupId,
  ) {
    buffer.apply {
      code("DELETE FROM $TABLE WHERE $ID = ")
      timeAllowanceRuleGroupId(ruleGroupId)
      code(";")
    }
  }

  fun deleteOrThrow(
    database: DatabaseConnection,
    ruleGroupId: TimeAllowanceRuleGroupId,
  ) {
    val buffer = Buffer()
    writeDelete(buffer, ruleGroupId)
    database.execSqlOrThrow(buffer.string())
  }
}