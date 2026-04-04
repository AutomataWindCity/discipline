package com.example.app.database

public class Buffer(val string: StringBuilder = StringBuilder()) {
  fun write(slice: String) {

  }

  fun builder(): StringBuilder {
    return string
  }

  fun string(): String {
    return string.toString()
  }
}
