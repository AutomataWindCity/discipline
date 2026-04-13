package com.example.app

import com.example.app.database.*

object CountdownConditionalProcedures {
  sealed class ReactivateReturn {
    class NonExisting(val it: GetCountdownConditionalError) : ReactivateReturn() {}
    class Database(val it: Throwable) : ReactivateReturn() {}
    class Success() : ReactivateReturn() {}
  }

  fun reactivate(
    state: State,
    database: Database,
    locator: CountdownConditionalLocation,
  ): ReactivateReturn {
    val conditional = state.getCountdownConditional(locator).let {
      when (it) {
        is Tried.Success -> it.value,
        is Tried.Failure -> return ReactivateReturn.NonExisting(it.error),
      }
    }

    val now = state.getMonotonicNow()
    val reactivateState = conditional.createReactivateState(now)

    try {
      database.reactivateCountdownConditional(locator, reactivateState)
    } catch (exception: Throwable) {
      return ReactivateReturn.Database(exception)
    }

    conditional.reactivateFromState(reactivateState)
    return ReactivateReturn.Success()
  }
}