// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'recently_played.dart';

// **************************************************************************
// TypeAdapterGenerator
// **************************************************************************

class RecentlyPlayedAdapter extends TypeAdapter<RecentlyPlayed> {
  @override
  final int typeId = 1;

  @override
  RecentlyPlayed read(BinaryReader reader) {
    final numOfFields = reader.readByte();
    final fields = <int, dynamic>{
      for (int i = 0; i < numOfFields; i++) reader.readByte(): reader.read(),
    };
    return RecentlyPlayed(
      song: fields[0] as SongInfo,
      lastPlayedAt: fields[1] as DateTime,
    );
  }

  @override
  void write(BinaryWriter writer, RecentlyPlayed obj) {
    writer
      ..writeByte(2)
      ..writeByte(0)
      ..write(obj.song)
      ..writeByte(1)
      ..write(obj.lastPlayedAt);
  }

  @override
  int get hashCode => typeId.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is RecentlyPlayedAdapter &&
          runtimeType == other.runtimeType &&
          typeId == other.typeId;
}
