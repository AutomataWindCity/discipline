package com.example.app.database

object MainUserProfileApplicationTimeRangeRulesLinkingTable {
  const val TABLE = "MainUserProfileApplicationTimeRangeRulesLinkingTable"

  const val RULE_ID = "rule_id"
  const val REGULATION_ID = "regulation_id"

  fun writeCreateTable(buffer: Buffer) {
    buffer.code("""
      CREATE IF NOT EXISTS $TABLE (
        $RULE_ID INTEGER NOT NULL,
        $REGULATION_ID INTEGER NOT NULL,
        FOREIGN KEY ($RULE_ID) REFERENCES ${TimeRangeRulesTable.TABLE}(${TimeRangeRulesTable.ID}) ON DELETE CASCADE,
        FOREIGN KEY ($REGULATION_ID) REFERENCES ${ApplicationRegulationsTable.TABLE}(${ApplicationRegulationsTable.ID}) ON DELETE CASCADE 
      ) STRICT, WITHOUT ROWID;
    """)
  }

  fun writeInsert(buffer: Buffer, regulationId: ApplictionRegulationId, ruleId: TimeRangeRuleId) {
    buffer.code("INSERT INTO $TABLE VALUES (")
    buffer.applicationRegulationId(regulationId)
    comma()
    buffer.timeRangeRuleId(ruleId)
    buffer.code(");")
  }

  fun insertOrThrow(database: DatabaseConnection, regulationId: ApplictionRegulationId, ruleId: TimeRangeRuleId) {
    val buffer = Buffer()
    writeInsert(buffer, regulationId, ruleId)
    database.execSqlOrThrow(buffer.string())
  }
}