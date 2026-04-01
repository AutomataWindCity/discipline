package com.example.app

public sealed class Tried<out T, out E> {
  public data class Success<T>(public val value: T) : Tried<T, Nothing>()
  public data class Failure<E>(public val error: E) : Tried<Nothing, E>()

  public companion object {
    public fun <T> success(value: T): Tried<T, Nothing> = Success(value)

    public fun <E> failure(error: E): Tried<Nothing, E> = Failure(error)

    public inline fun <T> catching(
      block: () -> T,
    ): Tried<T, Throwable> = try {
      Success(block())
    } catch (throwable: Throwable) {
      Failure(throwable)
    }

    public inline fun <T, E> catching(
      errorTransform: (Throwable) -> E,
      block: () -> T,
    ): Tried<T, E> = try {
      Success(block())
    } catch (throwable: Throwable) {
      Failure(errorTransform(throwable))
    }
  }

  public inline fun <U> map(transform: (T) -> U): Tried<U, E> = when (this) {
    is Success -> Success(transform(value))
    is Failure -> this
  }

  public inline fun <U> flatMap(transform: (T) -> Tried<U, @UnsafeVariance E>): Tried<U, E> = when (this) {
    is Success -> transform(value)
    is Failure -> this
  }

  public inline fun <F> mapError(transform: (E) -> F): Tried<T, F> = when (this) {
    is Success -> this
    is Failure -> Failure(transform(error))
  }

  public inline fun recover(transform: (E) -> @UnsafeVariance T): Tried<T, Nothing> = when (this) {
    is Success -> this
    is Failure -> Success(transform(error))
  }

  public inline fun <F> recoverWith(transform: (E) -> Tried<@UnsafeVariance T, F>): Tried<T, F> = when (this) {
    is Success -> this
    is Failure -> transform(error)
  }

  public fun getOrThrow(): T = when (this) {
    is Success -> value
    is Failure -> when (error) {
      is Throwable -> throw error
      else -> throw IllegalStateException("Tried is failure: $error")
    }
  }

  public fun getOrElse(defaultValue: @UnsafeVariance T): T = when (this) {
    is Success -> value
    is Failure -> defaultValue
  }

  public inline fun getOrElse(defaultValue: (E) -> @UnsafeVariance T): T = when (this) {
    is Success -> value
    is Failure -> defaultValue(error)
  }

  public fun getOrNull(): T? = when (this) {
    is Success -> value
    is Failure -> null
  }

  public fun errorOrNull(): E? = when (this) {
    is Success -> null
    is Failure -> error
  }

  public inline fun onSuccess(action: (T) -> Unit): Tried<T, E> {
    if (this is Success) {
      action(value)
    }

    return this
  }

  public inline fun onFailure(action: (E) -> Unit): Tried<T, E> {
    if (this is Failure) {
      action(error)
    }

    return this
  }

  public fun isSuccess(): Boolean = this is Success

  public fun isFailure(): Boolean = this is Failure

  public inline fun <R> fold(
    onSuccess: (T) -> R,
    onFailure: (E) -> R,
  ): R = when (this) {
    is Success -> onSuccess(value)
    is Failure -> onFailure(error)
  }
}

public fun <T> triedSuccess(value: T): Tried<T, Nothing> = Tried.success(value)

public fun <E> triedFailure(error: E): Tried<Nothing, E> = Tried.failure(error)

public fun <T, E> T?.toTried(errorIfNull: () -> E): Tried<T, E> =
  if (this != null) {
    Tried.Success(this)
  } else {
    Tried.Failure(errorIfNull())
  }
