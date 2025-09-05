import 'package:nm/nm.dart';

class AccessPoints {
  bool isActive;
  bool isSaved;
  bool isSelected = false; // used for selecting the network
  NetworkManagerAccessPoint nmAccessPoint; // used for connecting to the network
  AccessPoints({
    required this.isActive,
    required this.isSaved,
    required this.nmAccessPoint,
  });
}
