package com.example.app.database

import com.example.app.*

object AlwaysRuleGroupsTable {
  const val TABLE = "AlwaysRuleGroupsTable"

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

  fun insertOrThrow(database: DatabaseConnection): AlwaysRuleGroupId {
    val buffer = Buffer()
    writeInsert(buffer) 
    return AlwaysRuleGroupId(database.insertOrThrow(buffer.string()))
  }

  fun writeDelete(
    buffer: Buffer,
    ruleGroupId: AlwaysRuleGroupId,
  ) {
    buffer.apply {
      code("DELETE FROM $TABLE WHERE $ID = ")
      alwaysRuleGroupId(ruleGroupId)
      code(";")
    }
  }

  fun deleteOrThrow(
    database: DatabaseConnection,
    ruleGroupId: AlwaysRuleGroupId,
  ) {
    val buffer = Buffer()
    writeDelete(buffer, ruleGroupId)
    database.execSqlOrThrow(buffer.string())
  }
}