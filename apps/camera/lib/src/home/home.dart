import 'package:camera/camera.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_camera/src/bloc/camera_bloc.dart';
import 'package:mechanix_camera/src/bloc/camera_state.dart';
import 'package:mechanix_camera/src/home/bottom_actions.dart';
import 'package:mechanix_camera/src/home/bottom_settings.dart';
import 'package:mechanix_camera/src/home/camera_view.dart';
import 'package:mechanix_camera/src/home/capture_button.dart';
import 'package:mechanix_camera/src/home/circle_overlay_painter.dart';
import 'package:mechanix_camera/src/home/circular_frame_painter.dart';
import 'package:mechanix_camera/src/home/widgets/center_dot_painter.dart';
import 'package:mechanix_camera/src/home/widgets/video_bottom_actions.dart';
import 'package:mechanix_camera/src/home/widgets/video_timer.dart';
import 'package:mechanix_camera/src/home/zoom_strips.dart';
import 'package:widgets/mechanix.dart';

class CameraScreen extends StatefulWidget {
  const CameraScreen({super.key});

  @override
  State<CameraScreen> createState() => _CameraScreenState();
}

class _CameraScreenState extends State<CameraScreen>
    with WidgetsBindingObserver {
  CameraController? _cameraController;
  List<CameraDescription>? _cameras;
  bool _isCameraInitialized = false;

  final ValueNotifier<double> _zoomLevel = ValueNotifier<double>(1.0);
  final ValueNotifier<bool> _isRecording = ValueNotifier<bool>(false);
  final ValueNotifier<double> _captureButtonOffset = ValueNotifier<double>(0.0);

  double _minZoomLevel = 1.0;
  double _maxZoomLevel = 1.0;
  int _currentCameraIndex = 0;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addObserver(this);
    _requestPermissionsAndInitialize();
  }

  Future<void> _requestPermissionsAndInitialize() async {
    await _initializeCamera();
  }

  @override
  void dispose() {
    WidgetsBinding.instance.removeObserver(this);
    _disposeCamera();
    _zoomLevel.dispose();
    _isRecording.dispose();
    _captureButtonOffset.dispose();
    super.dispose();
  }

  @override
  void didChangeAppLifecycleState(AppLifecycleState state) {
    final CameraController? cameraController = _cameraController;

    if (cameraController == null || !cameraController.value.isInitialized) {
      return;
    }

    if (state == AppLifecycleState.inactive) {
      _disposeCamera();
    } else if (state == AppLifecycleState.resumed) {
      _initializeCamera();
    }
  }

  @override
  void reassemble() {
    super.reassemble();
    // Handle hot reload - dispose and reinitialize camera
    _disposeCamera();
    _requestPermissionsAndInitialize();
  }

  Future<void> _disposeCamera() async {
    if (_cameraController != null) {
      // Stop recording if active
      if (_cameraController!.value.isRecordingVideo) {
        try {
          await _cameraController!.stopVideoRecording();
          _isRecording.value = false;
        } catch (e) {
          debugPrint('Error stopping recording during disposal: $e');
        }
      }

      // Dispose the controller
      await _cameraController!.dispose();
      _cameraController = null;

      if (mounted) {
        setState(() {
          _isCameraInitialized = false;
        });
      }
    }
  }

  Future<void> _initializeCamera() async {
    try {
      _cameras = await availableCameras();

      if (_cameras == null || _cameras!.isEmpty) {
        debugPrint('No cameras available');
        return;
      }

      await _setupCamera(_cameras![_currentCameraIndex]);
    } catch (e) {
      debugPrint('Error initializing camera: $e');
    }
  }

  Future<void> _setupCamera(CameraDescription cameraDescription) async {
    // Ensure previous controller is disposed
    await _disposeCamera();

    final CameraController cameraController = CameraController(
      cameraDescription,
      ResolutionPreset.high,
      enableAudio: true,
      imageFormatGroup: ImageFormatGroup.jpeg,
    );

    _cameraController = cameraController;

    try {
      await cameraController.initialize();
      _maxZoomLevel = await cameraController.getMaxZoomLevel();
      _minZoomLevel = await cameraController.getMinZoomLevel();

      await cameraController.setZoomLevel(_zoomLevel.value);

      if (mounted) {
        setState(() {
          _isCameraInitialized = true;
        });
      }
    } catch (e) {
      debugPrint('Error setting up camera: $e');
      if (mounted) {
        setState(() {
          _isCameraInitialized = false;
        });
      }
    }
  }

  Future<void> _updateZoom(double zoom) async {
    if (_cameraController == null || !_cameraController!.value.isInitialized) {
      return;
    }

    final clampedZoom = zoom.clamp(_minZoomLevel, _maxZoomLevel);
    _zoomLevel.value = clampedZoom;
    try {
      await _cameraController!.setZoomLevel(clampedZoom);
    } catch (e) {
      debugPrint('Error setting zoom level: $e');
    }
  }

  Future<void> _takePicture() async {
    if (_cameraController == null || !_cameraController!.value.isInitialized) {
      return;
    }
    try {
      final XFile picture = await _cameraController!.takePicture();
      debugPrint('Picture saved to: ${picture.path}');
    } catch (e) {
      debugPrint('Error taking picture: $e');
    }
  }

  Future<void> _startVideoRecording() async {
    if (_cameraController == null || !_cameraController!.value.isInitialized) {
      return;
    }

    try {
      await _cameraController!.startVideoRecording();
      _isRecording.value = true;
    } catch (e) {
      debugPrint('Error starting video recording: $e');
    }
  }

  Future<void> _stopVideoRecording() async {
    if (_cameraController == null || !_cameraController!.value.isInitialized) {
      return;
    }

    if (!_cameraController!.value.isRecordingVideo) {
      return;
    }

    try {
      final XFile video = await _cameraController!.stopVideoRecording();

      _isRecording.value = false;
      debugPrint('Video saved to: ${video.path}');
    } catch (e) {
      debugPrint('Error stopping video recording: $e');
      _isRecording.value = false;
    }
  }

  @override
  Widget build(BuildContext context) {
    final screenWidth = MediaQuery.of(context).size.width;
    final circleSize = screenWidth * 0.89;
    return Scaffold(
      body: BlocSelector<CameraBloc, CameraState, bool>(
        selector: (state) => state.isSettingsOpen,
        builder:
            (context, isSettingsOpen) => Stack(
              children: [
                // Camera Preview
                Positioned.fill(
                  child: CameraView(
                    isCameraInitialized: _isCameraInitialized,
                    cameraController: _cameraController,
                  ),
                ),
                // Dark overlay outside circle (when zooming)
                if (!isSettingsOpen) ...[
                  // Bottom Actions
                  ValueListenableBuilder(
                    valueListenable: _isRecording,
                    builder: (context, value, child) {
                      if (value) {
                        return VideoBottomActions(
                          cameraController: _cameraController,
                        );
                      }
                      return const BottomActions();
                    },
                  ),
                  ValueListenableBuilder<double>(
                    valueListenable: _captureButtonOffset,
                    builder: (context, offset, child) {
                      if (offset == 0.0) return const SizedBox.shrink();
                      return Positioned.fill(
                        child: IgnorePointer(
                          child: CustomPaint(
                            painter: CircleOverlayPainter(
                              colorScheme: context.colorScheme,
                              circleSize: circleSize,
                              screenSize: MediaQuery.of(context).size,
                            ),
                          ),
                        ),
                      );
                    },
                  ),

                  ValueListenableBuilder(
                    valueListenable: _isRecording,
                    builder:
                        (context, value, child) =>
                            _isRecording.value
                                ? const Positioned(
                                  top: 15,
                                  left: 0,
                                  right: 0,
                                  child: Center(child: VideoTimer()),
                                )
                                : const SizedBox.shrink(),
                  ),
                  // Circular UI Overlay
                  Center(
                    child: CustomPaint(
                      size: Size(circleSize, circleSize),
                      painter: CircularFramePainter(
                        colorScheme: context.colorScheme,
                      ),
                    ),
                  ),

                  // Capture Button with Drag Control
                  CaptureButton(
                    zoomLevel: _zoomLevel,
                    isRecording: _isRecording,
                    captureButtonOffset: _captureButtonOffset,
                    onCapture: _takePicture,
                    onStartRecording: _startVideoRecording,
                    onStopRecording: _stopVideoRecording,
                    onZoomChange: _updateZoom,
                    minZoom: _minZoomLevel,
                    maxZoom: _maxZoomLevel,
                  ),

                  // Zoom Strips Overlay (separate layer)
                  ValueListenableBuilder<double>(
                    valueListenable: _captureButtonOffset,
                    builder: (context, offset, child) {
                      if (offset == 0.0) return const SizedBox.shrink();

                      return Center(
                        child: ValueListenableBuilder<double>(
                          valueListenable: _zoomLevel,
                          builder: (context, zoom, child) {
                            return CustomPaint(
                              size: Size(circleSize, circleSize),
                              painter: ZoomStrips(
                                zoomLevel: zoom >= 2.0 ? 2.0 : zoom,
                              ),
                            );
                          },
                        ),
                      );
                    },
                  ),
                ] else
                  // Bottom Action Buttons
                  const BottomSettings(),

                // Center Circle Dot
                Center(
                  child: CustomPaint(
                    painter: CenterDotPainter(colorScheme: context.colorScheme),
                  ),
                ),
              ],
            ),
      ),
    );
  }
}
