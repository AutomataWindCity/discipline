package org.autonytro.discipline

interface Platform {
    val name: String
}

expect fun getPlatform(): Platform