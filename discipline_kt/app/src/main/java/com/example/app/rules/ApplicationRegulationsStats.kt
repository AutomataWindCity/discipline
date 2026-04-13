package com.example.app

import androidx.room.Entity

@Entity
public data class ApplicationRegulationsStats(
  var applicationRegulationsNumber: Int,
  var maximumApplicationRegulationsNumber: Int,
) {
  fun isFull(): Boolean {
    return applicationRegulationsNumber >= maximumApplicationRegulationsNumber
  }
}