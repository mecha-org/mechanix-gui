import 'dart:convert';

import 'package:equatable/equatable.dart';
import 'package:nm/nm.dart';

class AccessPoints extends Equatable {
  final bool isActive;
  final bool isSaved;
  final bool isSelected;
  final bool isSecure;
  final NetworkManagerAccessPoint nmAccessPoint;
  final NetworkManagerIP4Config? ip4Config;
  final NetworkManagerIP6Config? ip6Config;

  const AccessPoints({
    required this.isActive,
    required this.isSaved,
    this.isSelected = false,
    required this.isSecure,
    required this.nmAccessPoint,
    this.ip4Config,
    this.ip6Config,
  });

  @override
  List<Object?> get props => [
        isActive,
        isSaved,
        isSelected,
        isSecure,
        utf8.decode(nmAccessPoint.ssid),
        nmAccessPoint.strength,
        nmAccessPoint.flags,
        nmAccessPoint.frequency,
        nmAccessPoint.rsnFlags,
        nmAccessPoint.wpaFlags,
        nmAccessPoint.mode,
        nmAccessPoint.maxBitrate,
        ip4Config?.addressData.join(','),
        ip4Config?.nameserverData.join(','),
        ip4Config?.gateway,
        ip6Config?.addressData.join(','),
        ip6Config?.nameserverData.join(','),
        ip6Config?.gateway,
      ];
}
