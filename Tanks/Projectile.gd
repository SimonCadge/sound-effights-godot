extends KinematicBody

var velocity = Vector3()
var bounces = 2
var TankFilename = "res://Tanks/Tank.tscn"

func _physics_process(delta):
	if velocity.y > -10:
		velocity.y -= delta * 5
	var collision = move_and_collide(delta * velocity, false)
	if collision:
		if collision.collider.get_parent().get_filename() == TankFilename:
			print("Tank")
			collision.collider.get_parent().call("hit")
			queue_free()
		elif bounces > 0:
			velocity = velocity.bounce(collision.normal)
			bounces -= 1
		else:
			queue_free()
	look_at(translation - velocity, transform.basis.y)
