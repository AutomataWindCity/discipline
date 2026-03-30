import { Branded, Database, DateTime, State, TextualError, Tried } from "./x.ts";

const BRAND = Symbol();

export type App = Branded<typeof BRAND, {
  state: State,
  database: Database,
}>;

export const open = (): Tried<App, TextualError> => {
  // TODO
  throw new Error("Not implemented")
};

export const onAndroidProfileProvisioningComplete = (it: App) => {};
export const onAndroidAdminEnabled = (it: App) => {};
export const onAndroidAdminDisabled = (it: App) => {};
export const onAndroidDeviceStartupEvent = (it: App, when: DateTime) => {};
export const onAndroidDeviceShutdownEvent = (it: App, when: DateTime) => {};
export const onAndroidKeyguardShownEvent = (it: App, when: DateTime) => {};
export const onAndroidKeyguardHiddenEvent = (it: App, when: DateTime) => {};
export const onAndroidActivityResumedEvent = (it: App, packageName: string, when: DateTime) => {};
export const onAndroidActivityPausedEvent = (it: App, packageName: string, when: DateTime) => {};
export const onAndroidActivityStoppedEvent = (it: App, packageName: string, when: DateTime) => {};
export const onAndroidUserStarted = (it: App) => {};
export const onAndroidUserStopped = (it: App) => {};
export const onAndroidUserRemoved = (it: App) => {};
export const onAndroidUserSwitched = (it: App) => {};
export const onAndroidServiceCreated = (it: App) => {};
export const onAndroidServiceStartCommand = (it: App) => {};
export const onAndroidServiceDestroy = (it: App) => {};

export const App = {
  open,
  onAndroidProfileProvisioningComplete,
  onAndroidAdminEnabled,
  onAndroidAdminDisabled,
  onAndroidDeviceStartupEvent,
  onAndroidDeviceShutdownEvent,
  onAndroidKeyguardShownEvent,
  onAndroidKeyguardHiddenEvent,
  onAndroidActivityResumedEvent,
  onAndroidActivityPausedEvent,
  onAndroidActivityStoppedEvent,
  onAndroidUserStarted,
  onAndroidUserStopped,
  onAndroidUserRemoved,
  onAndroidUserSwitched,
  onAndroidServiceCreated,
  onAndroidServiceStartCommand,
  onAndroidServiceDestroy,

};