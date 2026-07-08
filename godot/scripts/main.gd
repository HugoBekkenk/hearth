extends Node2D

@onready var bridge = $HearthBridge
@onready var camera = $Camera2D
@onready var terrain_sprite = $TerrainSprite
var creature_scenes = [
	preload("res://scenes/creatures/black_cat.tscn"),
	preload("res://scenes/creatures/ginger_cat.tscn"),
	preload("res://scenes/creatures/white_cat.tscn"),
]
var world_width
var world_height
var tile_size

var camera_speed = 5

func _ready() -> void:
	spawn_creature()
	world_width = bridge.get_world_width()
	world_height = bridge.get_world_height()
	tile_size = bridge.get_tile_size()
	
	var terrain_ids = bridge.get_terrain_ids()
	var image = Image.create(world_width, world_height, false, Image.FORMAT_RGB8)
	for y in range(world_height):
		for x in range(world_width):
			var id = terrain_ids[y * world_width + x]
			image.set_pixel(x, y, get_terrain_color(id))
	var texture = ImageTexture.create_from_image(image)
	terrain_sprite.texture = texture
	terrain_sprite.centered = false
	terrain_sprite.scale = Vector2(tile_size, tile_size)
	terrain_sprite.texture_filter = CanvasItem.TEXTURE_FILTER_NEAREST

func _process(_delta):
	var direction = Vector2.ZERO
	if Input.is_key_pressed(KEY_W): direction += Vector2.UP
	if Input.is_key_pressed(KEY_S): direction += Vector2.DOWN
	if Input.is_key_pressed(KEY_A): direction += Vector2.LEFT
	if Input.is_key_pressed(KEY_D): direction += Vector2.RIGHT
	camera.offset += direction * camera_speed / camera.zoom.x

func _unhandled_input(event):
	if event is InputEventKey:
		if event.keycode == KEY_C and event.pressed:
			spawn_creature()
		if event.keycode == KEY_X and event.pressed:
			bridge.select_all_creature()
		if event.keycode == KEY_Z and event.pressed:
			bridge.deselect_all_creature()
		
	if event is InputEventMouseButton:
		if event.button_index == MOUSE_BUTTON_LEFT and event.pressed:
			bridge.set_creature_target(get_global_mouse_position())
		
		if event.button_index == MOUSE_BUTTON_WHEEL_DOWN:
			var new_zoom = camera.zoom - Vector2(0.1, 0.1)
			camera.zoom = new_zoom.clamp(Vector2(0.1, 0.1), Vector2(5.0, 5.0))
		
		if event.button_index == MOUSE_BUTTON_WHEEL_UP:
			var new_zoom = camera.zoom + Vector2(0.1, 0.1)
			camera.zoom = new_zoom.clamp(Vector2(0.1, 0.1), Vector2(5.0, 5.0))
		

func spawn_creature():
	var creature_id = bridge.spawn_creature()
	if creature_id >= 0:
		var creature = creature_scenes.pick_random().instantiate()
		creature.id = creature_id
		creature.bridge = bridge
		creature.position = bridge.get_creature_position(creature_id)
		add_child(creature)

func get_terrain_color(terrain_id):
	if terrain_id == 0:
		return Color.AQUA
	elif terrain_id == 1:
		return Color.BEIGE
	elif terrain_id == 2:
		return Color.PALE_GREEN
	elif terrain_id == 3:
		return Color.DARK_GREEN
	elif terrain_id == 4:
		return Color.GRAY
	elif terrain_id == 5:
		return Color.WHITE
	else:
		return Color.MAGENTA
	
