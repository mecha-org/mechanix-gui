import 'package:flutter/material.dart';

class ContainerWidget extends StatelessWidget {
  final Widget child;
  const ContainerWidget({super.key, required this.child});

  @override
  Widget build(BuildContext context) {
    return Container(margin: EdgeInsets.all(4), child: child);
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
      height: 60,
      alignment: Alignment.center,
      padding: EdgeInsets.fromLTRB(16, 0, 16, 0),
      decoration: BoxDecoration(
        border: Border(
          top:
              showTopBorder ?? false
                  ? BorderSide(
                    width: 1.0,
                    color: Color.fromRGBO(32, 32, 32, 0.5),
                  )
                  : BorderSide.none,
          bottom:
              showBottomBorder ?? true
                  ? BorderSide(
                    width: 1.0,
                    color: Color.fromRGBO(32, 32, 32, 0.5),
                  )
                  : BorderSide.none,
        ),
      ),
      child: child,
    );
  }
}
