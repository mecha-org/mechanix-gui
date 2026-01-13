import 'package:mechanix_settings/src/features/network/models/access_points.dart';
import 'package:mechanix_settings/src/features/network/models/saved_networks.dart';
import 'package:mechanix_settings/src/features/network/models/types.dart';
import 'package:nm/nm.dart';

abstract class WifiRepository {
  Future<void> init(); // connect client
  Future<bool> isWirelessEnabled();
  Stream<bool> get wirelessEnabledStream;
  Future<NetworkManagerState> getWifiState();
  Future<StreamAndDevice> getWifiStateAndReason();
  Future<void> setWifiEnabled(bool enable);
  Future<Stream<List<String>>> streamWifiEvents(); // client
  Future<Stream<List<String>>> streamWirelessDeviceStream(); // device
  Future<NetworkManagerDevice> getWiredDevice();
  Future<NetworkManagerDevice> getWifiDevice();

  Future<({AccessPoints? active, List<AccessPoints> available})>
      availableAccessPoints(List<SavedWirelessNetwork>? allSavedNetworks);

  // Future<({AccessPoints? active, List<AccessPoints> available})>
  //     scanAvailableAccessPoints(List<SavedWirelessNetwork>? savedNetworks);

  Future<List<SavedNetworks>> savedNetworks(
      List<AccessPoints> availableAccessPoints);

  Future<List<SavedWirelessNetwork>> getSavedNetworks();

  Future<void> connectToNetwork(
      NetworkManagerAccessPoint accessPoint, String password);
  Future<void> connectToSavedNetwork(NetworkManagerAccessPoint accessPoint);
  Future<void> connectToHiddenNetwork(String ssid, String password);
  Future<void> forgetNetwork(String ssid);
  Future<NetworkManagerDeviceState?> getNetworkState();
  Future<void> close();
}
