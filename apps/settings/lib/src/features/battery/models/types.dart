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
  final String content;

  const ModeOption({
    required this.mode,
    required this.content,
  });
}

final Map<String, ModeOption> batteryModes = {
  "low": ModeOption(
    mode: "Low",
    content:
        "Low performance mode reduces battery dissipation and reduces  computation speed in Comet",
  ),
  "balanced": ModeOption(
    mode: "Balanced",
    content: "Balanced power and performance for everyday use.",
  ),
  "high": ModeOption(
    mode: "High",
    content:
        "High performance mode dissipates battery quicker to enhance computations in Comet",
  ),
};
