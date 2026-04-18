package com.example.app.procedures.countdownconditional

import com.example.app.*
import com.example.app.database.*

sealed class ReactivateReturn {
  class Database(val error: Throwable) : ReactivateReturn() {}
  class Success() : ReactivateReturn() {}
}

fun reactivate(
  database: DatabaseConnection,
  adapter: CountdownConditionalDbAdapter,
  location: CountdownConditionalLocation,
  conditional: CountdownConditional,
  clock: MonotonicClock,
): ReactivateReturn {
  val now = clock.getNow()
  val reactivateState = conditional.createReactivateState(now)

  try {
    adapter.reactivateOrThrow(database, location, reactivateState)
  } catch (exception: Throwable) {
    return ReactivateReturn.Database(exception)
  }

  conditional.reactivateFromState(reactivateState)
  return ReactivateReturn.Success()
}
