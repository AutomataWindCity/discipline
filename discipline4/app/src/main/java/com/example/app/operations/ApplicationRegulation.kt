package com.example.app.procedures.applicationregulation

import com.example.app.*
import com.example.app.database.*

sealed class CreateReturn {
  class TooManyRegulations() : CreateReturn() {}
  class DuplicateRegulationId() : CreateReturn() {}
  class InternalError(val error: Throwable) : CreateReturn() {}
  class Success(val app: ApplicationName, val regulation: ApplicationRegulation) : CreateReturn() {}
}

fun create(
  database: DatabaseConnection,
  adapter: ApplicationRegulationDbAdapter,
  location: ApplicationRegulationLocation,
  regulations: ApplicationRegulations,
  regulationsStats: ApplicationRegulationsStats,
  applicationName: ApplicationName,
): CreateReturn {
  if (regulationsStats.isFull()) {
    return CreateReturn.TooManyRegulations()
  }
  
  if (regulations.has(applicationName)) {
    return CreateReturn.DuplicateRegulationId()
  }

  val regulation = ApplicationRegulation.createDefault()

  try {
    adapter.createOrThrow(database, location, applicationName)
  } catch (exception: Throwable) {
    return CreateReturn.InternalError(exception)
  }

  regulations.add(applicationName, regulation)
  regulationsStats.applicationRegulationsNumber += 1
  return CreateReturn.Success(applicationName, regulation)
}

sealed class DeleteReturn {
  class NoSuchApplicationRegulation() : DeleteReturn() {}
  class PermissionDenied() : DeleteReturn() {}
  class InternalError(val error: Throwable) : DeleteReturn() {}
  class Success(val rule: ApplicationRegulation) : DeleteReturn() {}
}

fun delete(
  database: DatabaseConnection,
  adapter: ApplicationRegulationDbAdapter,
  location: ApplicationRegulationLocation,
  regulations: ApplicationRegulations,
  regulationsStats: ApplicationRegulationsStats,
  applicationName: ApplicationName,
  clock: MonotonicClock,
): DeleteReturn {
  val regulation = regulations.get(applicationName)
    ?: return DeleteReturn.NoSuchApplicationRegulation()

  val now = clock.getNow()
  if (regulation.isEnabled(now)) {
    return DeleteReturn.PermissionDenied()
  }

  try {
    adapter.deleteOrThrow(database, location, applicationName)
  } catch (exception: Throwable) {
    return DeleteReturn.InternalError(exception)
  }

  regulations.remove(applicationName)
  regulationsStats.applicationRegulationsNumber -= 1
  return DeleteReturn.Success(regulation)
}