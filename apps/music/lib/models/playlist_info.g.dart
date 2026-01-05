// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'playlist_info.dart';

// **************************************************************************
// TypeAdapterGenerator
// **************************************************************************

class PlaylistInfoAdapter extends TypeAdapter<PlaylistInfo> {
  @override
  final int typeId = 2;

  @override
  PlaylistInfo read(BinaryReader reader) {
    final numOfFields = reader.readByte();
    final fields = <int, dynamic>{
      for (int i = 0; i < numOfFields; i++) reader.readByte(): reader.read(),
    };
    return PlaylistInfo(
      id: fields[0] as String,
      name: fields[1] as String,
      songIds: (fields[2] as List).cast<String>(),
      createdAt: fields[3] as DateTime,
      updatedAt: fields[4] as DateTime,
      coverImagePath: fields[5] as String?,
      isShuffle: fields[6] as bool,
      isLiked: fields[7] as bool,
    );
  }

  @override
  void write(BinaryWriter writer, PlaylistInfo obj) {
    writer
      ..writeByte(8)
      ..writeByte(0)
      ..write(obj.id)
      ..writeByte(1)
      ..write(obj.name)
      ..writeByte(2)
      ..write(obj.songIds)
      ..writeByte(3)
      ..write(obj.createdAt)
      ..writeByte(4)
      ..write(obj.updatedAt)
      ..writeByte(5)
      ..write(obj.coverImagePath)
      ..writeByte(6)
      ..write(obj.isShuffle)
      ..writeByte(7)
      ..write(obj.isLiked);
  }

  @override
  int get hashCode => typeId.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is PlaylistInfoAdapter &&
          runtimeType == other.runtimeType &&
          typeId == other.typeId;
}
