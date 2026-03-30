import { DeviceAdminReceiver, Context, Intent, UserHandle, Log } from "./android.ts"
import { WorkerApi } from "./backgraound_thread/worker_api.ts";
import { TextualError } from "./discipline.ts";
import { DisciplineService } from "./discipline_service.ts"

const LOG_TAG = "DisciplineDeviceAdminReceiver";

@NativeClass()
@JavaProxy("awc.autonytro.discipline.DeviceAdminReceiver")
export class DisciplineDeviceAdminReceiver extends DeviceAdminReceiver {
  override onProfileProvisioningComplete(context: Context, intent: Intent): void {
    super.onProfileProvisioningComplete(context, intent);

    const error = WorkerApi.staticOnDeviceAdminEnabled()
    if (error !== null) {
      TextualError.changeContext(error, "Calling 'onProfileProvisioningComplete' on 'DisciplineDeviceAdminReceiver'");
      Log.e(LOG_TAG, TextualError.prettyPrint(error));
      return;
    }
  }

  override onEnabled(context: Context, intent: Intent): void {
    super.onEnabled(context, intent);

    this.startServices(context);

    const error = WorkerApi.staticOnDeviceAdminEnabled();
    if (error !== null) {
      TextualError.changeContext(error, "Calling 'onEnabled' on 'DisciplineDeviceAdminReceiver'");
      Log.e(LOG_TAG, TextualError.prettyPrint(error));
      return;
    }
  }

  override onDisabled(context: Context, intent: Intent): void {
    super.onDisabled(context, intent);

    this.stopServices(context);

    const error = WorkerApi.staticOnDeviceAdminDisabled();
    if (error !== null) {
      TextualError.changeContext(error, "Calling 'onDisabled' on 'DisciplineDeviceAdminReceiver'");
      Log.e(LOG_TAG, TextualError.prettyPrint(error));
      return;
    }
  }

  private startServices(context: Context) {
    const intent = new Intent(context, DisciplineService.class);
    context.startService(intent);
  }

  private stopServices(context: Context) {
    const intent = new Intent(context, DisciplineService.class);
    context.stopService(intent);
  }

  public override onUserStarted(context: Context, intent: Intent, user: UserHandle): void {
    super.onUserStarted(context, intent, user);

    const error = WorkerApi.staticOnUserStarted(user);
    if (error !== null) {
      TextualError.changeContext(error, "Calling 'onUserStarted' on 'DisciplineDeviceAdminReceiver'");
      Log.e(LOG_TAG, TextualError.prettyPrint(error));
      return;
    }
  }

  public override onUserRemoved(context: Context, intent: Intent, user: UserHandle): void {
    super.onUserRemoved(context, intent, user);

    const error = WorkerApi.staticOnUserRemoved(user);
    if (error !== null) {
      TextualError.changeContext(error, "Calling 'onUserRemoved' on 'DisciplineDeviceAdminReceiver'");
      Log.e(LOG_TAG, TextualError.prettyPrint(error));
      return;
    }
  }

  public override onUserStopped(context: Context, intent: Intent, user: UserHandle): void {
    super.onUserStopped(context, intent, user);

    const error = WorkerApi.staticOnUserStopped(user);
    if (error !== null) {
      TextualError.changeContext(error, "Calling 'onUserStopped' on 'DisciplineDeviceAdminReceiver'");
      Log.e(LOG_TAG, TextualError.prettyPrint(error));
      return;
    }
  }

  public override onUserSwitched(context: Context, intent: Intent, user: UserHandle): void {
    super.onUserSwitched(context, intent, user);

    const error = WorkerApi.staticOnUserSwitched(user);
    if (error !== null) {
      TextualError.changeContext(error, "Calling 'onUserSwitched' on 'DisciplineDeviceAdminReceiver'");
      Log.e(LOG_TAG, TextualError.prettyPrint(error));
      return;
    }
  }
}
