extends Node2D

@onready var bridge = $HearthBridge
@onready var sprite = $AnimatedSprite2D

func _process(_delta: float) -> void:
	sprite.global_position = bridge.get_creature_position()
	sprite.play(bridge.get_animation_name())

func _input(event):
	if event is InputEventMouseButton:
		if event.button_index == MOUSE_BUTTON_LEFT and event.pressed:
			bridge.set_creature_target(get_global_mouse_position())
