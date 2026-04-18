package com.example.app.database

import com.example.app.*

object MainUserProfileApplicationRegulationAlwaysRulesLinkingTable {
  const val TABLE = "MainUserProfileApplicationRegulationAlwaysRulesLinkingTable"

  const val RULE_ID = "rule_id"
  const val REGULATION_ID = "regulation_id"

  fun writeCreateTable(buffer: Buffer) {
    buffer.code("""
      CREATE IF NOT EXISTS $TABLE (
        $RULE_ID INTEGER NOT NULL,
        $REGULATION_ID INTEGER NOT NULL,
        PRIMARY KEY($RULE_ID, $REGULATION_ID),
        FOREIGN KEY ($RULE_ID) REFERENCES ${AlwaysRulesTable.TABLE}(${AlwaysRulesTable.ID}) ON DELETE CASCADE,
        FOREIGN KEY ($REGULATION_ID) REFERENCES ${ApplicationRegulationsTable.TABLE}(${ApplicationRegulationsTable.ID}) ON DELETE CASCADE 
      ) STRICT, WITHOUT ROWID;
    """)
  }

  fun writeInsert(buffer: Buffer, regulationId: ApplicationRegulationId, ruleId: AlwaysRuleId) {
    buffer.code("INSERT INTO $TABLE VALUES (")
    buffer.alwaysRuleId(ruleId)
    buffer.comma()
    buffer.applicationRegulationId(regulationId)
    buffer.code(");")
  }

  fun insertOrThrow(database: DatabaseConnection, regulationId: ApplicationRegulationId, ruleId: AlwaysRuleId) {
    val buffer = Buffer()
    writeInsert(buffer, regulationId, ruleId)
    database.execSqlOrThrow(buffer.string())
  }
}