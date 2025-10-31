import 'package:dbus/dbus.dart';
import 'package:logger/web.dart';

Future<Map<String, dynamic>> connectToMxconf() async {
  final logger = Logger();

  final result = <String, dynamic>{};

  final client = DBusClient.session();

  final object = DBusRemoteObject(
    client,
    name: 'org.mechanix.MxConf',
    path: DBusObjectPath('/org/mechanix/MxConf'),
  );

  try {
    final filesHomePageSettings = await object.callMethod(
      'org.mechanix.MxConf', // Valid interface name
      'GetSetting', // Method name
      [const DBusString('org.mechanix.files.files_home_page.*')], // Parameters
    );
    result['filesHomePage'] = filesHomePageSettings;

    final generalSettings = await object.callMethod(
      'org.mechanix.MxConf',
      'GetSetting',
      [const DBusString('org.mechanix.files.general.*')],
    );
    result['generalSettings'] = generalSettings;
  } catch (e) {
    logger.e('Error connecting to mxconf: $e');
  } finally {
    await client.close();
  }
  return result;
}
