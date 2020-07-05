extends Spatial

export var max_forward_speed = 10
export var max_reverse_speed = -6
export var id = 1
export var debug = false

func hit():
	print("Boom")
	queue_free()
