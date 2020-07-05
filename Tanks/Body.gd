extends RigidBody

# Declare member variables here. Examples:
# var a = 2
onready var DebugLabel = get_parent().get_parent().get_node("DebugLabel")
onready var ProjectileScene = load("res://Tanks/Projectile.tscn")
onready var id = get_parent().id
onready var debug = get_parent().debug

func _input(event):
	if event.is_action("p%s_shoot" % id) && event.is_pressed():
		apply_central_impulse(-50 * transform.basis.z)
		var bullet = ProjectileScene.instance()
		bullet.transform = global_transform
		bullet.translation = bullet.translation + transform.basis.z * 2.2
		bullet.translation = bullet.translation + transform.basis.y * 1
		bullet.velocity = 20 * bullet.transform.basis.z + bullet.transform.basis.y
		get_parent().get_parent().add_child(bullet)

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta):
	if debug:
		DebugLabel.text = "Linear Velocity: " + str(linear_velocity) + "\n" \
						+ "Angular Velocity: " + str(angular_velocity) + "\n" \
						+ "Translation: " + str(translation) + "\n" \
						+ "Rotation Degrees: " + str(rotation_degrees) + "\n" \
						+ "Basis: " + str(transform.basis) + "\n" \
						+ "x Velocity: " + str(linear_velocity.dot(transform.basis.x)) + "\n" \
						+ "y Velocity: " + str(linear_velocity.dot(transform.basis.y)) + "\n" \
						+ "z Velocity: " + str(linear_velocity.dot(transform.basis.z)) + "\n"

func _integrate_forces(state):
	state.linear_velocity = state.linear_velocity - state.linear_velocity.project(transform.basis.x)
