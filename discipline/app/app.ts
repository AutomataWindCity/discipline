/*
In NativeScript, the app.ts file is the entry point to your application.
You can use this file to perform app-level initialization, but the primary
purpose of the file is to pass control to the app’s first module.
*/

import { Application } from '@nativescript/core'

@NativeClass()
@JavaProxy("com.discipline.DeviceAdminReceiver")
class DeviceAdminReceiver extends android.app.admin.DeviceAdminReceiver {
  override onEnabled(context: android.content.Context, intent: android.content.Intent): void {
    super.onEnabled(context, intent);
    android.widget.Toast.makeText(context, "Device Admin Enabled", android.widget.Toast.LENGTH_SHORT);
  }

  override onDisabled(context: android.content.Context, intent: android.content.Intent): void {
    super.onDisabled(context, intent);
    android.widget.Toast.makeText(context, "Device Admin Disabled", android.widget.Toast.LENGTH_SHORT);
  }
}

Application.run({ moduleName: 'app-root' })

/*
Do not place any code after the application has been started as it will not
be executed on iOS.
*/
