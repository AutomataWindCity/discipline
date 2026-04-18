package com.example.app.database

import com.example.app.*

object MainUserProfileScreenRegulationAlwaysRulesLinkingTable {
  const val TABLE = "MainUserProfileScreenRegulationAlwaysRulesLinkingTable"

  const val ID = "id"
  const val ID_INDEX = 0

  fun writeCreateTable(buffer: Buffer) {
    buffer.code("""
      CREATE IF NOT EXISTS $TABLE (
        $ID INTEGER PRIMARY KEY,
        FOREIGN KEY ($ID) REFERENCES ${AlwaysRulesTable.TABLE}(${AlwaysRulesTable.ID}) ON DELETE CASCADE 
      ) STRICT, WITHOUT ROWID;
    """)
  }

  fun writeInsert(buffer: Buffer, ruleId: AlwaysRuleId) {
    buffer.code("INSERT INTO $TABLE VALUES (")
    buffer.alwaysRuleId(ruleId)
    buffer.code(");")
  }

  fun insertOrThrow(database: DatabaseConnection, ruleId: AlwaysRuleId) {
    val buffer = Buffer()
    writeInsert(buffer, ruleId)
    database.execSqlOrThrow(buffer.string())
  }
}