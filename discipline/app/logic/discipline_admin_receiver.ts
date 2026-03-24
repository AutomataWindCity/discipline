import { DeviceAdminReceiver, Context, Intent, Log, Toast } from "./android.ts"
import { DisciplineMonitoringService } from "./discipline_monitoring_service.ts"

@NativeClass()
@JavaProxy("automata-wind-city.auto-nytro.discipline.DisciplineDeviceAdminReceiver")
export class DisciplineDeviceAdminReceiver extends DeviceAdminReceiver {
  private static readonly TAG = "DisciplineDeviceAdminReceiver";

  override onProfileProvisioningComplete(context: Context, intent: Intent): void {
    super.onProfileProvisioningComplete(context, intent);

    Log.d(DisciplineDeviceAdminReceiver.TAG, "Device admin provisioning complete")

    // TODO: initialize the daemon
  }

  override onEnabled(context: Context, intent: Intent): void {
    super.onEnabled(context, intent);
    
    Log.d(DisciplineDeviceAdminReceiver.TAG, "Device admin enabled");

    Toast.makeText(context, "Device admin enabled", Toast.LENGTH_SHORT);

    this.startServices(context);

    // todo: open daemon
  }

  override onDisabled(context: Context, intent: Intent): void {
    super.onDisabled(context, intent);
    
    Log.d(DisciplineDeviceAdminReceiver.TAG, "Device admin disabled");

    Toast.makeText(context, "Devive admin disabled", Toast.LENGTH_SHORT);

    this.stopServices(context);
  }

  private startServices(context: Context) {
    const intent = new Intent(context, DisciplineMonitoringService.class);
    context.startService(intent);
  }

  private stopServices(context: Context) {
    const intent = new Intent(context, DisciplineMonitoringService.class);
    context.stopService(intent);
  }

  public override onUserStarted(param0: android.content.Context, param1: android.content.Intent, param2: android.os.UserHandle): void {
    
  }

  public override onUserRemoved(param0: android.content.Context, param1: android.content.Intent, param2: android.os.UserHandle): void {
    
  }

  public override onUserStopped(param0: android.content.Context, param1: android.content.Intent, param2: android.os.UserHandle): void {
    
  }

  public override onUserSwitched(param0: android.content.Context, param1: android.content.Intent, param2: android.os.UserHandle): void {
    
  }

  
  // public getManager(param0: android.content.Context): android.app.admin.DevicePolicyManager {
    
  // }
}