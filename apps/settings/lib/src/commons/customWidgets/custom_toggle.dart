import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/styles/custom_styles.dart';

class CustomToggle extends StatelessWidget {
  final bool value;
  final ValueChanged<bool> onChanged;

  const CustomToggle({
    super.key,
    required this.value,
    required this.onChanged,
  });

  @override
  Widget build(BuildContext context) {
    return MouseRegion(
      cursor: SystemMouseCursors.click,
      child: GestureDetector(
        onTap: () => onChanged(!value),
        child: AnimatedContainer(
          duration: Duration(milliseconds: 300),
          curve: Curves.easeInOut,
          width: 70,
          height: 32,
          decoration: BoxDecoration(
            color: Colors.black,
            borderRadius: BorderRadius.circular(6),
            border: Border.all(
              color: value ? Color(0xFF2D8AFF) : Color(0xFF2B2B2B),
              width: 2,
            ),
          ),
          child: Stack(
            children: [
              AnimatedOpacity(
                opacity: value ? 1.0 : 0.0,
                duration: Duration(milliseconds: 200),
                child: Align(
                  alignment: Alignment.centerLeft,
                  child: Padding(
                    padding: const EdgeInsets.only(left: 5),
                    child: Text('ON', style: baseHeaderStyle),
                  ),
                ),
              ),

              // Toggle handle
              AnimatedAlign(
                duration: Duration(milliseconds: 300),
                curve: Curves.easeInOut,
                alignment: value ? Alignment.centerRight : Alignment.centerLeft,
                child: 
                Padding(
                  padding: const EdgeInsets.only(left: 4,right: 3),
                  child: Container(
                    width: 27,
                    height: 22,
                    decoration: BoxDecoration(
                      color: value ? Color(0xFF2D8AFF) : Color(0xFF2B2B2B),
                      borderRadius: BorderRadius.circular(2),
                    ),
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
