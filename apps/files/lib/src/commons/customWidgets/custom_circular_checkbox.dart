import 'package:flutter/material.dart';
import 'package:mechanix_files/src/commons/styles/file_theme_extenstions.dart';

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
        width: 22,
        height: 22,
        decoration: BoxDecoration(
          shape: BoxShape.circle,
          color: isChecked
              ? Theme.of(context).extension<FilesTheme>()!.primaryColor
              : Colors.transparent,
          border: Border.all(
            color: isChecked
                ? Theme.of(context).extension<FilesTheme>()!.primaryColor
                : Colors.grey.shade600,
            width: 2.5,
          ),
        ),
        child: isChecked
            ? const Center(
                child: Icon(
                  Icons.check,
                  size: 16,
                  color: Colors.black,
                ),
              )
            : null,
      ),
    );
  }
}
