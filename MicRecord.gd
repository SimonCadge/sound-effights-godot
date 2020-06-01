extends Control

var recorder
var time_step = MAX_TIMESTEP
var audio_analyser
var analysis

const MAX_TIMESTEP = 0.2

func _ready():
	var recorder_index = AudioServer.get_bus_index("Record")
	recorder = AudioServer.get_bus_effect(recorder_index, 0)
	audio_analyser = load("res://AudioAnalyser.gdns").new() # Load an instance of the rust Audio Analyser code.
	
func _process(delta):
	if recorder.is_recording_active():
		if time_step - delta <= 0:
			var recording = recorder.get_recording()
			var analysis_result = audio_analyser.cut_and_analyse_audio(recording, MAX_TIMESTEP * 0.9)
			if analysis_result.has("Ok"):
				print(analysis_result.Ok.frequency)
			time_step = MAX_TIMESTEP
		else :
			time_step -= delta

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
		time_step = MAX_TIMESTEP
		$RecordButton.text = "Stop"


func _on_PlayButton_pressed():
	$AudioStreamPlayer.stream = analysis.sample
	$AudioStreamPlayer.play()
