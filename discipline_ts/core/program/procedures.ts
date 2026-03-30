import { 
  Conditional, Countdown, CountdownConditional, 
  Duration, TimeRange, TimeRangeConditional, 
  UptimeAllowanceConditional, Vault, VaultData, 
  VaultName, Program,
  TextualError,
} from "../x.ts";

const fiveMinutes = Duration.fromMillisecondsOrThrow(1000 * 60 * 5);

export class ConditionalLocation {
  static readonly LUNY_SCREEN = "luny-screen";
  static readonly LUNY_INTERNET = "luny-internet";
  static readonly RURU_SCREEN = "ruru-screen";
  static readonly RURU_INTERNET = "ruru-internet";

  private constructor(
    private readonly value: (
      | typeof ConditionalLocation.LUNY_SCREEN
      | typeof ConditionalLocation.LUNY_INTERNET
      | typeof ConditionalLocation.RURU_SCREEN
      | typeof ConditionalLocation.RURU_INTERNET
    )
  ) {}

  static fromString(string: string) {
    switch (string) {
      case "luny-internet":
      case "ruru-screen":
      case "ruru-internet":
      case "luny-screen": {
        return new ConditionalLocation(string);
      }
      default: {
        return new Error(`Creating ConditionalLocation from string: Expected string to be either "luny-screen", "luny-internet", "ruru-screen", or "ruru-internet" but found "${string}"`)
      }
    }
  }

  match<A, B, C, D>(cases: {
    LunyScreen: () => A,
    LunyInternet: () => B,
    RuruScreen: () => C,
    RuruInternet: () => D,
  }): A | B | C | D {
    switch (this.value) {
      case "luny-screen": return cases.LunyScreen();
      case "luny-internet": return cases.LunyInternet();
      case "ruru-screen": return cases.RuruScreen();
      case "ruru-internet": return cases.RuruInternet();
    }
  }
}

export const enum StatusType {
  ExternalError,
  InternalError,
  Success,
}

export class StatusExternalError {
  readonly type = StatusType.ExternalError;

  constructor(readonly message: TextualError) {}
}

export class StatusInternalError {
  readonly type = StatusType.InternalError;

}

export class StatusSuccess {
  readonly type = StatusType.Success;

}

const addConditional = (
  program: Program,
  location: ConditionalLocation, 
  conditional: Conditional,
) => {
  location.match({
    LunyScreen() {
      program.luny.screen.push(conditional);
    },
    LunyInternet() {
      program.luny.internet.push(conditional);
    },
    RuruScreen() {
      program.ruru.screen.push(conditional);
    },
    RuruInternet() {
      program.ruru.internet.push(conditional);
    },
  })
};

export const createTimeRangeConditional = async (
  program: Program,
  timeRange: TimeRange,
  lifetime: Duration,
  location: ConditionalLocation,
) => {
  if (program.conditionalNumber >= program.maximumConditionalNumber) {
    return new StatusExternalError(
      TextualError
        .create("Creating 'TimeRangeConditional'")
        .addMessage("Reached maximum number of conditionals allowed. Wait for some of them to be deleted before attempting to create new ones")
        .addNumberAttachment("Maximum conditionals number", program.maximumConditionalNumber)
    );
  }

  const lifetimeAsCountdown = Countdown.create(
    program.monotonicClock.getNow(),
    lifetime,
  );

  const conditional = TimeRangeConditional.create(
    timeRange,
    lifetimeAsCountdown,
  );

  addConditional(program, location, conditional);
  
  const error = await program.saveData1();
  if (TextualError.is(error)) {
    console.trace(`Failed to save program data 1: ${error.print({ color: true })}`);
    return new StatusInternalError();
  }

  return new StatusSuccess();
};

export const createCountdownConditional = async (
  program: Program,
  duration: Duration,
  location: ConditionalLocation,
) => {
  if (program.conditionalNumber >= program.maximumConditionalNumber) {
    return new StatusExternalError(
      TextualError
        .create("Creating 'CountdownConditional'")
        .addMessage("Reached maximum number of conditionals allowed. Wait for some of them to be deleted before attempting to create new ones")
        .addNumberAttachment("Maximum conditionals number", program.maximumConditionalNumber)
    );
  }

  const conditional = CountdownConditional.create(
    Countdown.create(
      program.monotonicClock.getNow(),
      duration,
    )
  );

  addConditional(program, location, conditional);

  const error = await program.saveData1();
  if (TextualError.is(error)) {
    console.trace(`Failed to save program data 1: ${error.print()}`);
    return new StatusInternalError();
  }

  return new StatusSuccess();
}

// todo: check max conditionals number violation
// todo: check durations
export const createUptimeAllowanceConditional = async (
  program: Program,
  allowance: Duration,
  lifetime: Duration,
  location: ConditionalLocation,
) => {
  if (program.conditionalNumber >= program.maximumConditionalNumber) {
    return new StatusExternalError(
      TextualError
        .create("Creating 'UptimeAllowanceConditional'")
        .addMessage("Reached maximum number of conditionals allowed. Wait for some of them to be deleted before attempting to create new ones")
        .addNumberAttachment("Maximum conditionals number", program.maximumConditionalNumber)
    );
  }

  const conditional = UptimeAllowanceConditional.create(
    allowance,
    Countdown.create(
      program.monotonicClock.getNow(),
      lifetime,
    ),
  );
  if (TextualError.is(conditional)) {
    return new StatusExternalError(conditional);
  }

  addConditional(program, location, conditional);

  const error = await program.saveData1();
  if (TextualError.is(error)) {
    console.trace(`Failed to save program data 1: ${error.print()}`);
    return new StatusInternalError();
  }

  return new StatusSuccess();
};

export const createVault = async (
  program: Program,
  name: VaultName,
  data: VaultData,
  protectionDuration: Duration,
) => {
  if (program.vaultNumber >= program.maximumVaultNumber) {
    return new StatusExternalError(
      TextualError
        .create("Creating 'Vault'")
        .addMessage("Reached maximum number of vaults allowed. Delete some of them before attempting to create new ones")
        .addNumberAttachment("Maximum vaults number", program.maximumVaultNumber)
    );
  }

  for (const vault of program.vaults) {
    if (vault.getName().isEqualTo(name)) {
      return new StatusExternalError(
        TextualError
          .create("Creating 'Vault'")
          .addMessage("Provided vault name is already used by another vault")
          .addStringAttachment("Provided vault name", name.toString())
      );
    }
  }

  const protectionCountdown = Countdown.create(
    program.monotonicClock.getNow(),
    protectionDuration,
  );

  const vault = Vault.create(
    name,
    data,
    protectionCountdown,
  );

  program.vaults.push(vault);

  const error = await program.saveData1();
  if (TextualError.is(error)) {
    console.trace(`Failed to save program data 1: ${error.print()}`);
    return new StatusInternalError();
  }

  return new StatusSuccess();
};

export const deleteVault = async (
  program: Program,
  name: VaultName,
) => {
  const vaultIndex = program.vaults.findIndex(it => it.getName().isEqualTo(name));
  if (vaultIndex === -1) {
    return new StatusExternalError(
      TextualError
        .create("Deleting 'Vault'")
        .addMessage("There is no vault with the given name")
        .addStringAttachment("Given name", name.toString())
    );
  }

  const vault = program.vaults[vaultIndex];
  const now = program.monotonicClock.getNow();
  if (vault.isProtected(now)) {
    return new StatusExternalError(
      TextualError
        .create("Deleting 'Vault'")
        .addMessage("'Vault' is protected")
        .addStringAttachment("Remaining protection duration", vault.getProtection().getRemainingTimeOrZero(now).toString2())
    );
  }

  program.vaults.splice(vaultIndex, 1);

  const error = await program.saveData1();
  if (TextualError.is(error)) {
    console.trace(`Failed to save program data 1: ${error.print()}`);
    return new StatusInternalError();
  }

  return new StatusSuccess();
};

export const prolongVaultProtection = async (
  program: Program,
  name: VaultName,
) => {
  const vault = program.vaults.find(it => it.getName().isEqualTo(name));
  if (vault === undefined) {
    return new StatusExternalError(
      TextualError
        .create("Prolonging 'Vault' protection")
        .addMessage("There is no 'Vault' with the given name")
        .addStringAttachment("Given name", name.toString())
    );
  }

  const now = program.monotonicClock.getNow();
  vault.extendProtectionByOrSetToMaxSafeValue(now, fiveMinutes);

  const error = await program.saveData1();
  if (TextualError.is(error)) {
    console.trace(`Failed to save program data 1: ${error.print()}`);
    return new StatusInternalError();
  }

  return new StatusSuccess();
};