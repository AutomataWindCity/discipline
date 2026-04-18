package com.example.app

object ApplicationRegulationProcedure {
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
  ) {
    if (regulationsStats.isFull()) {
      return CreateReturn.TooManyRegulations()
    }
    
    if (context.regulations.has(applicationName)) {
      return CreateReturn.DuplicateRegulationId()
    }

    val regulation = ApplicationRegulation.createDefault()

    try {
      adapter.createOrThrow(database, location, applicationName, regulation)
    } catch (exception: Throwable) {
      return CreateReturn.InternalError(exception)
    }

    regulations.add(app, regulation)
    regulationsStats.applicationRegulationsNumber += 1
    return CreateReturn.Success(app, regulation)
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
      return Return.InternalError(exception)
    }

    regulations.remove(app)
    regulationsStats.applicationRegulationsNumber -= 1
    return DeleteReturn.Success(regulation)
  }
}