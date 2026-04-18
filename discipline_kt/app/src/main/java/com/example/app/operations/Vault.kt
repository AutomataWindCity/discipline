// package com.example.app.procedures.vault

// import com.example.app.*
// import com.example.app.database.*

// sealed class CreateReturn {
//   class TooManyVaults() : CreateReturn() {}
//   class DuplicateVaultId() : CreateReturn() {}
//   class InternalError(val error: Throwable) : CreateReturn() {}
//   class Success(val id: Vault, val vault: Vault) : CreateReturn() {}
// }

// fun create(
//   database: DatabaseConnection,
//   adapter: VaultDbAdapter,
//   location: VaultLocation,
//   stats: VaultsStats,
//   optionalVaultId: Vault?,
//   vaultName: VaultName,
//   vaultData: VaultData,
//   vaultProtectorCreator: VaultProtector.Creator,
// ): CreateReturn {
//   if (stats.isFull()) {
//     return CreateReturn.TooManyVaults()
//   }

//   let vaultId = optionalRuleId ?: Vault.generateOrThrow()

//   // TODO: Use the adapter to check whether there is already a value with this id. 

//   val vault = Vault.create(
//     vaultName,
//     vaultData,
//     vaultProtectorCreator.create(),
//   )

//   try {
//     adapter.createOrThrow(database, location, vaultId, vault)
//   } catch (exception: Throwable) {
//     return CreateReturn.InternalError(exception)
//   }

//   stats.vaultsNumber += 1
//   return CreateReturn.Success(vaultId, vault)
// }

// sealed class DeleteReturn {
//   class NoSuchVault() : DeleteReturn() {}
//   class PermissionDenied() : DeleteReturn() {}
//   class InternalError(val error: Throwable) : DeleteReturn() {}
//   class Success() : DeleteReturn() {}
// }

// fun delete(
//   database: DatabaseConnection,
//   adapter: VaultDbAdapter,
//   location: VaultLocation,
//   stats: VaultsStats,
//   vaultId: Vault,
//   clock: MonotonicClock,
// ): DeleteReturn {
//   val protector = try {
//     adapter.getVaultProtectorOrThrow(database, location, vaultId)
//   } catch (exception: Throwable) {
//     return DeleteReturn.InternalError(exception)
//   }

//   val now = clock.getNow()
//   if (protector.isActive(now)) {
//     return DeleteReturn.PermissionDenied()
//   }

//   try {
//     adapter.deleteVaultOrThrow(database, location, vaultId)
//   } catch (exception: Throwable) {
//     return DeleteReturn.InternalError(exception)
//   }

//   stats.vaultsNumber -= 1
//   return DeleteReturn.Success()
// }