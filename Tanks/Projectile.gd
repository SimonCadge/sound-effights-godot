extends KinematicBody

# Declare member variables here. Examples:
# var a = 2
# var b = "text"
var velocity = Vector3()
var bounces = 3

# Called when the node enters the scene tree for the first time.
#func _ready():
#	linear_velocity = 10 * transform.basis.z

# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass

func _physics_process(delta):
	if velocity.y > -10:
		velocity.y -= delta * 5
	var collision = move_and_collide(delta * velocity, false)
	if collision:
		# Emit signal to destroy tank when hit by bullet.
		if bounces > 0:
			velocity = velocity.bounce(collision.normal)
			bounces -= 1
		else:
			queue_free()
	look_at(translation - velocity, transform.basis.y)

#func _on_Projectile_body_entered(body):
#	if body.name == "Floor":
#		print("Floor")
#		visible = false
#		queue_free()
#	else:
#		print(body.name)
#		look_at(translation - linear_velocity, Vector3(0,1,0))
