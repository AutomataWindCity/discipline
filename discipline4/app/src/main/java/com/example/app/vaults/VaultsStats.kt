package com.yourpackage.discipline

/**
 * Statistics about vaults
 */
data class VaultsStats private constructor(
    val vaultsNumber: Int,
    val maximumVaultsNumber: Int
) {
    companion object {
        fun create(maximumVaultsNumber: Int): VaultsStats = VaultsStats(0, maximumVaultsNumber)
        fun construct(vaultsNumber: Int, maximumVaultsNumber: Int): VaultsStats = 
            VaultsStats(vaultsNumber, maximumVaultsNumber)
    }
    
    fun isFull(): Boolean = vaultsNumber >= maximumVaultsNumber
    
    fun hasSpace(): Boolean = vaultsNumber < maximumVaultsNumber
    
    fun remainingCapacity(): Int = maximumVaultsNumber - vaultsNumber
    
    fun withVaultAdded(): VaultsStats = copy(vaultsNumber = vaultsNumber + 1)
    
    fun withVaultRemoved(): VaultsStats = copy(vaultsNumber = (vaultsNumber - 1).coerceAtLeast(0))
    
    fun canAddVaults(count: Int): Boolean = vaultsNumber + count <= maximumVaultsNumber
    
    override fun toString(): String = "VaultsStats($vaultsNumber/$maximumVaultsNumber)"
}