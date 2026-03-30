export const Settings = android.provider.Settings;
export type Settings = android.provider.Settings;

export const AccessibilityEvent = android.view.accessibility.AccessibilityEvent;
export type AccessibilityEvent = android.view.accessibility.AccessibilityEvent;

export const AccessibilityService = android.accessibilityservice.AccessibilityService;
export type AccessibilityService = android.accessibilityservice.AccessibilityService;

export const AccessibilityServiceInfo = android.accessibilityservice.AccessibilityServiceInfo;
export type AccessibilityServiceInfo = android.accessibilityservice.AccessibilityServiceInfo;

export const DeviceAdminReceiver = android.app.admin.DeviceAdminReceiver;
export type DeviceAdminReceiver = android.app.admin.DeviceAdminReceiver;

export const DevicePolicyManager = android.app.admin.DevicePolicyManager;
export type DevicePolicyManager = android.app.admin.DevicePolicyManager;

export const DeviceAdminService = android.app.admin.DeviceAdminService
export type DeviceAdminService = android.app.admin.DeviceAdminService

export const NotificationManager = android.app.NotificationManager;
export type NotificationManager = android.app.NotificationManager;

export const Notification = android.app.Notification;
export type Notification = android.app.Notification;

export const NotificationChannel = android.app.NotificationChannel;
export type NotificationChannel = android.app.NotificationChannel;

export const ActivityManager = android.app.ActivityManager;
export type ActivityManager = android.app.ActivityManager;

export const Service = android.app.Service;
export type Service = android.app.Service;

export const UsageStatsManager = android.app.usage.UsageStatsManager;
export type UsageStatsManager = android.app.usage.UsageStatsManager;

export const UsageEvents = android.app.usage.UsageEvents;
export type UsageEvents = android.app.usage.UsageEvents;

export const UsageEvent = UsageEvents.Event;
export type UsageEvent = InstanceType<(typeof UsageEvents)["Event"]>;

export const UsageStats = android.app.usage.UsageStats;
export type UsageStats = android.app.usage.UsageStats;

export const Context = android.content.Context;
export type Context = android.content.Context;

export const Intent = android.content.Intent;
export type Intent = android.content.Intent;

export const ComponentName = android.content.ComponentName;
export type ComponentName = android.content.ComponentName;

export const PackageManager = android.content.pm.PackageManager;
export type PackageManager = android.content.pm.PackageManager;

export const Log = android.util.Log;
export type Log = android.util.Log;

export const Toast = android.widget.Toast;
export type Toast = android.widget.Toast;

export const UserManager = android.os.UserManager;
export type UserManager = android.os.UserManager;

export const Build = android.os.Build;
export type Build = android.os.Build;

export const Handler = android.os.Handler;
export type Handler = android.os.Handler;

export const HandlerThread = android.os.HandlerThread;
export type HandlerThread = android.os.HandlerThread;

export const UserHandle = android.os.UserHandle;
export type UserHandle = android.os.UserHandle;

export const IBinder = android.os.IBinder;
export type IBinder = android.os.IBinder;

export const Looper = android.os.Looper;
export type Looper = android.os.Looper;


// AndroidX exports
export const NotificationCompat = androidx.core.app.NotificationCompat;
export type NotificationCompat = androidx.core.app.NotificationCompat;
