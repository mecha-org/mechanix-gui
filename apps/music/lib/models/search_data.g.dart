// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'search_data.dart';

// **************************************************************************
// TypeAdapterGenerator
// **************************************************************************

class SearchDataAdapter extends TypeAdapter<SearchData> {
  @override
  final int typeId = 3;

  @override
  SearchData read(BinaryReader reader) {
    final numOfFields = reader.readByte();
    final fields = <int, dynamic>{
      for (int i = 0; i < numOfFields; i++) reader.readByte(): reader.read(),
    };
    return SearchData(
      id: fields[0] as String,
      isPlaylist: fields[1] as bool,
      songId: fields[2] as String?,
      playlistId: fields[3] as String?,
      createdAt: fields[4] as DateTime,
    );
  }

  @override
  void write(BinaryWriter writer, SearchData obj) {
    writer
      ..writeByte(5)
      ..writeByte(0)
      ..write(obj.id)
      ..writeByte(1)
      ..write(obj.isPlaylist)
      ..writeByte(2)
      ..write(obj.songId)
      ..writeByte(3)
      ..write(obj.playlistId)
      ..writeByte(4)
      ..write(obj.createdAt);
  }

  @override
  int get hashCode => typeId.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SearchDataAdapter &&
          runtimeType == other.runtimeType &&
          typeId == other.typeId;
}
