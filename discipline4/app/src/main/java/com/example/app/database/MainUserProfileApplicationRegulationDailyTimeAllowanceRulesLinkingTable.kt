package com.example.app.database

object MainUserProfileApplicationDailyTimeAllowanceRulesLinkingTable {
  const val TABLE = "MainUserProfileApplicationDailyTimeAllowanceRulesLinkingTable"

  const val RULE_ID = "rule_id"
  const val REGULATION_ID = "regulation_id"

  fun writeCreateTable(buffer: Buffer) {
    buffer.code("""
      CREATE IF NOT EXISTS $TABLE (
        $RULE_ID INTEGER NOT NULL,
        $REGULATION_ID INTEGER NOT NULL,
        FOREIGN KEY ($RULE_ID) REFERENCES ${TimeAllowanceRulesTable.TABLE}(${TimeAllowanceRulesTable.ID}) ON DELETE CASCADE,
        FOREIGN KEY ($REGULATION_ID) REFERENCES ${ApplicationRegulationsTable.TABLE}(${ApplicationRegulationsTable.ID}) ON DELETE CASCADE 
      ) STRICT, WITHOUT ROWID;
    """)
  }

  fun writeInsert(buffer: Buffer, regulationId: ApplictionRegulationId, ruleId: TimeAllowanceRuleId) {
    buffer.code("INSERT INTO $TABLE VALUES (")
    buffer.applicationRegulationId(regulationId)
    comma()
    buffer.timeAllowanceRuleId(ruleId)
    buffer.code(");")
  }

  fun insertOrThrow(database: DatabaseConnection, regulationId: ApplictionRegulationId, ruleId: TimeAllowanceRuleId) {
    val buffer = Buffer()
    writeInsert(buffer, regulationId, ruleId)
    database.execSqlOrThrow(buffer.string())
  }
}