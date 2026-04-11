package com.example.app.database

import com.example.app.*

object TimeAllowanceRulesTable {
  const val TABLE = "TimeAllowanceRules"

  const val ID = "id"
  const val ALLOWANCE = "allowance"
  const val ENABLER_TYPE = "enabler_tag"
  const val ENABLER_DATA_1 = "enabler_data_1"
  const val ENABLER_DATA_2 = "enabler_data_2"
  const val ENABLER_DATA_3 = "enabler_data_3"
  const val ENABLER_DATA_4 = "enabler_data_4"
  const val ENABLER_DATA_5 = "enabler_data_5"
  const val LOCATION_ID = "location_id"

  const val ID_INDEX = 0
  const val ALLOWANCE_INDEX = 1
  const val ENABLER_TYPE_INDEX = 2
  const val ENABLER_DATA_1_INDEX = 3
  const val ENABLER_DATA_2_INDEX = 4
  const val ENABLER_DATA_3_INDEX = 5
  const val ENABLER_DATA_4_INDEX = 6
  const val ENABLER_DATA_5_INDEX = 7
  const val LOCATION_ID_INDEX = 8

  val names = TimeAllowanceRuleNames(
    enabler = RuleEnablerNames(
      variant = ENABLER_TYPE,
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

    allowance = ALLOWANCE,
  )

  val indexes = TimeAllowanceRuleIndexes(
    allowance = ALLOWANCE_INDEX,
    enabler = RuleEnablerIndexes(
      variant = ENABLER_TYPE_INDEX,
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
  )


  fun writeCreateTable(buffer: Buffer) {
    buffer.write("""
      CREATE TABLE IF NOT EXISTS $TABLE (
        $ID INTEGER PRIMARY KEY,
        $ALLOWANCE INTEGER NOT NULL,
        $ENABLER_TYPE INTEGER NOT NULL,
        $ENABLER_DATA_1 INTEGER NOT NULL,
        $ENABLER_DATA_2 INTEGER NOT NULL,
        $ENABLER_DATA_3 INTEGER,
        $ENABLER_DATA_4 INTEGER,
        $LOCATION_ID INTEGER NOT NULL
      ) STRICT, WITHOUT ROWID;
    """)
  }

  fun writeInsertRule(
    buffer: Buffer,
    ruleId: TimeAllowanceRuleId,
    rule: TimeAllowanceRule,
    locationId: LocationId,
  ) {
    buffer.apply {
      write("INSERT INTO $TABLE VALUES (")
      timeAllowanceRuleId(ruleId)
      comma()
      orderedTimeAllowanceRule(rule)
      comma()
      ruleLocationId(locationId)
      code(");")
    }
  }

  fun insertRuleOrThrow(
    database: DatabaseConnection,
    ruleId: TimeAllowanceRuleId,
    rule: TimeAllowanceRule,
    locationId: LocationId,
  ) {
    val buffer = Buffer()
    writeInsertRule(buffer, ruleId, rule, locationId)
    database.execSqlOrThrow(buffer.string())
  }

  fun writeDeleteRule(
    buffer: Buffer,
    ruleId: TimeAllowanceRuleId,
  ) {
    buffer.apply {
      code("DELETE FROM $TABLE WHERE $ID = ")
      timeAllowanceRuleId(ruleId)
      code(";")
    }
  }

  fun deleteRuleOrThrow(
    database: DatabaseConnection,
    ruleId: TimeAllowanceRuleId,
  ) {
    val buffer = Buffer()
    writeDeleteRule(buffer, ruleId)
    database.execSqlOrThrow(buffer.string())
  }

  fun writeEnablerCountdownConditionaReactivate(
    buffer: Buffer,
    ruleId: TimeAllowanceRuleId,
    reactivateState: CountdownConditional.ReactivateState,
  ) {
    buffer.apply {
      code("UPDATE ${TABLE} SET ")
      reactivateCountdownConditional(names.enabler.countdownConditional, reactivateState)
      code(" WHERE ${ID} = ")
      timeAllowanceRuleId(ruleId)
      code(";")
    }
  }

  fun writeEnablerCountdownAfterPleaConditionalReactivate(
    buffer: Buffer,
    ruleId: TimeAllowanceRuleId,
  ) {
    buffer.apply {
      code("UPDATE ${TABLE} SET ")
      reactivateCountdownAfterPleaConditional(names.enabler.countdownAfterPleaConditional)
      code(" WHERE ${ID} = ")
      timeAllowanceRuleId(ruleId)
      code(";")
    }
  }

  fun writeEnablerCountdownAfterPleaConditionalReDeactivate(
    buffer: Buffer,
    ruleId: TimeAllowanceRuleId,
    reDeactivateState: CountdownAfterPleaConditional.ReDeactivateState,
  ) {
    buffer.apply {
      code("UPDATE ${TABLE} SET ")
      reDeactivateCountdownAfterPleaConditional(names.enabler.countdownAfterPleaConditional, reDeactivateState)
      code(" WHERE ${ID} = ")
      timeAllowanceRuleId(ruleId)
      code(";")
    }
  }

  fun enablerCountdownConditionalReactivate(
    database: DatabaseConnection,
    ruleId: TimeAllowanceRuleId,
    reactivateState: CountdownConditional.ReactivateState,
  ) {
    val buffer = Buffer()
    writeEnablerCountdownConditionaReactivate(buffer, ruleId, reactivateState)
    database.execSqlOrThrow(buffer.string())
  }

  fun enablerCountdownAfterPleaConditionalReactivate(
    database: DatabaseConnection,
    ruleId: TimeAllowanceRuleId,
  ) {
    val buffer = Buffer()
    writeEnablerCountdownAfterPleaConditionalReactivate(buffer, ruleId)
    database.execSqlOrThrow(buffer.string())
  }

  fun enablerCountdownAfterPleaConditionalReDeactivate(
    database: DatabaseConnection,
    ruleId: TimeAllowanceRuleId,
    reDeactivateState: CountdownAfterPleaConditional.ReDeactivateState,
  ) {
    val buffer = Buffer()
    writeEnablerCountdownAfterPleaConditionalReDeactivate(buffer, ruleId, reDeactivateState)
    database.execSqlOrThrow(buffer.string())
  }
}
