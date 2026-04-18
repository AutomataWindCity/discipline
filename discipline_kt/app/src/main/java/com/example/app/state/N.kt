package com.example.app

fun State.getMonotonicNow(): Instant {
  return monotonicClock.getNow()
}

fun State.getAlwaysRule() {
  
}