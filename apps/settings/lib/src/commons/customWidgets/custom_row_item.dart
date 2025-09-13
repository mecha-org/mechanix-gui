import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/styles/custom_styles.dart';

class CustomRowItem extends StatelessWidget {
  final Widget child;
  final String? title;
  final TextStyle? titleStyle;
  final VoidCallback? onTap;
  const CustomRowItem(
      {super.key,
      required this.child,
      this.title,
      this.titleStyle = baseHeaderStyle,
      this.onTap});

  @override
  Widget build(BuildContext context) {
    return Container(
        height: 48,
        decoration: rowBoxDecoration,
        padding: EdgeInsets.symmetric(horizontal: 0),
        margin: EdgeInsets.symmetric(vertical: 5),
        child: ListTile(
          onTap: onTap,
          contentPadding: const EdgeInsets.only(left: 8, right: 5),
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(8),
          ),
          title: (title != null && title!.isNotEmpty)
              ? Text(
                  title!,
                  style: titleStyle,
                )
              : Text(''),
          trailing: child,
        ));
  }
}
