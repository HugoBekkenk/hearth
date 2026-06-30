extends Node2D

@onready var bridge = $HearthBridge
var creature_scenes = [
	preload("res://scenes/creatures/black_cat.tscn"),
	preload("res://scenes/creatures/ginger_cat.tscn"),
	preload("res://scenes/creatures/white_cat.tscn"),
]
var world_width
var world_height
var tile_size

func _ready() -> void:
	spawn_creature()
	world_width = bridge.get_world_width()
	world_height = bridge.get_world_height()
	tile_size = bridge.get_tile_size()

func _draw():
	for x in range(world_width):
		for y in range(world_height):
			draw_rect(Rect2(x * tile_size, y * tile_size, tile_size, tile_size), Color.AQUAMARINE, false)

func _unhandled_input(event):
	if event is InputEventKey:
		if event.keycode == KEY_C and event.pressed:
			spawn_creature()
		if event.keycode == KEY_X and event.pressed:
			bridge.select_all_creature()
	if event is InputEventMouseButton:
		if event.button_index == MOUSE_BUTTON_LEFT and event.pressed:
			bridge.set_creature_target(get_global_mouse_position())

func spawn_creature():
	var creature_id = bridge.spawn_creature()
	if creature_id >= 0:
		var creature = creature_scenes.pick_random().instantiate()
		creature.id = creature_id
		creature.bridge = bridge
		creature.position = bridge.get_creature_position(creature_id)
		add_child(creature)
