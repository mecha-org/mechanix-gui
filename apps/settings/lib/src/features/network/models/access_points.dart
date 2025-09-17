import 'package:nm/nm.dart';

class AccessPoints {
  bool isActive;
  bool isSaved;
  bool isSelected = false; // used for selecting the network
  NetworkManagerAccessPoint nmAccessPoint; // used for connecting to the network
  NetworkManagerIP4Config? ip4Config;
  NetworkManagerIP6Config? ip6Config;

  AccessPoints(
      {required this.isActive,
      required this.isSaved,
      required this.nmAccessPoint,
      required this.ip4Config,
      required this.ip6Config});
}
