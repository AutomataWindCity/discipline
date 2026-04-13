package com.example.app.database

import com.example.app.*

object MainUserProfileScreenRegulationDailyTimeAllowanceRulesLinkingTable {
  const val TABLE = "MainUserProfileScreenRegulationDailyTimeAllowanceRulesLinkingTable"

  const val ID = "id"
  const val ID_INDEX = 0

  fun writeCreateTable(buffer: Buffer) {
    buffer.code("""
      CREATE IF NOT EXISTS $TABLE (
        $ID INTEGER PRIMARY KEY,
        FOREIGN KEY ($ID) REFERENCES ${TimeAllowanceRulesTable.TABLE}(${TimeAllowanceRulesTable.ID}) ON DELETE CASCADE 
      ) STRICT, WITHOUT ROWID;
    """)
  }

  fun writeInsert(buffer: Buffer, ruleId: TimeAllowanceRuleId) {
    buffer.code("INSERT INTO $TABLE VALUES (")
    buffer.timeAllowanceRuleId(ruleId)
    buffer.code(");")
  }

  fun insertOrThrow(database: DatabaseConnection, ruleId: TimeAllowanceRuleId) {
    val buffer = Buffer()
    writeInsert(buffer, ruleId)
    database.execSqlOrThrow(buffer.string())
  }
}