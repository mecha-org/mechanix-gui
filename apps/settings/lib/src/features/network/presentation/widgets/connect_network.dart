import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/features/network/presentation/connect_secure_network.dart';
import 'package:nm/nm.dart';

class ConnectNetwork extends StatefulWidget {
  const ConnectNetwork({
    super.key,
    this.accessPoint,
  });

  final NetworkManagerAccessPoint? accessPoint;

  @override
  State<ConnectNetwork> createState() => _ConnectNetworkState();
}

class _ConnectNetworkState extends State<ConnectNetwork>
    with TickerProviderStateMixin {
  bool isUpcomingTrackWindow = false;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: AnimatedSwitcher(
        duration: const Duration(milliseconds: 500),
        reverseDuration: const Duration(milliseconds: 400),
        switchInCurve: Curves.easeOutCubic,
        switchOutCurve: Curves.easeInCubic,
        transitionBuilder: _slideFadeTransition,
        child: ConnectSecureNetwork(accessPoint: widget.accessPoint),
      ),
    );
  }

  ///  Slide from bottom + fade animation
  Widget _slideFadeTransition(Widget child, Animation<double> animation) {
    final isUpcoming = child.key == const ValueKey('upcoming');

    final offsetAnimation = Tween<Offset>(
      begin: isUpcoming ? const Offset(0, 0.25) : const Offset(0, -0.15),
      end: Offset.zero,
    ).animate(animation);

    return SlideTransition(
      position: offsetAnimation,
      child: FadeTransition(opacity: animation, child: child),
    );
  }
}
