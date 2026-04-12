package com.example.procedures.countdownafterpleaconditional

import com.example.app.*
import com.example.app.database.*

sealed class ReactivateReturn {
  class Database(val error: Throwable) : ReactivateReturn() {}
  class Success() : ReactivateReturn() {}
}

fun reactivate(
  database: DatabaseConnection,
  adapter: CountdownAfterPleaConditionalDbAdapter,
  location: CountdownAfterPleaConditionalLocation,
  conditional: CountdownAfterPleaConditional,
): ReactivateReturn {
  try {
    adapter.reactivateOrThrow(database, location)
  } catch (exception: Throwable) {
    return ReactivateReturn.Database(exception)
  }

  conditional.reactivate()
  return ReactivateReturn.Success()
}

sealed class ReDeactivateReturn {
  class Database(val error: Throwable) : ReDeactivateReturn() {}
  class Success() : ReDeactivateReturn() {}
}

fun reDeactivate(
  database: DatabaseConnection,
  adapter: CountdownAfterPleaConditionalDbAdapter,
  location: CountdownAfterPleaConditionalLocation,
  conditional: CountdownAfterPleaConditional,
  clock: MonotonicClock,
): ReDeactivateReturn {
  val now = clock.getNow()
  val reDeactivateState = conditional.createReDeactivateState(now)

  try {
    adapter.reDeactivateOrThrow(database, location, reDeactivateState)
  } catch (exception: Throwable) {
    return ReDeactivateReturn.Database(exception)
  }

  conditional.reDeactivateFromState(reDeactivateState)
  return ReDeactivateReturn.Success()
}