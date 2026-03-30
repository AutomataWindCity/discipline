import { _ } from "../../encode/index.ts";
import { Branded, Duration, Tried, ProgramT, State, Countdown, Program, UptimeAllowanceConditional, VaultName, VaultData, Integer, Vault, DateTime } from "../x.ts";

const HTTP_OK_CODE = 200;
const HTTP_OK_TEXT = "OK";

const HTTP_NOT_FOUND_CODE = 404;
const HTTP_NOT_FOUND_TEXT = "Not Found";

const HTTP_BAD_REQUEST_CODE = 400;
const HTTP_BAD_REQUEST_TEXT = "Bad Request";

const HTTP_INTERNAL_SERVER_ERROR_CODE = 500;
const HTTP_INTERNAL_SERVER_ERROR_TEXT = "Internal Server Error";

const createOkResponse = (message: string) => {
  return new Response(message, {
    status: HTTP_OK_CODE,
    statusText: HTTP_OK_TEXT,
  });
};

const createNotFoundResponse = (message: string) => {
  return new Response(message, {
    status: HTTP_NOT_FOUND_CODE,
    statusText: HTTP_NOT_FOUND_TEXT,
  });
};

const createBadRequestResponse = (message: string) => {
  return new Response(message, {
    status: HTTP_BAD_REQUEST_CODE,
    statusText: HTTP_BAD_REQUEST_TEXT,
  });
};

const createInternalServerErrorResponse = (message: string) => {
  return new Response(message, {
    status: HTTP_INTERNAL_SERVER_ERROR_CODE,
    statusText: HTTP_INTERNAL_SERVER_ERROR_TEXT,
  });
};

const FIVE_MINUTES = Duration.fromMillisecondsOrThrow(Duration.MILLISECONDS_PER_MINUTE * 5);
const HOUR = Duration.fromMillisecondsOrThrow(Duration.MILLISECONDS_PER_HOUR);
const WEEK = Duration.fromMillisecondsOrThrow(Duration.MILLISECONDS_PER_WEEK);

const denyDeviceAccessCountdownIncrement = (program: ProgramT) => {
  const totalRemainingDuration = Duration.plusOrMax(
    Countdown.getRemainingDuration(program.state.denyDeviceAccessCountdown),
    FIVE_MINUTES,
  );

  if (Duration.isGreaterThan(totalRemainingDuration, State.DENY_DEVICE_ACCESS_COUNTDOWN_MAXIMUM_DURATION)) {
    return createBadRequestResponse(`Failure: Incremetning would block the device for than the maximum safe duration`);
  }

  Countdown.setRemaniningDuration(
    program.state.denyDeviceAccessCountdown,
    FIVE_MINUTES,
  );

  const saveResult = Program.saveState(program);
  if (Tried.isFailure(saveResult)) {
    return createInternalServerErrorResponse("Failure: Failed to save updates to the database");
  }

  return createOkResponse("Success: Deny Device Access Countdown incremented by five minutes");
};

const denyDeviceAccessUptimeAllowanceDecrement = (program: ProgramT) => {
  const originalRemainingDuration = UptimeAllowanceConditional.getRemainingDuration(
    program.state.denyDeviceAccessUptimeAllowance
  );

  const updatedRemainingDuration = Duration.minusOrZero(
    originalRemainingDuration,
    FIVE_MINUTES,
  );

  UptimeAllowanceConditional.setRemainingDuration(
    program.state.denyDeviceAccessUptimeAllowance,
    updatedRemainingDuration,
  );

  let decrement: Duration.DurationT;
  if (Duration.isGreaterThanOrEqualTo(originalRemainingDuration, FIVE_MINUTES)) {
    decrement = FIVE_MINUTES;
  } else {
    decrement = Duration.minusOrZero(FIVE_MINUTES, originalRemainingDuration);
  }

  const saveResult = Program.saveState(program);
  if (Tried.isFailure(saveResult)) {
    return createInternalServerErrorResponse("Failure: Failed to save updates to the database");
  }

  const decrementAsString = Duration.toString(decrement);
  const updatedRemainingDurationAsString = Duration.toString(updatedRemainingDuration);

  return createOkResponse(`Success: Decremented Deny Device Access Uptime Allowance by ${decrementAsString} and it's now ${updatedRemainingDurationAsString}`)
};

const vaultsCreate = (program: ProgramT, params: URLSearchParams) => {
  const vaultNameParam = params.get("vault_name");
  const vaultDataParam = params.get("vault_data");
  const vaultProtectionDurationParam = params.get("vault_protection_duration");

  if (vaultNameParam === null) {
    return createBadRequestResponse(`Failure: "vault_name" search param was not set`)
  }
  if (vaultDataParam === null) {
    return createBadRequestResponse(`Failure: "vault_data" search param was not set`)
  }
  if (vaultProtectionDurationParam === null) {
    return createBadRequestResponse(`Failure: "vault_protection_duration" search param was not set`)
  }

  const vaultNameOrError = VaultName.create(vaultNameParam);
  if (Tried.isFailure(vaultNameOrError)) {
    return createBadRequestResponse(`Failure: "vault_name" param length is invalid. Minimum length: ${VaultName.minimumLength}. Maximum length: ${VaultName.maximumLength}. Param value: ${vaultNameParam}. Param value length: ${vaultNameParam.length}`)
  }

  const vaultDataOrError = VaultData.create(vaultDataParam);
  if (Tried.isFailure(vaultDataOrError)) {
    return createBadRequestResponse(`Failure: "vault_data" param length is invalid. Minimum length: ${VaultName.minimumLength}. Maximum length: ${VaultName.maximumLength}. Param value: ${vaultNameParam}. Param value length: ${vaultNameParam.length}`)
  }

  const vaultProtectionDurationInMinutesOrError = Integer.fromString(vaultProtectionDurationParam);
  if (vaultProtectionDurationInMinutesOrError === null) {
    return createBadRequestResponse(`Failure: "vault_protection_duration" param is invalid: It could not be parsed as number. Param value: ${vaultProtectionDurationParam}`);
  }

  const vaultProtectionDurationOrError = Duration.fromMilliseconds(vaultProtectionDurationInMinutesOrError * Duration.MILLISECONDS_PER_MINUTE);
  if (
    Tried.isFailure(vaultProtectionDurationOrError) 
    || 
    Duration.isGreaterThan(Tried.value(vaultProtectionDurationOrError), WEEK)
  ) {
    return createBadRequestResponse(`Failure: "vault_protection_duration" param, which is a number of minutes, is too large. The maximum allowed value is ${Duration.toTotaMinutes(WEEK)} minutes (a single week). Param value: ${vaultProtectionDurationInMinutesOrError}`);
  }

  const now = DateTime.now();
  const newVault = Vault.create(
    Tried.value(vaultNameOrError),
    Tried.value(vaultDataOrError),
    Tried.value(vaultProtectionDurationOrError),
    now,
  );

  if (program.state.vaults.some(existingVault => VaultName.isEqualTo(
    existingVault.name,
    newVault.name,
  ))) {
    return createBadRequestResponse(`Failure: "vault_name" param specifies the name of an existing vault. Choose another name. Param value: ${VaultName.toString(newVault.name)}`);
  }

  program.state.vaults.push(newVault);
  
  const saveResult = Program.saveState(program);
  if (Tried.isFailure(saveResult)) {
    return createInternalServerErrorResponse("Failure: Failed to save updates to the database");
  }

  return createOkResponse(`Success: Added a new vault named ${VaultName.toString(newVault.name)}`)
};

const vaultsDelete = (program: ProgramT, params: URLSearchParams) => {
  const vaultNameParam = params.get("vault_name");
  if (vaultNameParam === null) {
    return createBadRequestResponse(`Failure: "vault_name" param is not set`);
  }

  const vaultNameOrError = VaultName.create(vaultNameParam);
  if (Tried.isFailure(vaultNameOrError)) {
    return createBadRequestResponse(`Failure: "vault_name" param length is invalid. Minimum length: ${VaultName.minimumLength}. Maximum length: ${VaultName.maximumLength}. Param value: ${vaultNameParam}. Param value length: ${vaultNameParam.length}`)
  }

  const vaultName = Tried.value(vaultNameOrError);
  const vaultIndex = program.state.vaults.findIndex(vault => VaultName.isEqualTo(
    vault.name,
    vaultName,
  ));

  if (vaultIndex < 0) {
    return createBadRequestResponse(`Failure: There is no vault named ${VaultName.toString(vaultName)}`);
  }

  program.state.vaults.splice(vaultIndex, 1);

  const saveResult = Program.saveState(program);
  if (Tried.isFailure(saveResult)) {
    return createInternalServerErrorResponse("Failure: Failed to save updates to the database");
  }

  return createOkResponse(`Success: Deleted a vault named ${VaultName.toString(vaultName)}`)
};

const vaultProtectionIncrement = (program: ProgramT, params: URLSearchParams) => {
  const vaultNameParam = params.get("vault_name");
  if (vaultNameParam === null) {
    return createBadRequestResponse(`Failure: "vault_name" param is not set`);
  }

  const vaultNameOrError = VaultName.create(vaultNameParam);
  if (Tried.isFailure(vaultNameOrError)) {
    return createBadRequestResponse(`Failure: "vault_name" param length is invalid. Minimum length: ${VaultName.minimumLength}. Maximum length: ${VaultName.maximumLength}. Param value: ${vaultNameParam}. Param value length: ${vaultNameParam.length}`)
  }

  const vaultName = Tried.value(vaultNameOrError);
  const vault = program.state.vaults.find(vault => VaultName.isEqualTo(
    vault.name,
    vaultName,
  ));

  if (vault === undefined) {
    return createBadRequestResponse(`Failure: There is no vault named ${VaultName.toString(vaultName)}`);
  }

  const durationTillMax = Duration.minusOrZero(
    WEEK,
    vault.protection.remainingDuration,
  );

  const increment = Duration.min(
    durationTillMax,
    HOUR,
  );

  vault.protection.remainingDuration = Duration.plusOrMax(
    vault.protection.remainingDuration,
    increment,
  );

  const saveResult = Program.saveState(program);
  if (Tried.isFailure(saveResult)) {
    return createInternalServerErrorResponse("Failure: Failed to save updates to the database");
  }

  return createOkResponse(`Success: Incremented the protection duration of a vault named ${VaultName.toString(vaultName)} by ${Duration.toString(increment)}`)
};

const home = (program: ProgramT) => {
  return `
  
  `;
};

const DENY_DEVICE_ACCESS_COUNTDOWN_INCREMENT_URL_PATH = "/deny_device_access_countdown/increment";
const DENY_DEVICE_ACCESS_UPTIME_ALLOWANCE_URL_PATH = "/deny_device_access_uptime_allowance/decrement";
const VAULT_CREATE_URL_PATH = "/vault/create";
const VAULT_DELETE_URL_PATH = "/vault/delete";
const VAULT_PROTECTION_INCREMENT_URL_PATH = "/vault/protection/increment";

const process = (
  program: ProgramT,
  request: Request,
): Response => {
  let url: URL;
  try {
    url = new URL(request.url);
  } catch (error) {
    console.error(error);
    return createBadRequestResponse("Malformed Url");
  }

  const pathname = url.pathname;

  if (pathname === DENY_DEVICE_ACCESS_COUNTDOWN_INCREMENT_URL_PATH) {
    return denyDeviceAccessCountdownIncrement(program);
  }
  if (pathname === DENY_DEVICE_ACCESS_UPTIME_ALLOWANCE_URL_PATH) {
    return denyDeviceAccessUptimeAllowanceDecrement(program);
  }
  if (pathname === VAULT_CREATE_URL_PATH) {
    return vaultsCreate(program, url.searchParams);
  }
  if (pathname === VAULT_DELETE_URL_PATH) {
    return vaultsDelete(program, url.searchParams);
  }
  if (pathname === VAULT_PROTECTION_INCREMENT_URL_PATH) {
    return vaultProtectionIncrement(program, url.searchParams);
  }

  return createNotFoundResponse(`No such path: ${pathname}`)
};

const BRAND = Symbol();

export type ServerT = Branded<typeof BRAND, {
  readonly server: Deno.HttpServer,
}>;

const construct = (server: Deno.HttpServer): ServerT => {
  return Branded(BRAND, {
    server,
  });
};

export const start = (port: number, program: ProgramT): Tried.Tried<ServerT, unknown> => {
  let server;

  try {
    server = Deno.serve(
      { 
        port, 
        hostname: "127.0.0.1",
      }, 
      request => {
        return process(program, request);
      }
    );
  } catch (error) {
    return Tried.Failure(error);
  }

  return Tried.Success(construct(server));
};