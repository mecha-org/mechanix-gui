import 'package:flutter/material.dart';
import 'dart:async';

import 'package:mechanix_notes/src/features/editor/audio_recorder.dart';

class LinuxAudioRecordingDialog extends StatefulWidget {
  const LinuxAudioRecordingDialog({super.key});

  @override
  State<LinuxAudioRecordingDialog> createState() => _LinuxAudioRecordingDialogState();
}

class _LinuxAudioRecordingDialogState extends State<LinuxAudioRecordingDialog>
    with TickerProviderStateMixin {
  bool _isRecording = false;
  bool _isInitializing = false;
  int _recordingDuration = 0;
  Timer? _timer;
  late AnimationController _animationController;
  String? _recordingPath;

  @override
  void initState() {
    super.initState();
    _animationController = AnimationController(
      duration: const Duration(milliseconds: 1500),
      vsync: this,
    );
    _checkRecordingAvailability();
  }

  Future<void> _checkRecordingAvailability() async {
    setState(() => _isInitializing = true);
    
    final isAvailable = true;
    
    setState(() => _isInitializing = false);
    
    // if (!isAvailable && mounted) {
    //   _showErrorDialog('Audio recording requires alsa-utils to be installed.\n'
    //       'Please run: sudo apt install alsa-utils');
    // }
  }

  void _showErrorDialog(String message) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Recording Not Available'),
        content: Text(message),
        actions: [
          TextButton(
            onPressed: () {
              Navigator.of(context).pop(); // Close error dialog
              Navigator.of(context).pop(); // Close recording dialog
            },
            child: const Text('OK'),
          ),
        ],
      ),
    );
  }

  void _startTimer() {
    _timer = Timer.periodic(const Duration(seconds: 1), (timer) {
      setState(() => _recordingDuration++);
    });
  }

  void _stopTimer() {
    _timer?.cancel();
    _timer = null;
  }

  Future<void> _startRecording() async {
    setState(() => _isInitializing = true);
    
    _recordingPath = await LinuxAudioRecorder.startRecording();
    
    if (_recordingPath != null) {
      setState(() {
        _isRecording = true;
        _isInitializing = false;
        _recordingDuration = 0;
      });
      _startTimer();
      _animationController.repeat();
    } else {
      setState(() => _isInitializing = false);
      _showErrorDialog('Failed to start recording. Please check your microphone permissions.');
    }
  }

  Future<void> _stopRecording() async {
    setState(() => _isInitializing = true);
    _stopTimer();
    _animationController.stop();
    
    final audioPath = await LinuxAudioRecorder.stopRecording();
    
    if (mounted) {
      Navigator.of(context).pop(audioPath);
    }
  }

  String _formatDuration(int seconds) {
    int minutes = seconds ~/ 60;
    int remainingSeconds = seconds % 60;
    return '${minutes.toString().padLeft(2, '0')}:${remainingSeconds.toString().padLeft(2, '0')}';
  }

  @override
  Widget build(BuildContext context) {
    return Dialog(
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
      child: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Text(
              _isInitializing 
                  ? 'Initializing...'
                  : _isRecording 
                      ? 'Recording...' 
                      : 'Ready to Record',
              style: Theme.of(context).textTheme.headlineSmall,
            ),
            const SizedBox(height: 24),
            if (_isInitializing)
              const CircularProgressIndicator()
            else
              AnimatedBuilder(
                animation: _animationController,
                builder: (context, child) {
                  return Container(
                    width: 80,
                    height: 80,
                    decoration: BoxDecoration(
                      shape: BoxShape.circle,
                      color: _isRecording
                          ? Colors.red.withOpacity(0.3 + (_animationController.value * 0.4))
                          : Colors.grey[300],
                    ),
                    child: Icon(
                      Icons.mic,
                      size: 40,
                      color: _isRecording ? Colors.red : Colors.grey[600],
                    ),
                  );
                },
              ),
            const SizedBox(height: 16),
            if (_isRecording)
              Text(
                _formatDuration(_recordingDuration),
                style: Theme.of(context).textTheme.headlineMedium?.copyWith(
                  color: Colors.red,
                  fontWeight: FontWeight.bold,
                ),
              ),
            const SizedBox(height: 24),
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceEvenly,
              children: [
                ElevatedButton(
                  onPressed: _isInitializing ? null : () => Navigator.of(context).pop(),
                  style: ElevatedButton.styleFrom(
                    backgroundColor: Colors.grey,
                    foregroundColor: Colors.white,
                  ),
                  child: const Text('Cancel'),
                ),
                ElevatedButton(
                  onPressed: _isInitializing 
                      ? null 
                      : _isRecording 
                          ? _stopRecording 
                          : _startRecording,
                  style: ElevatedButton.styleFrom(
                    backgroundColor: _isRecording ? Colors.red : Colors.blue,
                    foregroundColor: Colors.white,
                  ),
                  child: Text(_isRecording ? 'Stop' : 'Start'),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }

  @override
  void dispose() {
    _stopTimer();
    _animationController.dispose();
    LinuxAudioRecorder.dispose();
    super.dispose();
  }
}
