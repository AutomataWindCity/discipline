package com.example.app.database

import android.database.sqlite.SQLiteDatabase
import android.database.Cursor
import com.example.app.*

object AlwaysRulesTable {
  const val TABLE = "AlwaysRules"

  // INTEGER NOT NULL PRIMARY KEY
  // AlwaysRuleId: .id
  const val ID = "id"
  // INTEGER NOT NULL
  // RuleEnablerVariant: .enabler.variant
  const val ENABLER_VARIANT = "enabler_variant"
  // INTEGER NOT NULL
  // Duration: .enabler.countdownConditional.duration
  // Duration: .enabler.countdownAfterPleaConditional.intervalFromPleaTillDeactivation
  const val ENABLER_DATA_1 = "enabler_data_1"
  // INTEGER NOT NULL
  // OptionVariant: .enabler.countdownConditional.countdown.variant
  // OptionVariant: .enabler.countdownAfterPleaConditional.countdown.variant
  const val ENABLER_DATA_2 = "enabler_data_2"
  // INTEGER
  // Instant: .enabler.countdownConditional.countdown.some.from
  // Instant: .enabler.countdownAfterPleaConditional.countdown.some.from
  const val ENABLER_DATA_3 = "enabler_data_3"
  // INTEGER
  // Duration: .enabler.countdownConditional.countdown.some.duration
  // Duration: .enabler.countdownAfterPleaConditional.countdown.some.duration
  const val ENABLER_DATA_4 = "enabler_data_4"

  const val ID_INDEX = 0
  const val ENABLER_VARIANT_INDEX = 1
  const val ENABLER_DATA_1_INDEX = 2
  const val ENABLER_DATA_2_INDEX = 3
  const val ENABLER_DATA_3_INDEX = 4
  const val ENABLER_DATA_4_INDEX = 5

  val names = AlwaysRuleNames(
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
  )

  val indexes = AlwaysRuleIndexes(
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
    )
  )

  fun writeCreateTable(
    code: Buffer,
  ) {
    code.code("""
      CREATE TABLE IF NOT EXISTS $TABLE (
        $ID INTEGER PRIMARY KEY,
        $ENABLER_VARIANT INTEGER NOT NULL,
        $ENABLER_DATA_1 INTEGER NOT NULL,
        $ENABLER_DATA_2 INTEGER NOT NULL,
        $ENABLER_DATA_3 INTEGER,
        $ENABLER_DATA_4 INTEGER
      ) STRICT, WITHOUT ROWID;
    """)
  }

  fun writeInsertRule(
    buffer: Buffer,
    rule: AlwaysRule,
  ) {
    buffer.apply {
      code("INSERT INTO $TABLE VALUES (NULL, ")
      orderedAlwaysRule(rule)
      code(");")
    }
  }

  fun insertRuleOrThrow(
    database: DatabaseConnection,
    rule: AlwaysRule,
  ): AlwaysRuleId {
    val buffer = Buffer()
    writeInsertRule(buffer, rule)
    return AlwaysRuleId(database.insertOrThrow(buffer.string()))
  }

  fun writeDeleteRule(
    buffer: Buffer,
    ruleId: AlwaysRuleId,
  ) {
    buffer.apply {
      code("DELETE FROM $TABLE WHERE $ID = ")
      alwaysRuleId(ruleId)
      code(";")
    }
  }

  fun deleteRuleOrThrow(
    database: DatabaseConnection,
    ruleId: AlwaysRuleId,
  ) {
    val buffer = Buffer()
    writeDeleteRule(buffer, ruleId)
    database.execSqlOrThrow(buffer.string())
  }

  fun writeSelectRule(
    buffer: Buffer,
    ruleId: AlwaysRuleId,
  ) {
    buffer.apply {
      code("SELECT * FROM $TABLE WHERE $ID = ")
      alwaysRuleId(ruleId)
      code(";")
    }
  }

  // fun indexedReadRule(
  //   cursor: Cursor,
  // ): Triple<AlwaysRuleId, AlwaysRule, LocationId> {
  //   val id = cursor.readAlwaysRuleId(ID_INDEX)
  //   val rule = cursor.readAlwaysRule(indexes)
  //   val locationId = cursor.readLocationId(LOCATION_ID_INDEX)
  //   return Triple(id, rule, locationId)
  // }

  // fun selectRuleOrThrow(
  //   database: DatabaseConnection,
  //   ruleId: AlwaysRuleId,
  // ): AlwaysRule? {
  //   val buffer = Buffer()
  //   writeSelectRule(buffer, ruleId)

  //   val cursor = database.queryOrThrow(buffer.string())
  //   while (cursor.moveToNext()) {
  //     return indexedReadRule(cursor)
  //   }

  //   return null
  // }

  fun writeEnablerCountdownConditionaReactivate(
    buffer: Buffer,
    ruleId: AlwaysRuleId,
    reactivateState: CountdownConditional.ReactivateState,
  ) {
    buffer.apply {
      code("UPDATE ${TABLE} SET ")
      reactivateCountdownConditional(names.enabler.countdownConditional, reactivateState)
      code(" WHERE ${ID} = ")
      alwaysRuleId(ruleId)
      code(";")
    }
  }

  fun writeEnablerCountdownAfterPleaConditionalReactivate(
    buffer: Buffer,
    ruleId: AlwaysRuleId,
  ) {
    buffer.apply {
      code("UPDATE ${TABLE} SET ")
      reactivateCountdownAfterPleaConditional(names.enabler.countdownAfterPleaConditional)
      code(" WHERE ${ID} = ")
      alwaysRuleId(ruleId)
      code(";")
    }
  }

  fun writeEnablerCountdownAfterPleaConditionalReDeactivate(
    buffer: Buffer,
    ruleId: AlwaysRuleId,
    reDeactivateState: CountdownAfterPleaConditional.ReDeactivateState,
  ) {
    buffer.apply {
      code("UPDATE ${TABLE} SET ")
      reDeactivateCountdownAfterPleaConditional(names.enabler.countdownAfterPleaConditional, reDeactivateState)
      code(" WHERE ${ID} = ")
      alwaysRuleId(ruleId)
      code(";")
    }
  }

  fun enablerCountdownConditionalReactivate(
    database: DatabaseConnection,
    ruleId: AlwaysRuleId,
    reactivateState: CountdownConditional.ReactivateState,
  ) {
    val buffer = Buffer()
    writeEnablerCountdownConditionaReactivate(buffer, ruleId, reactivateState)
    database.execSqlOrThrow(buffer.string())
  }

  fun enablerCountdownAfterPleaConditionalReactivate(
    database: DatabaseConnection,
    ruleId: AlwaysRuleId,
  ) {
    val buffer = Buffer()
    writeEnablerCountdownAfterPleaConditionalReactivate(buffer, ruleId)
    database.execSqlOrThrow(buffer.string())
  }

  fun enablerCountdownAfterPleaConditionalReDeactivate(
    database: DatabaseConnection,
    ruleId: AlwaysRuleId,
    reDeactivateState: CountdownAfterPleaConditional.ReDeactivateState,
  ) {
    val buffer = Buffer()
    writeEnablerCountdownAfterPleaConditionalReDeactivate(buffer, ruleId, reDeactivateState)
    database.execSqlOrThrow(buffer.string())
  }
}