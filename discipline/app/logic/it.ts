import { 
  DateTime, Duration, TextualError, 
  Tried, App, Instant, MonotonicClock,
} from "../../../discipline_android_primitive_ts/x.ts"

import { 
  AccessibilityEvent, AccessibilityService, 
  AccessibilityServiceInfo, 
  Build, ComponentName, Context, 
  DeviceAdminReceiver, DevicePolicyManager, 
  Handler, IBinder, Intent, Log, Looper, 
  Notification, NotificationChannel, 
  NotificationCompat, NotificationManager, 
  PackageManager, Runnable, 
  Service, Settings, 
  TimeUnit, 
  Toast, UsageEvent, UsageEvents, 
  UsageStatsManager, 
  UserManager,
} from "./imports.ts"

const logError = (tag: string, string: string) => {

};


@NativeClass()
class DisciplineRunnable extends Runnable {
  private action: () => void;

  private constructor(action: () => void) {
    super();
    this.action = action;
    return global.__native(this);
  }

  static create(action: () => void) {
    return new DisciplineRunnable(action);
  }

  override run(): void {
    this.action();
  }
}


type JavaNullable<T> = T;

const JavaNullable = <T>(value: T | null): JavaNullable<Exclude<T, null>> => {
  return value as any as JavaNullable<Exclude<T, null>>;
};
