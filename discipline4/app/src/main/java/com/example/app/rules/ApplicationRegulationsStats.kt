package com.example.app

public data class ApplicationRegulationsStats(
  val applicationRegulationsNumber: Int,
  val maximumApplicationRegulationsNumber: Int,
) {
  fun isFull(): Boolean {
    return applicationRegulationsNumber >= maximumApplicationRegulationsNumber
  }
}