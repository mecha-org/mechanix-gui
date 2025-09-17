import 'package:dbus/dbus.dart';
import 'package:logger/web.dart';
import 'package:mechanix_settings/src/features/about/models/types.dart';

class DBusAboutService {
  final logger = Logger();
  static const String _busName = 'org.freedesktop.hostname1';
  static const String _objectPath = '/org/freedesktop/hostname1';

  Future<AboutDetailsType> getDeviceDetails() async {
    final client = DBusClient.system();
    String hostnameValue = '';
    String hardwareModelValue = '';
    String hardwareSerialValue = '';
    String firmwareVersionValue = '';
    String kernelReleaseValue = '';
    String operatingSystemNameValue = '';

    try {
      final object = DBusRemoteObject(
        client,
        name: _busName,
        path: DBusObjectPath(_objectPath),
      );

      final hostname = await object.getProperty(
        _busName,
        'Hostname',
      );

      if (hostname is DBusString) {
        hostnameValue = hostname.value;
      }

      final hardwareModel = await object.getProperty(
        _busName,
        'HardwareModel',
      );

      if (hardwareModel is DBusString) {
        hardwareModelValue = hardwareModel.value;
      }

      final firmwareVersion = await object.getProperty(
        _busName,
        'FirmwareVersion',
      );

      if (firmwareVersion is DBusString) {
        firmwareVersionValue = firmwareVersion.value;
      }

      final kernelRelease = await object.getProperty(
        _busName,
        'KernelRelease',
      );

      if (kernelRelease is DBusString) {
        kernelReleaseValue = kernelRelease.value;
      }

      final operatingSystemName = await object.getProperty(
        _busName,
        'OperatingSystemPrettyName',
      );

      if (operatingSystemName is DBusString) {
        operatingSystemNameValue = operatingSystemName.value;
      }

      final AboutDetailsType about = AboutDetailsType(
        firmwareVersion: firmwareVersionValue,
        hardwareModel: hardwareModelValue,
        hardwareSerial: hardwareSerialValue,
        hostname: hostnameValue,
        kernelRelease: kernelReleaseValue,
        operatingSystemName: operatingSystemNameValue,
      );
      return about;
    } catch (e) {
      logger.e('error while getting details $e');

      final AboutDetailsType about = AboutDetailsType(
        firmwareVersion: firmwareVersionValue,
        hardwareModel: hardwareModelValue,
        hardwareSerial: hardwareSerialValue,
        hostname: hostnameValue,
        kernelRelease: kernelReleaseValue,
        operatingSystemName: operatingSystemNameValue,
      );
      return about;
    } finally {
      client.close();
    }
  }
}
