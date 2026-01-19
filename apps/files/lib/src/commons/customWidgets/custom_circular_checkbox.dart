import 'package:flutter/material.dart';
import 'package:widgets/mechanix.dart';

class CustomCircleCheckbox extends StatelessWidget {
  final bool isChecked;
  final VoidCallback onTap;

  const CustomCircleCheckbox({
    super.key,
    required this.isChecked,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(20),
      child: Container(
        width: 20.75,
        height: 20.75,
        decoration: BoxDecoration(
          shape: BoxShape.circle,
          color: isChecked ? context.colorScheme.primary : Colors.transparent,
          border: Border.all(
            color: isChecked
                ? context.colorScheme.primary
                : context.colorScheme.outline,
            width: 2.5,
          ),
        ),
        child: isChecked
            ? Center(
                child: Icon(
                  Icons.check,
                  size: 16,
                  color: context.colorScheme.surface,
                ),
              )
            : null,
      ),
    );
  }
}
