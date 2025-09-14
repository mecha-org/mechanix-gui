import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_trailing_text.dart';
import 'package:widgets/mechanix.dart';

class UpdateCard extends StatelessWidget {
  const UpdateCard({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: Spacing.only(top: 24, bottom: 40),
      decoration: BoxDecoration(
        borderRadius: CircularRadius.sm,
        color: context.colorScheme.secondary,
      ),
      child: GestureDetector(
        behavior: HitTestBehavior.translucent,
        // onTap: () => Navigator.pushNamed(context, AppRoutes.updates),
        child: Row(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            IconWidget(
              boxWidth: 36,
              boxHeight: 36,
              iconWidth: 29,
              iconHeight: 27,
              iconPath: Images.settings,
            ).padOnly(top: 16, left: 16),
            Column(
              mainAxisAlignment: MainAxisAlignment.start,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Container(
                    margin: Spacing.bottom(10),
                    child: Text(
                      'Software Update',
                      style: context.textTheme.titleMedium,
                    )),
                CustomTrailingText(
                    title: 'Update for Mechanix OS 1.01 available')
              ],
            ).padSymmetric(horizontal: 12, vertical: 22)
          ],
        ),
      ),
    );
  }
}
