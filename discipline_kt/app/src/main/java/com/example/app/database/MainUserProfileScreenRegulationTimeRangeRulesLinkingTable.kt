package com.example.app.database

import com.example.app.*

object MainUserProfileScreenRegulationTimeRangeRulesLinkingTable {
  const val TABLE = "MainUserProfileScreenRegulationTimeRangeRulesLinkingTable"

  const val ID = "id"
  const val ID_INDEX = 0

  fun writeCreateTable(buffer: Buffer) {
    buffer.code("""
      CREATE IF NOT EXISTS $TABLE (
        $ID INTEGER PRIMARY KEY,
        FOREIGN KEY ($ID) REFERENCES ${TimeRangeRulesTable.TABLE}(${TimeRangeRulesTable.ID}) ON DELETE CASCADE 
      ) STRICT, WITHOUT ROWID;
    """)
  }

  fun writeInsert(buffer: Buffer, ruleId: TimeRangeRuleId) {
    buffer.code("INSERT INTO $TABLE VALUES (")
    buffer.timeRangeRuleId(ruleId)
    buffer.code(");")
  }

  fun insertOrThrow(database: DatabaseConnection, ruleId: TimeRangeRuleId) {
    val buffer = Buffer()
    writeInsert(buffer, ruleId)
    database.execSqlOrThrow(buffer.string())
  }
}