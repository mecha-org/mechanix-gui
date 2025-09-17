import 'package:flutter/material.dart';
import 'package:widgets/widgets/select/select_type.dart';

enum BatteryStatus {
  Unknown,
  Charging,
  Discharging,
  Empty,
  FullCharged,
  PendingCharge,
  PendingDischarge,
}

class ModeOption {
  final String mode;
  final String? content;
  final Color color;

  const ModeOption({
    required this.mode,
    this.content,
    required this.color,
  });
}

// power-saver, balanced, performance,
final List<SelectOption<String>> performanceOptions = [
  SelectOption(value: 'power-saver', label: 'Low'),
  SelectOption(value: 'balanced', label: 'Balanced'),
  SelectOption(value: 'performance', label: 'High'),
];

ModeOption getModeDetails(String value) {
  switch (value) {
    case 'power-saver':
      return ModeOption(mode: "Low", color: Color(0xFFF39C23));

    case 'performance':
      return ModeOption(
          mode: "High",
          content:
              "High performance mode dissipates battery quicker to enhance computations in Comet",
          color: Color(0xFF8220DE));

    default:
      return ModeOption(mode: "Balanced", color: Color(0xFF34C759));
  }
}
