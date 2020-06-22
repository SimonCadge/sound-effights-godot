extends Control

var recorder
var last_time
var audio_analyser
var analysis

const MAX_TIMESTEP = 0.2 * 1000

func _ready():
	var recorder_index = AudioServer.get_bus_index("Record")
	recorder = AudioServer.get_bus_effect(recorder_index, 0)
	audio_analyser = load("res://Godot Native/AudioAnalyser.gdns").new() # Load an instance of the rust Audio Analyser code.

func _process(_delta):
	if recorder.is_recording_active():
		var current_time = OS.get_ticks_msec()
		var elapsed_time = current_time - last_time
		if elapsed_time >= MAX_TIMESTEP:
			var recording = recorder.get_recording()
			var analysis_result = audio_analyser.analyse_end_of_audio(recording, elapsed_time / 1000.0)
			if analysis_result.has("Ok"):
				# print(analysis_result.Ok.frequency)
				print(analysis_result.Ok.volume)
				# print(analysis_result.Ok.duration)
			else:
				print(analysis_result.Err)
			last_time = current_time

func _on_RecordButton_pressed():
	if recorder.is_recording_active():
		var recording = recorder.get_recording()
		var analysis_result = audio_analyser.trim_and_analyse_audio(recording)
		if analysis_result.has("Err"):
			print(analysis_result.Err)
		else:
			print(analysis_result.Ok)
			analysis = analysis_result.Ok
		$PlayButton.disabled = false
		recorder.set_recording_active(false)
		$RecordButton.text = "Record"
	else:
		$PlayButton.disabled = true
		recorder.set_recording_active(true)
		last_time = OS.get_ticks_msec()
		$RecordButton.text = "Stop"

func _on_PlayButton_pressed():
	$AudioStreamPlayer.stream = analysis.sample
	$AudioStreamPlayer.play()
