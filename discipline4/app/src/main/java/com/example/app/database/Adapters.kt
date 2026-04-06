package com.example.app

class AlwaysRulesTable {
  companion object {
    const val TABLE = ""

    const val ID = ""
    const val ENABLER_TYPE = ""
    const val ENABLER_DATA_1 = ""
    const val ENABLER_DATA_2 = ""
    const val ENABLER_DATA_3 = ""
    const val LOCATION_ID = ""

    const val ENABLER_COUNTDOWN_DURATION = ENABLER_DATA_1
    const val ENABLER_COUNTDOWN_COUNTDOWN_FROM = ENABLER_DATA_2
    const val ENABLER_COUNTDOWN_COUNTDOWN_TILL = ENABLER_DATA_3
  }

  fun writeCreateTable() {

  }

  fun writeAddRule() {}
}

class TimeRangeRulesTable {
  companion object {
    const val TABLE = ""

    const val ID = ""
    const val ENABLER_TYPE = ""
    const val ENABLER_DATA_1 = ""
    const val ENABLER_DATA_2 = ""
    const val ENABLER_DATA_3 = ""

    const val ENABLER_COUNTDOWN_DURATION = ENABLER_DATA_1
    const val ENABLER_COUNTDOWN_COUNTDOWN_FROM = ENABLER_DATA_2
    const val ENABLER_COUNTDOWN_COUNTDOWN_TILL = ENABLER_DATA_3
  }
}

class MainUserScreenRegulationTimeAllowanceRulesTable {
  companion object {
    const val TABLE = ""

    const val ID = ""
    const val ENABLER_TYPE = ""
    const val ENABLER_DATA_1 = ""
    const val ENABLER_DATA_2 = ""
    const val ENABLER_DATA_3 = ""

    const val ENABLER_COUNTDOWN_DURATION = ENABLER_DATA_1
    const val ENABLER_COUNTDOWN_COUNTDOWN_FROM = ENABLER_DATA_2
    const val ENABLER_COUNTDOWN_COUNTDOWN_TILL = ENABLER_DATA_3
  }
}

class CountdownConditionalDbAdapter {
  fun reactivateOrThrow(
    database: Database,
    location: CountdownConditionalLocation,
    reactivateState: CountdownConditional.ReactivateState,
  ) {}
}


class CountdownAfterPleaConditionalDbAdapter {
  fun reactivateOrThrow(
    database: Database,
    location: CountdownAfterPleaConditionalLocation,
  ) {}

  fun reDeactivateOrThrow(
    database: Database,
    location: CountdownAfterPleaConditionalLocation,
    reDeactivateState: CountdownAfterPleaConditional.ReDeactivateState
  ) {}
}

class AlwaysRuleDbAdapter {
  fun createOrThrow(
    database: Database,
    location: AlwaysRuleLocation,
    ruleId: UuidV4,
    rule: AlwaysRule,
  ) {

  }

  fun deleteOrThrow(
    database: Database,
    location: AlwaysRuleLocation,
    ruleId: UuidV4,
  ) {

  }
}

class TimeRangeRuleDbAdapter {
  fun createOrThrow(
    database: Database,
    location: TimeRangeRuleLocation,
    ruleId: UuidV4,
    rule: TimeRangeRule,
  ) {

  }

  fun deleteOrThrow(
    database: Database,
    location: TimeRangeRuleLocation,
    ruleId: UuidV4,
  ) {

  }
}

class TimeAllowanceRuleDbAdapter {
  fun createOrThrow(
    database: Database,
    location: TimeAllowanceRuleLocation,
    ruleId: UuidV4,
    rule: TimeAllowanceRule,
  ) {

  }

  fun deleteOrThrow(
    database: Database,
    location: TimeAllowanceRuleLocation,
    ruleId: UuidV4,
  ) {

  }
}

class ApplicationRegulationDbAdapter {
  fun createOrThrow(
    database: Database,
    location: ApplicationRegulationLocation,
    applicationName: ApplicationName,
    regulation: ApplicationRegulation,
  ) {

  }

  fun deleteOrThrow(
    database: Database,
    location: ApplicationRegulationLocation,
    applicationName: ApplicationName,
  ) {

  }
}