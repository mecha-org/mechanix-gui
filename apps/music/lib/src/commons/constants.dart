class TableName {
  static const String songsInfoTable = "musicSongsTablev4";
  static const String recentlyPlayedTable = "recentlyPlayedTablev2";
  static const String playlistTable = "playlistTablev2";
  static const String searchTable = "searchTablev1";
}

class Constants {
  static const double recentlyPlayedLimit = 9;
  static const double playlistLimit = 5;
  static const double maxSongsPerPlaylist = 30;
  static const double maxSearchItems = 30;
  static const Duration debounceDuration = Duration(milliseconds: 300);
}

const audioExt = ['.mp3', '.wav', '.flac', '.m4a', '.aac', '.ogg', '.opus'];
