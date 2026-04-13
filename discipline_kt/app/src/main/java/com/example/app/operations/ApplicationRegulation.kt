package com.example.app

import com.example.app.database.*

class ApplicationRegulationGroupId() {}
class ApplicationRegulationGroupLocationInfo() {}
class ApplicationRegulationGroupsMap() {}

object ApplicationRegulationProcedure {
  sealed class CreateReturn {
    class TooManyRegulations() : CreateReturn() {}
    class ApplicationAlreadyRegulated() : CreateReturn() {}
    class InternalError(val error: Throwable) : CreateReturn() {}
    class Success(val app: ApplicationName, val regulation: ApplicationRegulation) : CreateReturn() {}
  }

  fun create(
    state: State,
    database: Database,
    location: ApplicationRegulationLocation,
    applicationName: ApplicationName,
  ): CreateReturn {
    if (state.applicationRegulationsStats.isFull()) {
      return CreateReturn.TooManyRegulations()
    }
    
    if (state.mainUserProfile.applicationRegulations.has(applicationName)) {
      return CreateReturn.ApplicationAlreadyRegulated()
    }

    val regulation = ApplicationRegulation.createDefault()

    try {
      database.createApplicationRegulation(location, applicationName)
    } catch (exception: Throwable) {
      return CreateReturn.InternalError(exception)
    }

    state.mainUserProfile.applicationRegulations.add(applicationName, regulation)
    state.applicationRegulationsStats.applicationRegulationsNumber += 1
    return CreateReturn.Success(applicationName, regulation)
  }

  sealed class DeleteReturn {
    class NoSuchApplicationRegulation() : DeleteReturn() {}
    class PermissionDenied() : DeleteReturn() {}
    class InternalError(val error: Throwable) : DeleteReturn() {}
    class Success(val rule: ApplicationRegulation) : DeleteReturn() {}
  }

  fun delete(
    state: State,
    database: Database,
    location: ApplicationRegulationLocation,
    applicationName: ApplicationName,
  ): DeleteReturn {
    val regulations = state.mainUserProfile.applicationRegulations
    val stats = state.applicationRegulationsStats

    val regulation = regulations.get(applicationName)
      ?: return DeleteReturn.NoSuchApplicationRegulation()

    val now = state.getMonotonicNow()
    if (regulation.isEnabled(now)) {
      return DeleteReturn.PermissionDenied()
    }

    try {
      database.deleteApplicationRegulation(location, applicationName)
    } catch (exception: Throwable) {
      return DeleteReturn.InternalError(exception)
    }

    regulations.remove(applicationName)
    stats.applicationRegulationsNumber -= 1
    return DeleteReturn.Success(regulation)
  }
}