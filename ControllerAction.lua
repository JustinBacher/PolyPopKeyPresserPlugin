Instance.properties = properties({
	{ name="Action", type="Enum", items={"None", "A" "B" "D Pad Down" "D Pad Left" "D Pad Right" "D Pad Up" "Left Shoulder" "Left Thumbstick Button" "Left Thumbstick Down" "Left Thumbstick Left" "Left Thumbstick Right" "Left Thumbstick Up" "Left Trigger" "Menu" "Right Shoulder" "Right Thumbstick Button" "Right Thumbstick Down" "Right Thumbstick Left" "Right Thumbstick Right" "Right Thumbstick Up" "Right Trigger" "View" "X" "Y"} }
})

function Instance:onInit()

end

function Instance:getAction()
	return "GamePad" .. self.properties.Action:gsub("%s+", "")
end