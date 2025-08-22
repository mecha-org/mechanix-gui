enum DisplayScreenOffTime {
  tenSeconds,
  thirtySeconds,
  sixtySeconds,
  fiveMinutes,
  never
}

Map<DisplayScreenOffTime, String> displayScreenOffTimeToString = {
  DisplayScreenOffTime.tenSeconds: '10s',
  DisplayScreenOffTime.thirtySeconds: '30s',
  DisplayScreenOffTime.sixtySeconds: '60s',
  DisplayScreenOffTime.fiveMinutes: '5m',
  DisplayScreenOffTime.never: 'Never'
};