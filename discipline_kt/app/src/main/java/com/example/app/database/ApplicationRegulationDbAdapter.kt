package com.example.app.database

import com.example.app.*

object ApplicationRegulationDbAdapter {
  fun createOrThrow(
    database: DatabaseConnection,
    location: ApplicationRegulationLocation,
    applicationName: ApplicationName,
  ): ApplicationRegulationId {
    when (location) {
      is ApplicationRegulationLocation.MainUserProfile -> {
        return ApplicationRegulationsTable.insertOrThrow(database, applicationName)
      }
    }
  }

  fun deleteOrThrow(
    database: DatabaseConnection,
    location: ApplicationRegulationLocation,
    applicationName: ApplicationName,
  ) {
    when (location) {
      is ApplicationRegulationLocation.MainUserProfile -> {
        ApplicationRegulationsTable.deleteOrThrow(database, applicationName)
      }
    }
  }
}