import 'dart:convert';
import 'dart:developer';
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
  final NetworkManagerClient _client = NetworkManagerClient();
  bool _connected = false;
  final logger = Logger();

  WifiRepositoryImpl() {
    _init();
  }

  Future<void> _init() async {
    if (!_connected) {
      await _client.connect();
      _connected = true;
    }
  }

  Future<void> _ensureConnected() async {
    if (!_connected) {
      await _client.connect();
      _connected = true;
    }
  }

  @override
  Future<bool> isWirelessEnabled() async {
    log('Fetching WiFi status');
    var client = NetworkManagerClient();
    await client.connect();
    var result = _client.wirelessEnabled;
    return result;
  }

  @override
  Future<bool> setWifiEnabled(bool enable) async {
    await _ensureConnected();
    await _client.setWirelessEnabled(enable);
    return enable;
  }

  @override
  Future<Stream<List<String>>> streamWifiEvents() async {
    logger.i('Subscribing to WiFi events');
    var client = NetworkManagerClient();
    await client.connect();
    logger.i('Subscribing to WiFi events prop ${client.propertiesChanged}');
    return client.propertiesChanged;
  }

  @override
  Future<({AccessPoints? active, List<AccessPoints> available})>
      availableAccessPoints(List<SavedNetworks>? savedNetworks) async {
    logger.i('Fetching available access points');
    var client = NetworkManagerClient();
    await client.connect();
    var devices = client.devices;
    if (devices.isEmpty) {
      logger.w('No devices found');
      return (active: null, available: <AccessPoints>[]);
    }
    var wifiDevice = devices
        .where((device) => device.deviceType == NetworkManagerDeviceType.wifi);
    var device = wifiDevice.isNotEmpty ? wifiDevice.first : null;

    if (device == null) {
      logger.w('No WiFi device found');
      return (active: null, available: <AccessPoints>[]);
    }
    await device.wireless!.requestScan();
    List<AccessPoints> accessPoints = [];
    final seenSsids = <String>{}; // to track unique SSIDs
    var activeAccessPoint = device.wireless?.activeAccessPoint;
    var nmAccessPoints = device.wireless?.accessPoints;
    AccessPoints? connectedAccessPoint;

    var ip4Config = device.ip4Config;
    var ip6Config = device.ip6Config;

    for (var nmAccessPoint in nmAccessPoints!) {
      final ssid =
          utf8.decode(nmAccessPoint.ssid); // Convert List<int> to String
      if (ssid.isNotEmpty && !seenSsids.contains(ssid)) {
        seenSsids.add(ssid); // mark this SSID as seen
        var isActive = listEquals(activeAccessPoint?.ssid, nmAccessPoint.ssid);
        var isSaved = savedNetworks?.any((sn) => sn.ssid == ssid) ?? false;
        if (isActive) {
          connectedAccessPoint = AccessPoints(
            isActive: isActive,
            isSaved: isSaved,
            nmAccessPoint: nmAccessPoint,
            ip4Config: ip4Config,
            ip6Config: ip6Config,
          );
        } else {
          var accessPoint = AccessPoints(
            nmAccessPoint: nmAccessPoint,
            isActive: isActive,
            isSaved: isSaved,
            ip4Config: ip4Config,
            ip6Config: ip6Config,
          );
          accessPoints.add(accessPoint);
        }
      }
    }
    return (active: connectedAccessPoint, available: accessPoints);
  }

  @override
  Future<void> connectToUnknownNetwork(String ssid, String password) async {
    // // Define the connection settings
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
    logger.i("init connect to unknown network");
    var client = NetworkManagerClient();
    await client.connect();
    NetworkManagerDevice device;
    try {
      device = client.devices
          .firstWhere((d) => d.deviceType == NetworkManagerDeviceType.wifi);
    } catch (e) {
      logger.e('No WiFi devices found');
      return;
    }

    try {
      // Has password
      String? psk;
      if (password.isEmpty) {
        psk ??= stdin.readLineSync(encoding: utf8);
      }
      logger.i('password: $password, psk: $psk');

      await client.addAndActivateConnection(
        device: device,
        connection: connection,
      );
      logger.i('Connected to hidden network $ssid successfully');
    } catch (e) {
      logger.e('Failed to connect to network: $e');
    }
  }

  @override
  Future<void> connectToNetwork(
      NetworkManagerAccessPoint accessPoint, String password) async {
    logger.i("init connect to network");
    var client = NetworkManagerClient();
    await client.connect();
    NetworkManagerDevice device;
    try {
      device = client.devices
          .firstWhere((d) => d.deviceType == NetworkManagerDeviceType.wifi);
    } catch (e) {
      logger.e('No WiFi devices found');
      return;
    }
    try {
      // Has password
      if (accessPoint.rsnFlags.isNotEmpty) {
        String? psk;
        if (password.isEmpty) {
          var psk = await getSavedWifiPsk(device, accessPoint);
          psk ??= stdin.readLineSync(encoding: utf8);
        }
        logger.i('IF password: $password, psk: $psk');
        await client.addAndActivateConnection(
            device: device,
            accessPoint: accessPoint,
            connection: {
              '802-11-wireless-security': {
                'key-mgmt': DBusString('wpa-psk'),
                'psk': DBusString(psk ?? password),
              }
            }).then((res) {
          logger.i('IF Connected to network: $res');
        }).catchError((e) {
          logger.e('IF Failed to connect to network: $e');
          Future.error('IF Failed to connect to network: $e');
        });
      } else {
        await client
            .addAndActivateConnection(device: device, accessPoint: accessPoint)
            .then((res) {
          logger.i('Connected to network: $res');
        }).catchError((e) {
          logger.e('Failed to connect to network: $e');
          Future.error('Failed to connect to network: $e');
        });
      }
    } catch (e) {
      logger.e('Failed to connect to network: $e');
    }
  }

  // disconnect network , keep profile settings
  Future<void> disconnectNetwork(String ssid) async {
    var client = NetworkManagerClient();
    await client.connect();
    NetworkManagerDevice device;
    try {
      device = client.devices
          .firstWhere((d) => d.deviceType == NetworkManagerDeviceType.wifi);
    } catch (e) {
      logger.e('No WiFi devices found');
      return;
    }

    var connection = device.activeConnection;
    try {
      client.deactivateConnection(connection!);
      logger.i('Connection $ssid deactivated successfully');
    } catch (e) {
      logger.e('wifi deactivation failed: $e');
      return;
    }
  }

  // delete network - remove profile settings
  @override
  Future<void> forgetNetwork(String ssid) async {
    var client = NetworkManagerClient();
    await client.connect();

    try {
      var connections = client.settings.connections;
      for (var connection in connections) {
        var settings = await connection.getSettings();
        var wifiSettings = settings['802-11-wireless'];

        if (wifiSettings != null && wifiSettings['ssid'] != null) {
          final ssidArray = wifiSettings['ssid'] as DBusArray;
          final ssidBytes =
              ssidArray.children.map((e) => (e as DBusByte).value).toList();
          final wifiSsid = utf8.decode(ssidBytes);
          if (wifiSsid == ssid) {
            await connection.delete();
            return;
          }
        }
      }
    } catch (e) {
      logger.e('Failed to forget network: $e');
      return;
    }
  }

  @override
  Future<NetworkManagerState> getWifiState() async {
    var client = NetworkManagerClient();
    await client.connect();
    return client.state;
  }

  @override
  Future<StreamAndDevice> getWifiStateAndReason() async {
    var client = NetworkManagerClient();
    await client.connect();

    var wifiDevice = client.devices.firstWhere(
      (d) => d.deviceType == NetworkManagerDeviceType.wifi,
    );

    // // var stateAndReason = wifiDevice.stateReason;
    // wifiDevice.propertiesChanged.listen((event) {
    //   if (event.contains('StateReason')) {
    //     logger.i('DEVICE STATE CHANGE: ${wifiDevice.stateReason.state} ||  ${wifiDevice.stateReason.reason}}');

    //   }
    // });

    return StreamAndDevice(wifiDevice.propertiesChanged, wifiDevice);
  }

  Future<void> connectedNetwork() async {
    var client = NetworkManagerClient();
    await client.connect();

    var primaryConnection = client.primaryConnection;
    if (primaryConnection != null && primaryConnection.ip4Config != null) {
      var ip4Config = primaryConnection.ip4Config!;

      var mainAddress = ip4Config.addressData.first;
      print('Main IP Address: ${mainAddress['address']}');
      print('Subnet Mask: /${mainAddress['prefix']}');
      print('Gateway: ${ip4Config.gateway}');
    }

    await client.close();
  }

  @override
  Future<List<SavedNetworks>> savedNetworks(
      List<AccessPoints>? availableAccessPoints) async {
    logger.i('Loading saved networks');
    var client = NetworkManagerClient();
    await client.connect();
    final List<SavedNetworks> savedNetworks = [];
    final connections = client.settings.connections;
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
    logger.i('Fetching saved networks');
    var client = NetworkManagerClient();
    await client.connect();
    final connections = client.settings.connections;
    final seenBssids = <String>{}; // to track unique SSIDs

    final List<SavedWirelessNetwork> savedNetworks = [];

    for (var cn in connections) {
      if (!cn.unsaved) {
        var connectionSettings = await cn.getSettings();
        final connectionId =
            connectionSettings["connection"]?["id"]?.toNative();

        final securityFlagValue = connectionSettings["802-11-wireless-security"]
                ?["key-mgmt"]
            ?.toString();
        final securityFlags =
            (securityFlagValue == "wpa-psk") ? "WPA-PSK" : "Open";

        final String bssid =
            connectionSettings["802-11-wireless"]?["seen-bssids"]?.toString() ??
                '';

        final String passphrase =
            connectionSettings["802-11-wireless-security"]?["psk"]?.toString() ??
                '';

        logger.i('connectionId: $connectionId | Security: $securityFlags | Passphrase: $passphrase');

        if (!seenBssids.contains(bssid)) {
          seenBssids.add(bssid); // mark this BSSID as seen
          SavedWirelessNetwork savedNetwork = SavedWirelessNetwork(
            ssid: connectionId,
            security: securityFlags,
          );
          savedNetworks.add(savedNetwork);
        }
      }
    }
    return savedNetworks;
  }

  @override
  Future<void> deleteSavedNetwork(String ssid) async {
    logger.i('Deleting saved network: $ssid');
    var client = NetworkManagerClient();
    await client.connect();
    final connections = client.settings.connections;

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
  Future<void> disconnectFromNetwork(String ssid) {
    // TODO: implement disconnectFromNetwork
    throw UnimplementedError();
  }

  @override
  Future<void> connectToSavedNetwork(
      NetworkManagerAccessPoint accessPoint) async {
    var accessPointSsid = utf8.decode(accessPoint.ssid);
    logger.i("Connecting to saved network: $accessPointSsid");
    var client = NetworkManagerClient();
    await client.connect();

    // Find the WiFi device
    NetworkManagerDevice device;
    try {
      device = client.devices
          .firstWhere((d) => d.deviceType == NetworkManagerDeviceType.wifi);
    } catch (e) {
      logger.e('No WiFi devices found');
      return;
    }
    var connection = client.settings.connections;
    for (var cn in connection) {
      if (!cn.unsaved) {
        var connectionSettings = await cn.getSettings();
        final connectionId =
            connectionSettings["connection"]?["id"]?.toNative();
        if (connectionId == accessPointSsid) {
          try {
            // Activate the saved connection
            await client.activateConnection(
                device: device, connection: cn, accessPoint: accessPoint);
            logger.i('Connection $accessPointSsid activated successfully');
          } catch (e) {
            logger.e('Failed to connect to saved network: $e');
            Future.error('Failed to connect to saved network: $e');
          }
          return;
        }
      }
    }
  }

  @override
  Future<Stream<List<String>>> streamWirelessDeviceStream() async {
    var client = NetworkManagerClient();
    await client.connect();
    final NetworkManagerDevice device = client.devices.firstWhere(
        (d) => d.deviceType == NetworkManagerDeviceType.wifi,
        orElse: () => throw Exception('No WiFi device found'));

    return device.wireless!.propertiesChanged;
  }

  @override
  Future<NetworkManagerDeviceState?> getNetworkState() async {
    var client = NetworkManagerClient();
    await client.connect();

    logger.i('getActivateNetworks state - ${client.activeConnections.length}');

    final wifiDevices = client.devices.where(
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
