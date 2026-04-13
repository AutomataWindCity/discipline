package com.example.app

import com.example.app.database.*

object CountdownAfterPleaConditionalProcedures {
  sealed class ReactivateReturn {
    class NoSuchConditional(val value: GetCountdownAfterPleaConditionalError) : ReactivateReturn() {}
    class Database(val error: Throwable) : ReactivateReturn() {}
    class Success() : ReactivateReturn() {}
  }

  fun reactivate(
    state: State,
    database: Database,
    locator: CountdownAfterPleaConditionalLocation,
  ): ReactivateReturn {
    val conditional = state.getCountdownAfterPleaConditional(locator).let {
      when (it) {
        is Tried.Success -> it.value,
        is Tried.Failure -> return ReactivateReturn.NoSuchConditional(it.error)
      }
    }

    try {
      database.reactivateCountdownAfterPleaConditional(locator)
    } catch (exception: Throwable) {
      return ReactivateReturn.Database(exception)
    }

    conditional.reactivate()
    return ReactivateReturn.Success()
  }

  sealed class ReDeactivateReturn {
    class NonExisting(val error: GetCountdownAfterPleaConditionalError) : ReDeactivateReturn() {}
    class Database(val error: Throwable) : ReDeactivateReturn() {}
    class Success() : ReDeactivateReturn() {}
  }

  fun reDeactivate(
    state: State,
    database: Database,
    locator: CountdownAfterPleaConditionalLocation,
  ): ReDeactivateReturn {
    val conditional = state.getCountdownAfterPleaConditional(locator).let {
      when (it) {
        is Tried.Success -> it.value,
        is Tried.Failure -> return ReDeactivateReturn.NonExisting(it.error),
      }
    }

    val now = state.getMonotonicNow()
    val reDeactivateState = conditional.createReDeactivateState(now)

    try {
      database.reDeactivateCountdownAfterPleaConditional(locator, reDeactivateState)
    } catch (exception: Throwable) {
      return ReDeactivateReturn.Database(exception)
    }

    conditional.reDeactivateFromState(reDeactivateState)
    return ReDeactivateReturn.Success()
  }
}