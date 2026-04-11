package com.example.app.database

public class Buffer(val string: StringBuilder = StringBuilder()) {
  fun code(slice: String) {
    string.append(slice)
  }

  fun builder(): StringBuilder {
    return string
  }

  fun string(): String {
    return string.toString()
  }
}
