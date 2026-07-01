extends Node2D

var id;
var bridge;
@onready var sprite = $AnimatedSprite2D
#@onready var select_area = $SelectArea

func _process(_delta: float) -> void:
	if id == null or bridge == null:
		return
	global_position = bridge.get_creature_position(id)
	sprite.play(bridge.get_animation_name(id))

func _on_select_area_input_event(_viewport: Node, event: InputEvent, _shape_idx: int) -> void:
	if event is InputEventMouseButton:
		if event.button_index == MOUSE_BUTTON_LEFT and event.pressed:
			print("Selected creature: ", id)
			bridge.select_creature(id)
