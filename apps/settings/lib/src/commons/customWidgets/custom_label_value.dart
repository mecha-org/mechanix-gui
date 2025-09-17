import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_row_item.dart';
import 'package:mechanix_settings/src/commons/styles/custom_styles.dart';

class CustomLabelValue extends StatelessWidget {
  final Widget? child;
  final String title;
  final String? value;
  final TextStyle? valueStyle; // Changed from String? to TextStyle?
  final VoidCallback? onTap;

  const CustomLabelValue(
      {super.key,
      this.child,
      required this.title,
      this.value,
      this.valueStyle,
      this.onTap});

  @override
  Widget build(BuildContext context) {
    return CustomRowItem(
      onTap: onTap,
      title: title,
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 5),
        child: value != null
            ? Text(
                value!,
                style: valueStyle ??
                    secondaryHeaderStyle, // Fallback to default style
              )
            : child,
      ),
    );
  }
}
