extends Node2D

@onready var bridge = $HearthBridge
var creature_scenes = [
	preload("res://scenes/creatures/black_cat.tscn"),
	preload("res://scenes/creatures/ginger_cat.tscn"),
	preload("res://scenes/creatures/white_cat.tscn"),
]

func _ready() -> void:
	spawn_creature()

func _unhandled_input(event):
	if event is InputEventKey:
		if event.keycode == KEY_C and event.pressed:
			spawn_creature()
	if event is InputEventMouseButton:
		if event.button_index == MOUSE_BUTTON_LEFT and event.pressed:
			bridge.set_creature_target(get_global_mouse_position())

func spawn_creature():
	var creature_id = bridge.spawn_creature()
	if creature_id > 0:
		var creature = creature_scenes.pick_random().instantiate()
		creature.id = creature_id
		creature.bridge = bridge
		add_child(creature)
