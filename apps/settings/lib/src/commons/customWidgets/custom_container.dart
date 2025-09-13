import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/styles/custom_styles.dart';

class ContainerWidget extends StatelessWidget {
  final Widget child;
  const ContainerWidget({super.key, required this.child});

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: EdgeInsets.symmetric(horizontal: 16, vertical: 0),
      child: child,
    );
  }
}

class FixedHeightRow extends StatelessWidget {
  final Widget child;
  final bool? showTopBorder;
  final bool? showBottomBorder;

  const FixedHeightRow({
    super.key,
    required this.child,
    this.showTopBorder = false,
    this.showBottomBorder = true,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 48,
      alignment: Alignment.center,
      decoration: rowBoxDecoration,
      child: child,
    );
  }
}
