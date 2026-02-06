import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:collection/collection.dart';
import 'package:dbus/dbus.dart';
import 'package:flutter/foundation.dart';
import 'package:logger/web.dart';
import 'package:mechanix_settings/src/features/network/models/access_points.dart';
import 'package:mechanix_settings/src/features/network/models/types.dart';
import 'package:nm/nm.dart';

import '../models/saved_networks.dart';
import 'wifi_repository.dart';

class WifiRepositoryImpl implements WifiRepository {
  bool _connected = false;
  final logger = Logger();
  late NetworkManagerClient _client;

  @override
  Stream<bool> get wirelessEnabledStream => _wirelessEnabledController.stream;
  final _wirelessEnabledController = StreamController<bool>.broadcast();

  @override
  Future<void> init() async {
    try {
      _client = NetworkManagerClient();
      await _client.connect();
      _connected = true;

      _client.propertiesChanged.listen((props) {
        if (props.contains('WirelessEnabled')) {
          _wirelessEnabledController.add(_client.wirelessEnabled);
        }
      });
      // _client.activeConnectionAdded.listen((connection) {
      //   logger.w('Active connection added: ${connection.id}');
      // });
      // _client.activeConnectionRemoved.listen((connection) {
      //   logger.w('Active connection removed: ${connection.id}');
      // });
      logger.i('NetworkManagerClient connected ${_client.wirelessEnabled}');
    } catch (e) {
      logger.e('Failed to connect to NetworkManagerClient: $e');
    }
  }

  @override
  Future<bool> isWirelessEnabled() async {
    if (_connected) {
      logger.i('Wireless Enabled --- check client: ${_client.wirelessEnabled}');
      return _client.wirelessEnabled;
    } else {
      logger.i(
          'FIRST TIME CLIENT Wireless Enabled --- check client: ${_client.wirelessEnabled}');
      var client = NetworkManagerClient();
      await client.connect();
      _connected = true;
      return client.wirelessEnabled;
    }
  }

  @override
  Future<NetworkManagerDevice> getWifiDevice() async {
    final devices = _client.devices;
    NetworkManagerDevice wifiDevice = devices.firstWhere(
      (d) => d.deviceType == NetworkManagerDeviceType.wifi,
    );

    return wifiDevice;
  }

// check: wiredDevice.managed prop - to check if wired network is connected or not
// check: wiredDevice.setManaged method - to set wired network is enabled true/false
  @override
  Future<NetworkManagerDevice> getWiredDevice() async {
    final devices = _client.devices;
    NetworkManagerDevice wiredDevice = devices.firstWhere(
      (d) => d.deviceType == NetworkManagerDeviceType.ethernet,
    );
    return wiredDevice;
  }

  @override
  Future<void> setWifiEnabled(bool enable) async {
    try {
      await _client.setWirelessEnabled(enable);
    } catch (e) {
      print('Failed to set Wireless Enabled: $e');
    }
  }

  @override
  Future<Stream<List<String>>> streamWifiEvents() async {
    logger.i('Subscribing to WiFi events prop ${_client.propertiesChanged}');
    return _client.propertiesChanged;
  }

  @override
  Future<Stream<List<String>>> streamWirelessDeviceStream() async {
    final NetworkManagerDevice device = await getWifiDevice();
    return device.wireless!.propertiesChanged;
  }

  @override
  Future<List<NetworkManagerActiveConnection>> activatingConnection() async {
    return _client.activeConnections;
  }

  @override
  Future<({AccessPoints? active, List<AccessPoints> available})>
      availableAccessPoints(
          List<SavedWirelessNetwork>? allSavedNetworks) async {
    try {
      NetworkManagerDevice? wifiDevice = await getWifiDevice();

      if (wifiDevice == null ||
          wifiDevice.state == NetworkManagerDeviceState.unavailable) {
        logger.e('No WiFi device found');
        return (active: null, available: <AccessPoints>[]);
      }

      await wifiDevice.wireless!.requestScan();
      List<AccessPoints> accessPoints = [];
      final seenSsids = <String>{}; // to track unique SSIDs
      var activeAccessPoint = wifiDevice.wireless?.activeAccessPoint;
      var nmAccessPoints = wifiDevice.wireless?.accessPoints;
      AccessPoints? connectedAccessPoint;

      var ip4Config = wifiDevice.ip4Config;
      var ip6Config = wifiDevice.ip6Config;

      for (var nmAccessPoint in nmAccessPoints!) {
        final ssid =
            utf8.decode(nmAccessPoint.ssid); // Convert List<int> to String

        if (ssid.isNotEmpty && !seenSsids.contains(ssid)) {
          seenSsids.add(ssid); // mark this SSID as seen
          var isActive =
              listEquals(activeAccessPoint?.ssid, nmAccessPoint.ssid) &&
                  wifiDevice.state == NetworkManagerDeviceState.activated;
          var isSaved = allSavedNetworks?.any((sn) => sn.ssid == ssid) ?? false;
          var isSecure = nmAccessPoint.wpaFlags.isNotEmpty ||
              nmAccessPoint.rsnFlags.isNotEmpty;

          if (isActive) {
            // connected
            connectedAccessPoint = AccessPoints(
              isActive: isActive,
              isSaved: isSaved,
              isSecure: isSecure,
              nmAccessPoint: nmAccessPoint,
              ip4Config: ip4Config,
              ip6Config: ip6Config,
            );
          } else {
            // active + saved
            var accessPoint = AccessPoints(
              nmAccessPoint: nmAccessPoint,
              isActive: isActive,
              isSaved: isSaved,
              isSecure: isSecure,
              ip4Config: ip4Config,
              ip6Config: ip6Config,
            );
            accessPoints.add(accessPoint);
          }
        }
      }
      return (active: connectedAccessPoint, available: accessPoints);
    } catch (e) {
      logger.e('Error in availableAccessPoints: $e');
      return (active: null, available: <AccessPoints>[]);
    }
  }

  @override
  Future<void> connectToHiddenNetwork(String ssid, String password) async {
    logger.i('Connecting to hidden network: $ssid');

    // Check if connection already exists
    final existingConnection =
        await _findAndRemoveExistingConnection(accessPoint: null, ssid: ssid);
    if (existingConnection != null) {
      logger.i('Found existing connection for hidden network, removing it');
      await existingConnection.delete();
    }

    if (password.isEmpty) {
      throw Exception('Password is required for hidden network');
    }

    final connection = <String, Map<String, DBusValue>>{
      'connection': <String, DBusValue>{
        'id': DBusString(ssid),
        'type': DBusString('802-11-wireless'),
        'autoconnect': DBusBoolean(true),
      },
      '802-11-wireless': <String, DBusValue>{
        'ssid': DBusArray(
          DBusSignature.byte,
          utf8.encode(ssid).map((b) => DBusByte(b)).toList(),
        ),
        'mode': DBusString('infrastructure'),
        'hidden': DBusBoolean(true),
      },
      '802-11-wireless-security': <String, DBusValue>{
        'key-mgmt': DBusString('wpa-psk'),
        'psk': DBusString(password),
      },
      'ipv4': <String, DBusValue>{
        'method': DBusString('auto'),
      },
      'ipv6': <String, DBusValue>{
        'method': DBusString('ignore'),
      },
    };

    // Create a new connection
    logger.i("init connect to hidden network");

    NetworkManagerDevice device = await getWifiDevice();

    try {
      // Has password
      String? psk;
      if (password.isEmpty) {
        psk ??= stdin.readLineSync(encoding: utf8);
      }
      logger.i('ssid: $ssid, password: $password, psk: $psk');

      final result = await _client.addAndActivateConnection(
        device: device,
        connection: connection,
      );
      logger.i('Connected to hidden network $ssid successfully : $result');
    } catch (e) {
      logger.e('Failed to connect to network: $e');
    }
  }

  @override
  Future<void> connectToNetwork(
      NetworkManagerAccessPoint accessPoint, String password) async {
    print("init connect to network");

    NetworkManagerDevice wifiDevice = await getWifiDevice();
    if (wifiDevice == null ||
        wifiDevice.state == NetworkManagerDeviceState.unavailable) {
      print('connectToNetwork::No WiFi device found');
      throw Exception('No WiFi device available');
    }

    try {
      // Check if connection already exists to avoid duplicates, if exist remove it
      final existingConnection = await _findAndRemoveExistingConnection(
          accessPoint: accessPoint, ssid: null);

      if (existingConnection != null) {
        print('Found existing connection, activating it');
        existingConnection.delete();
      }

      // Network requires password (WPA/WPA2)
      if (accessPoint.rsnFlags.isNotEmpty) {
        await _connectSecureNetwork(wifiDevice, accessPoint, password);
      } else {
        // Open network (no password)
        await _connectOpenNetwork(wifiDevice, accessPoint);
      }
    } catch (e) {
      print('Failed to connect to network: $e');
      rethrow; // Properly propagate the error
    }
  }

  @override
  Future<void> deleteSavedNetwork(NetworkManagerAccessPoint accessPoint) async {
    final existingConnection = await _findAndRemoveExistingConnection(
        accessPoint: accessPoint, ssid: null);

    if (existingConnection != null) {
      logger.i('Found existing connection, activating it');
      existingConnection.delete();
    }
  }

  /// Connect to a password-protected network
  Future<void> _connectSecureNetwork(NetworkManagerDevice wifiDevice,
      NetworkManagerAccessPoint accessPoint, String password) async {
    String? psk;
    print('Connecting to secure network: $accessPoint with $password');

    if (password.isEmpty) {
      psk = await getSavedWifiPsk(wifiDevice, accessPoint);
      psk ??= stdin.readLineSync(encoding: utf8);

      if (psk == null || psk.isEmpty) {
        throw Exception('Password is required for this network');
      }
    } else {
      psk = password;
    }

    print('Connecting to secure network with password $accessPoint with $psk ');
    try {
      final result = await _client.addAndActivateConnection(
        device: wifiDevice,
        accessPoint: accessPoint,
        connection: {
          '802-11-wireless-security': {
            'key-mgmt': DBusString('wpa-psk'),
            'psk': DBusString(psk),
          }
        },
      );

      print('Connected to secure network: $result');
    } catch (e) {
      print('Failed to connect to secure network: $e');
      rethrow;
    }
  }

  /// Connect to an open network (no password)
  Future<void> _connectOpenNetwork(NetworkManagerDevice wifiDevice,
      NetworkManagerAccessPoint accessPoint) async {
    print('Connecting to open network');

    try {
      final result = await _client.addAndActivateConnection(
        device: wifiDevice,
        accessPoint: accessPoint,
      );
      print('Connected to open network: $result');
    } catch (e) {
      print('Failed to connect to open network: $e');
      rethrow;
    }
  }

  /// Find existing connection for this access point to avoid duplicates
  Future<NetworkManagerSettingsConnection?> _findAndRemoveExistingConnection(
      {NetworkManagerAccessPoint? accessPoint, String? ssid}) async {
    try {
      final connections = _client.settings.connections;

      for (var connection in connections) {
        final settings = await connection.getSettings();
        final wirelessSettings = settings['802-11-wireless'];

        if (wirelessSettings != null) {
          final ssid = wirelessSettings['ssid'];
          if (ssid != null &&
              ssid is DBusArray &&
              ssid.children != null &&
              String.fromCharCodes(
                      ssid.children!.map((e) => (e as DBusByte).value)) ==
                  (accessPoint?.ssid != null
                      ? String.fromCharCodes(accessPoint!.ssid)
                      : String.fromCharCodes(
                          ssid.children!.map((e) => (e as DBusByte).value)))) {
            return connection;
          }
        }
      }
    } catch (e) {
      logger.w('Error finding existing connection: $e');
      // Continue with adding new connection if lookup fails
    }

    return null;
  }

  Future<void> disconnectNetwork(String ssid) async {
    NetworkManagerDevice device = await getWifiDevice();

    var connection = device.activeConnection;
    try {
      _client.deactivateConnection(connection!);
      logger.i('Connection $ssid deactivated successfully');
    } catch (e) {
      logger.e('wifi deactivation failed: $e');
      return;
    }
  }

  // forget network - remove profile settings

  @override
  Future<void> forgetNetwork(String ssid) async {
    logger.i('Deleting saved network: $ssid');

    final connections = _client.settings.connections;

    for (var cn in connections) {
      if (!cn.unsaved) {
        var connectionSettings = await cn.getSettings();
        final connectionId =
            connectionSettings["connection"]?["id"]?.toNative();
        if (connectionId == ssid) {
          try {
            await cn.delete();
            logger.i('Connection $ssid deleted successfully');
          } catch (e) {
            logger.e('Failed to delete saved network: $e');
          }
          return;
        }
      }
    }
  }

  @override
  Future<NetworkManagerState> getWifiState() async {
    return _client.state;
  }

  @override
  Future<StreamAndDevice> getWifiStateAndReason() async {
    var wifiDevice = await getWifiDevice();
    return StreamAndDevice(wifiDevice.propertiesChanged, wifiDevice);
  }

  Future<void> connectedNetwork() async {
    var primaryConnection = _client.primaryConnection;
    if (primaryConnection != null && primaryConnection.ip4Config != null) {
      var ip4Config = primaryConnection.ip4Config!;

      var mainAddress = ip4Config.addressData.first;
      print('Main IP Address: ${mainAddress['address']}');
      print('Subnet Mask: /${mainAddress['prefix']}');
      print('Gateway: ${ip4Config.gateway}');
    }

    await _client.close();
  }

  @override
  Future<List<SavedNetworks>> savedNetworks(
      List<AccessPoints>? availableAccessPoints) async {
    final List<SavedNetworks> savedNetworks = [];
    final connections = _client.settings.connections;
    final seenBssids = <String>{}; // to track unique SSIDs
    for (var cn in connections) {
      if (!cn.unsaved) {
        var connectionSettings = await cn.getSettings();

        final connectionId =
            connectionSettings["connection"]?["id"]?.toNative();
        final String bssid =
            connectionSettings["802-11-wireless"]?["seen-bssids"]?.toString() ??
                '';

        if (!seenBssids.contains(bssid)) {
          seenBssids.add(bssid); // mark this BSSID as seen
          AccessPoints? accessPoint =
              availableAccessPoints?.firstWhereOrNull((ap) {
            return utf8.decode(ap.nmAccessPoint.ssid) == connectionId;
          });

          SavedNetworks savedNetwork = SavedNetworks(
            ssid: connectionId,
            icon: '',
            connected: accessPoint?.isActive ?? false,
            accessPoint: accessPoint?.nmAccessPoint,
          );

          savedNetworks.add(savedNetwork);
        }
      }
    }
    return savedNetworks;
  }

  @override
  Future<List<SavedWirelessNetwork>> getSavedNetworks() async {
    try {
      final connections = _client.settings.connections;

      final List<SavedWirelessNetwork> savedNetworks = [];

      for (var cn in connections) {
        if (!cn.unsaved) {
          var connectionSettings = await cn.getSettings();

          final flatSettings = flattenConnectionSettings(connectionSettings);
          final String bssid = connectionSettings["802-11-wireless"]
                      ?["seen-bssids"]
                  ?.toString() ??
              '';
          final seenBssids = <String>{};

          if (!seenBssids.contains(bssid)) {
            seenBssids.add(bssid); // mark this BSSID as seen

            final macAddress =
                flatSettings["802-11-wireless.seen-bssids"].toString() == 'null'
                    ? ''
                    : flatSettings["802-11-wireless.seen-bssids"].toString();

            SavedWirelessNetwork savedNetwork = SavedWirelessNetwork(
              ssid: flatSettings["connection.id"],
              macAddress: macAddress.replaceAll(RegExp(r'[\[\]]'), ''),
              security: flatSettings["802-11-wireless-security.key-mgmt"],
              ipv4Method: flatSettings["ipv4.method"],
              autoConnect: flatSettings["connection.autoconnect"],
            );
            savedNetworks.add(savedNetwork);
          }
        }
      }
      return savedNetworks;
    } catch (e) {
      logger.e('Error in getSavedNetworks: $e');
      return [];
    }
  }

  Map<String, dynamic> flattenConnectionSettings(
      Map<String, Map<String, DBusValue>> rawSettings) {
    final flat = <String, dynamic>{};

    rawSettings.forEach((section, entries) {
      entries.forEach((key, dbusValue) {
        final nativeValue = dbusValueToDart(dbusValue, keyName: key);
        flat['$section.$key'] = nativeValue;
      });
    });

    return flat;
  }

  dynamic dbusValueToDart(DBusValue value, {String? keyName}) {
    // 1️⃣ Primitive numeric/string/bool
    if (value is DBusString ||
        value is DBusInt32 ||
        value is DBusUint32 ||
        value is DBusInt64 ||
        value is DBusUint64 ||
        value is DBusDouble ||
        value is DBusBoolean) {
      return value.toNative();
    }

    // 2️⃣ Byte array case — especially for SSID
    if (value is DBusArray &&
        value.signature == DBusSignature('y') && // array of bytes
        (keyName == 'ssid' ||
            keyName?.toLowerCase().contains('ssid') == true)) {
      final bytes =
          value.children.whereType<DBusByte>().map((b) => b.value).toList();
      try {
        return utf8.decode(bytes);
      } catch (_) {
        return bytes; // fallback to numeric list if not decodable
      }
    }

    // 3️⃣ Generic array
    if (value is DBusArray) {
      return value.children.map((v) => dbusValueToDart(v)).toList();
    }

    // 4️⃣ Dict/map
    if (value is DBusDict) {
      return value.children
          .map((k, v) => MapEntry(dbusValueToDart(k), dbusValueToDart(v)));
    }

    // 5️⃣ Struct
    if (value is DBusStruct) {
      return value.children.map(dbusValueToDart).toList();
    }

    // 6️⃣ Variant wrapper
    if (value is DBusVariant) {
      return dbusValueToDart(value.value, keyName: keyName);
    }

    // 7️⃣ Single DBusByte
    if (value is DBusByte) {
      return value.value;
    }

    // 8️⃣ Fallback
    return value.toString();
  }

  @override
  Future<void> connectToSavedNetwork(
      NetworkManagerAccessPoint accessPoint) async {
    var accessPointSsid = utf8.decode(accessPoint.ssid);
    print("Connecting to saved network: $accessPointSsid");

    // Find the WiFi device
    NetworkManagerDevice device = await getWifiDevice();
    var connection = _client.settings.connections;
    for (var cn in connection) {
      if (!cn.unsaved) {
        var connectionSettings = await cn.getSettings();
        final connectionId =
            connectionSettings["connection"]?["id"]?.toNative();
        if (connectionId == accessPointSsid) {
          try {
            // Activate the saved connection
            await _client.activateConnection(
                device: device, connection: cn, accessPoint: accessPoint);
            print('Connection $accessPointSsid activated successfully');
          } catch (e) {
            print('Failed to connect to saved network: $e');
            Future.error('Failed to connect to saved network: $e');
          }
          return;
        }
      }
    }
  }

  @override
  Future<NetworkManagerDeviceState?> getNetworkState() async {
    logger.i('getActivateNetworks state - ${_client.activeConnections.length}');

    final wifiDevices = _client.devices.where(
      (d) => d.deviceType == NetworkManagerDeviceType.wifi,
    );
    final device = wifiDevices.isNotEmpty ? wifiDevices.first : null;

    logger.i('getActivateNetworks devices.length - ${device?.state}');

    if (device != null) {
      return device.state;
    }
    return null;
  }

  @override
  Future<void> close() async {
    await _client.close();
    await _wirelessEnabledController.close();
    _connected = false;
  }
}

Future<NetworkManagerSettingsConnection?> getAccessPointConnectionSettings(
    NetworkManagerDevice device, NetworkManagerAccessPoint accessPoint) async {
  var ssid = utf8.decode(accessPoint.ssid);

  var settings = await Future.wait(device.availableConnections
      .map((e) async => {'settings': await e.getSettings(), 'connection': e}));
  NetworkManagerSettingsConnection? accessPointSettings;
  for (var element in settings) {
    var s = element['settings'] as dynamic;
    if (s != null) {
      var connection = s['connection'] as Map<String, DBusValue>?;
      if (connection != null) {
        var id = connection['id'];
        if (id != null) {
          if (id.toNative() == ssid) {
            accessPointSettings =
                element['connection'] as NetworkManagerSettingsConnection;
            break;
          }
        }
      }
    }
  }
  return accessPointSettings;
}

Future<String?> getSavedWifiPsk(
    NetworkManagerDevice device, NetworkManagerAccessPoint accessPoint) async {
  var settingsConnection =
      await getAccessPointConnectionSettings(device, accessPoint);
  if (settingsConnection != null) {
    var secrets =
        await settingsConnection.getSecrets('802-11-wireless-security');
    if (secrets.isNotEmpty) {
      var security = secrets['802-11-wireless-security'];
      if (security != null) {
        var psk = security['psk'];
        if (psk != null) {
          return psk.toNative();
        }
      }
    }
  }
  return null;
}
