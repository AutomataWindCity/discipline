package com.example.app

import androidx.room.Entity

/**
 * Statistics about vaults
 */
@Entity
public data class VaultsStats private constructor(
  var vaultsNumber: Int,
  var maximumVaultsNumber: Int
) {
  companion object {
    fun create(maximumVaultsNumber: Int): VaultsStats {
      return VaultsStats(0, maximumVaultsNumber)
    }

    fun construct(vaultsNumber: Int, maximumVaultsNumber: Int): Tried<VaultsStats, TextualError> {
      if (vaultsNumber > maximumVaultsNumber) {
        return Tried.failure(
          TextualError.create("Constructing VaultsStats")
            .addMessage("Argument 'vaultsNumber' is greater than argument 'maximumVaultsNumber'")
            .addIntAttachment("Vaults number", vaultsNumber)
            .addIntAttachment("Maximum vaults number", maximumVaultsNumber)
        )
      }

      return Tried.success(VaultsStats(vaultsNumber, maximumVaultsNumber))
    }
  }
  
  fun isFull(): Boolean {
    return vaultsNumber >= maximumVaultsNumber
  }
  
  fun hasSpaceForOneMore(): Boolean {
    return vaultsNumber < maximumVaultsNumber
  }

  fun hasSpaceForNMore(count: Int): Boolean {
    return vaultsNumber + count <= maximumVaultsNumber
  }
  
  fun remainingSpace(): Int {
    return maximumVaultsNumber - vaultsNumber
  }
  
  // TODO: 
  fun updateAfterVaultAdded() {
    vaultsNumber += 1
  }
  
  // TODO:
  fun updateAfterVaultDeleted() {
    vaultsNumber = (vaultsNumber - 1).coerceAtLeast(0)
  }
  
  override fun toString(): String {
    return "VaultsStats($vaultsNumber/$maximumVaultsNumber)"
  }
}
