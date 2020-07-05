extends RigidBody

var direction = 0
var id = 1
export var side = "left"
onready var max_forward_speed = get_parent().max_forward_speed
onready var max_reverse_speed = get_parent().max_reverse_speed
onready var GroundDetection = get_node("GroundDetection")

func _input(_event):
	direction = Input.get_action_strength("p%s_%s_forward" % [id, side]) - Input.get_action_strength("p%s_%s_backward" % [id, side])

func _physics_process(delta):
	if GroundDetection.is_colliding():
		if linear_velocity.dot(transform.basis.z) < max_forward_speed && direction > 0 || \
				linear_velocity.dot(transform.basis.z) > max_reverse_speed && direction < 0:
			add_central_force(delta * direction * 1500 * transform.basis.z)
