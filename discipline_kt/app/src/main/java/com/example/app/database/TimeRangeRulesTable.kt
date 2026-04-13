package com.example.app.database

import com.example.app.*

object TimeRangeRulesTable {
  const val TABLE = "TimeRangeRules"

  const val ID = "id"
  const val CONDITION_FROM = "condition_from"
  const val CONDITION_TILL = "condition_till"
  const val ENABLER_VARIANT = "enabler_variant"
  const val ENABLER_DATA_1 = "enabler_data_1"
  const val ENABLER_DATA_2 = "enabler_data_2"
  const val ENABLER_DATA_3 = "enabler_data_3"
  const val ENABLER_DATA_4 = "enabler_data_4"

  const val ID_INDEX = 0
  const val CONDITION_FROM_INDEX = 1
  const val CONDITION_TILL_INDEX = 2
  const val ENABLER_VARIANT_INDEX = 3
  const val ENABLER_DATA_1_INDEX = 4
  const val ENABLER_DATA_2_INDEX = 5
  const val ENABLER_DATA_3_INDEX = 6
  const val ENABLER_DATA_4_INDEX = 7

  val names = TimeRangeRuleNames(
    enabler = RuleEnablerNames(
      variant = ENABLER_VARIANT,
      countdownConditional = CountdownConditionalNames(
        duration = ENABLER_DATA_1,
        countdown = OptionNames(
          tag = ENABLER_DATA_2,
          some = CountdownNames(
            from = ENABLER_DATA_3,
            duration = ENABLER_DATA_4,
          ),
        )
      ),
      countdownAfterPleaConditional = CountdownAfterPleaConditionalNames(
        intervalFromPleaTillDeactivation = ENABLER_DATA_1,
        countdownTillDeactivation = OptionNames(
          tag = ENABLER_DATA_2,
          some = CountdownNames(
            from = ENABLER_DATA_3,
            duration = ENABLER_DATA_4,
          ),
        )
      ),
    ),
    condition = TimeRangeNames(
      from = CONDITION_FROM,
      till = CONDITION_TILL,
    ),
  )

  val indexes = TimeRangeRuleIndexes(
    enabler = RuleEnablerIndexes(
      variant = ENABLER_VARIANT_INDEX,
      countdownConditional = CountdownConditionalIndexes(
        duration = ENABLER_DATA_1_INDEX, 
        countdown = OptionIndexes(
          ENABLER_DATA_2_INDEX,
          CountdownIndexes(
            from = ENABLER_DATA_3_INDEX,
            duration = ENABLER_DATA_4_INDEX,
          ),
        ),
      ), 
      countdownAfterPleaConditional = CountdownAfterPleaConditionalIndexes(
        duration = ENABLER_DATA_1_INDEX, 
        countdown = OptionIndexes(
          ENABLER_DATA_2_INDEX,
          CountdownIndexes(
            from = ENABLER_DATA_3_INDEX,
            duration = ENABLER_DATA_4_INDEX,
          ),
        ),
      ),
    ), 
    condition = TimeRangeIndexes(
      from = CONDITION_FROM_INDEX, 
      till = CONDITION_TILL_INDEX,
    ),
  )


  fun writeCreateTable(
    buffer: Buffer,
  ) {
    buffer.apply { 
      code("""
        CREATE TABLE IF NOT EXISTS $TABLE (
          $ID INTEGER PRIMARY KEY,
          $CONDITION_FROM INTEGER NOT NULL,
          $CONDITION_TILL INTEGER NOT NULL,
          $ENABLER_VARIANT INTEGER NOT NULL,
          $ENABLER_DATA_1 INTEGER NOT NULL,
          $ENABLER_DATA_2 INTEGER NOT NULL,
          $ENABLER_DATA_3 INTEGER,
          $ENABLER_DATA_4 INTEGER
        ) STRICT, WITHOUT ROWID;
      """)
    }
  }

  fun writeInsertRule(
    buffer: Buffer,
    rule: TimeRangeRule,
  ) {
    buffer.apply {
      code("INSERT INTO $TABLE VALUES (NULL, ")
      orderedTimeRangeRule(rule)
      code(");")
    }
  }

  fun insertRuleOrThrow(
    database: DatabaseConnection,
    rule: TimeRangeRule,
  ): TimeRangeRuleId {
    val buffer = Buffer()
    writeInsertRule(buffer, rule)
    return TimeRangeRuleId(database.insertOrThrow(buffer.string()))
  }

  fun writeDeleteRule(
    buffer: Buffer,
    ruleId: TimeRangeRuleId,
  ) {
    buffer.apply {
      code("DELETE FROM $TABLE WHERE $ID = ")
      timeRangeRuleId(ruleId)
      code(");")
    }
  }

  fun deleteRuleOrThrow(
    database: DatabaseConnection,
    ruleId: TimeRangeRuleId,
  ) {
    val buffer = Buffer()
    writeDeleteRule(buffer, ruleId)
    database.execSqlOrThrow(buffer.string())
  }

  fun writeEnablerCountdownConditionaReactivate(
    buffer: Buffer,
    ruleId: TimeRangeRuleId,
    reactivateState: CountdownConditional.ReactivateState,
  ) {
    buffer.apply {
      code("UPDATE ${TABLE} SET ")
      reactivateCountdownConditional(names.enabler.countdownConditional, reactivateState)
      code(" WHERE ${ID} = ")
      timeRangeRuleId(ruleId)
      code(";")
    }
  }

  fun writeEnablerCountdownAfterPleaConditionalReactivate(
    buffer: Buffer,
    ruleId: TimeRangeRuleId,
  ) {
    buffer.apply {
      code("UPDATE ${TABLE} SET ")
      reactivateCountdownAfterPleaConditional(names.enabler.countdownAfterPleaConditional)
      code(" WHERE ${ID} = ")
      timeRangeRuleId(ruleId)
      code(";")
    }
  }

  fun writeEnablerCountdownAfterPleaConditionalReDeactivate(
    buffer: Buffer,
    ruleId: TimeRangeRuleId,
    reDeactivateState: CountdownAfterPleaConditional.ReDeactivateState,
  ) {
    buffer.apply {
      code("UPDATE ${TABLE} SET ")
      reDeactivateCountdownAfterPleaConditional(names.enabler.countdownAfterPleaConditional, reDeactivateState)
      code(" WHERE ${ID} = ")
      timeRangeRuleId(ruleId)
      code(";")
    }
  }

  fun enablerCountdownConditionalReactivate(
    database: DatabaseConnection,
    ruleId: TimeRangeRuleId,
    reactivateState: CountdownConditional.ReactivateState,
  ) {
    val buffer = Buffer()
    writeEnablerCountdownConditionaReactivate(buffer, ruleId, reactivateState)
    database.execSqlOrThrow(buffer.string())
  }

  fun enablerCountdownAfterPleaConditionalReactivate(
    database: DatabaseConnection,
    ruleId: TimeRangeRuleId,
  ) {
    val buffer = Buffer()
    writeEnablerCountdownAfterPleaConditionalReactivate(buffer, ruleId)
    database.execSqlOrThrow(buffer.string())
  }

  fun enablerCountdownAfterPleaConditionalReDeactivate(
    database: DatabaseConnection,
    ruleId: TimeRangeRuleId,
    reDeactivateState: CountdownAfterPleaConditional.ReDeactivateState,
  ) {
    val buffer = Buffer()
    writeEnablerCountdownAfterPleaConditionalReDeactivate(buffer, ruleId, reDeactivateState)
    database.execSqlOrThrow(buffer.string())
  }
}