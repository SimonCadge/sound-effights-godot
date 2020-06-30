extends RigidBody

# Declare member variables here. Examples:
# var a = 2
onready var DebugLabel = get_parent().get_node("DebugLabel")
var left = 0
var right = 0
export var max_forward_speed = 10
export var max_reverse_speed = -6
export var id = 0


# Called when the node enters the scene tree for the first time.
#func _ready():
#	pass # Replace with function body.

func _input(event):
	if event.is_action("p%s_shoot" % id) && event.is_pressed():
		apply_central_impulse(-50 * transform.basis.z)
		var projectile = load("res://Tanks/Projectile.tscn")
		var bullet = projectile.instance()
		bullet.transform = transform
		bullet.translation = bullet.translation + transform.basis.z * 2.2
		bullet.translation = bullet.translation + transform.basis.y * 1
		bullet.velocity = 20 * bullet.transform.basis.z + bullet.transform.basis.y
		get_parent().add_child(bullet)

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta):
	left = Input.get_action_strength("p%s_left_forward" % id) - Input.get_action_strength("p%s_left_backward" % id)
	right = Input.get_action_strength("p%s_right_forward" % id) - Input.get_action_strength("p%s_right_backward" % id)
	DebugLabel.text = "Left: " + str(left) + "\n" \
					+ "Right: " + str(right) + "\n" \
					+ "Linear Velocity: " + str(linear_velocity) + "\n" \
					+ "Angular Velocity: " + str(angular_velocity) + "\n" \
					+ "Translation: " + str(translation) + "\n" \
					+ "Rotation Degrees: " + str(rotation_degrees) + "\n" \
					+ "Basis: " + str(transform.basis) + "\n" \
					+ "x Velocity: " + str(linear_velocity.dot(transform.basis.x)) + "\n" \
					+ "y Velocity: " + str(linear_velocity.dot(transform.basis.y)) + "\n" \
					+ "z Velocity: " + str(linear_velocity.dot(transform.basis.z)) + "\n"
	

func _physics_process(delta):
	# Allow acceleration only if accelerating forward and the forward velocity isn't greater than the max forward velocity,
	# or if accelerating backward and the backward velocity isn't greater than the max backward velocity.
	if linear_velocity.dot(transform.basis.z) < max_forward_speed && (left + right) > 0 || \
			linear_velocity.dot(transform.basis.z) > max_reverse_speed && (left + right) < 0:
		var speed = (left + right) * 5000 * transform.basis.z
		add_central_force(delta * speed)

func _integrate_forces(state):
	# Only move forward, don't drift.
	state.linear_velocity = state.linear_velocity - state.linear_velocity.project(transform.basis.x)
	state.angular_velocity = (right - left) * Vector3(0,1,0)
